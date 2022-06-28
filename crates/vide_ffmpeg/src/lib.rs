pub mod quick_export;

use std::{fs::File, io::Write};

use ac_ffmpeg::{codec::{video::{VideoEncoder, self, VideoFrameMut, PixelFormat}, Encoder}, time::{TimeBase, Timestamp}, format::{muxer::{Muxer, OutputFormat}, io::IO}};
use vide_lib::io::{Import, Export};

pub struct FFmpegImporter {

}

impl Import for FFmpegImporter {
    fn supported_import_extensions() -> Vec<String> {
        todo!()
    }
}

fn open_output(path: &str, elementary_streams: &[ac_ffmpeg::codec::CodecParameters]) -> Result<Muxer<File>, ac_ffmpeg::Error> {
    let output_format = OutputFormat::guess_from_file_name(path)
        .ok_or_else(|| ac_ffmpeg::Error::new(format!("unable to guess output format for file: {}", path)))?;

    let output = File::create(path)
        .map_err(|err| ac_ffmpeg::Error::new(format!("unable to create output file {}: {}", path, err)))?;

    let io = IO::from_seekable_write_stream(output);

    let mut muxer_builder = Muxer::builder();

    for codec_parameters in elementary_streams {
        muxer_builder.add_stream(codec_parameters)?;
    }

    muxer_builder.build(io, output_format)
}

pub struct FFmpegExporter {
    output: String,

    container: String,
    video_coding: String,
    audio_coding: Option<String>,

    encoder: Option<VideoEncoder>,
    muxer: Option<Muxer<File>>,

    current_timestamp: i64,
    ms_per_frame: i64,
    pixel_format: Option<PixelFormat>,
    resolution: (usize, usize),
}

impl FFmpegExporter { // TODO: Support multiple encoders and stuff
    pub fn new(output: impl ToString, container: impl ToString, video_coding: impl ToString, audio_coding: Option<String>) -> Self {
        Self {
            output: output.to_string(),

            container: container.to_string(),
            video_coding: video_coding.to_string(),
            audio_coding: audio_coding,

            encoder: None,
            muxer: None,

            current_timestamp: 0,
            ms_per_frame: 0,
            pixel_format: None,
            resolution: (1920, 1080),
        }
    }
}

impl Export for FFmpegExporter {
    fn begin(&mut self, settings: vide_lib::api::video::VideoSettings) {
        let time_base = TimeBase::new(1, 1_000_000);
        let pixel_format = video::frame::get_pixel_format("rgb24");
        
        let encoder = VideoEncoder::builder("libx264rgb")
            .unwrap()
            .pixel_format(pixel_format)
            .width(settings.resolution.0 as usize)
            .height(settings.resolution.1 as usize)
            .time_base(time_base)
            .build()
            .unwrap();
        
        let codec_parameters = encoder.codec_parameters().into();
        let muxer = open_output(self.output.as_str(), &[codec_parameters]).unwrap();

        self.encoder = Some(encoder);
        self.muxer = Some(muxer);
        self.ms_per_frame = ((1.0 / settings.fps) * 1000000.0) as i64;
        self.pixel_format = Some(pixel_format);
        self.resolution = (settings.resolution.0 as usize, settings.resolution.1 as usize);
    }

    fn push_frame(&mut self, _keyframe: bool, frame: &[u8]) {
        let timestamp = Timestamp::from_micros(self.current_timestamp);
        let encoder = self.encoder.as_mut().unwrap();
        let muxer = self.muxer.as_mut().unwrap();

        {
            // Allocate frame
            let mut new_frame = VideoFrameMut::black(self.pixel_format.unwrap(), self.resolution.0, self.resolution.1);
            // Copy texture (and remove 4th component of each pixel) to frame
            new_frame.planes_mut()[0].data_mut().write_all(&frame.chunks(4).flat_map(|p|[p[0], p[1], p[2]]).collect::<Vec<_>>()[..]).unwrap();
            // Add to encoder queue
            encoder.push(new_frame.with_pts(timestamp).freeze()).unwrap();
        }

        // Await encoder and add to muxer queue
        while let Some(packet) = encoder.take().unwrap() {
            muxer.push(packet.with_stream_index(0)).unwrap();
        }

        self.current_timestamp += self.ms_per_frame;
    }

    fn end(mut self) {
        let encoder = self.encoder.as_mut().unwrap();
        let muxer = self.muxer.as_mut().unwrap();

        encoder.flush().unwrap();
        while let Some(packet) = encoder.take().unwrap() {
            muxer.push(packet.with_stream_index(0)).unwrap();
        }
        muxer.flush().unwrap();
    }
}
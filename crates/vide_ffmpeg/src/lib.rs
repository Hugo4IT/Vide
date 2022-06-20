pub mod quick_export;

use std::fs::File;

use ac_ffmpeg::{codec::{video::{VideoEncoder, self}, Encoder}, time::TimeBase, format::muxer::Muxer};
use vide_lib::io::{Import, Export};

pub struct FFmpegImporter {

}

impl Import for FFmpegImporter {
    fn supported_import_extensions() -> Vec<String> {
        todo!()
    }
}

fn open_output(path: &str, elementary_streams: &[CodecParameters]) -> Result<Muxer<File>, Error> {
    let output_format = OutputFormat::guess_from_file_name(path)
        .ok_or_else(|| Error::new(format!("unable to guess output format for file: {}", path)))?;

    let output = File::create(path)
        .map_err(|err| Error::new(format!("unable to create output file {}: {}", path, err)))?;

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
    muxer: Option<Muxer<File>>
}

impl FFmpegExporter {
    pub fn new(output: impl ToString, container: impl ToString, video_coding: impl ToString, audio_coding: Option<String>) -> Self {
        Self {
            output: output.to_string(),

            container: container.to_string(),
            video_coding: video_coding.to_string(),
            audio_coding: audio_coding,

            encoder: None,
            muxer: None,
        }
    }
}

impl Export for FFmpegExporter {
    fn begin(&mut self, settings: vide_lib::video::VideoSettings) {
        let time_base = TimeBase::new(1, settings.fps);
        
        self.encoder = VideoEncoder::builder("libx264")
            .unwrap()
            .pixel_format(video::frame::get_pixel_format("yuv420p"))
            .width(settings.resolution.0)
            .height(settings.resolution.1)
            .time_base(time_base)
            .build()
            .unwrap();
        
        let codec_parameters = encoder.codec_parameters().into();
        let mut muxer = open_output(self.output.as_str(), &[codec_parameters]).unwrap();
    }

    fn push_frame(&mut self, keyframe: bool, frame: &[u8]) {
        todo!()
    }

    fn end(self) {
        todo!()
    }
}
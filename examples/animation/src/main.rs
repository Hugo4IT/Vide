use std::time::Duration;

use vide::{sequence::{Clip, ClipInfo}, render::{Time, Renderer}, video::{Video, VideoSettings}};

struct MyClip;
impl Clip for MyClip {
    fn info(&self) -> ClipInfo {
        ClipInfo {
            name: Some("My Epic Clip".to_string()),
            duration: Duration::from_secs(2),
        }
    }

    fn render(&self, time: Time, renderer: &mut Renderer) {
        println!("Rendering frame {} of My Epic Clip ({}/300)", time.clip_frame, time.video_frame)
    }
}

fn main() {
    env_logger::init();

    let mut video = Video::new(VideoSettings {
        fps: 60.0,
        resolution: (1920, 1080),
        duration: Duration::from_secs_f64(5.0),
    });
    video.root().push_clip(1.0, MyClip);
    video.render(vide::quick_export::to("output.mp4"));
}

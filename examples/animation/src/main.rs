use std::time::Duration;

use vide::{sequence::{Clip, ClipInfo}, render::{Time, Renderer}, video::{Video, VideoSettings}, rgb8, api::rect::Rect};

struct MyClip;
impl Clip for MyClip {
    fn init(&mut self, renderer: &mut Renderer) { }

    fn info(&self) -> ClipInfo {
        ClipInfo {
            name: Some("My Epic Clip".to_string()),
            duration: Duration::from_secs(2),
        }
    }

    fn render(&mut self, time: Time, renderer: &mut Renderer) {
        renderer.rect();
        println!("Rendering frame {} of My Epic Clip ({}/300)", time.clip_frame, time.video_frame)
    }
}

fn main() {
    env_logger::init();

    let mut video = Video::new(VideoSettings {
        duration: Duration::from_secs_f64(5.0),
        ..Default::default()
    });
    video.root().push_clip(1.0, MyClip);
    video.render(vide::quick_export::to("output.mp4"));
}

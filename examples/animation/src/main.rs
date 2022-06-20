use std::time::Duration;

use vide::{sequence::{Clip, ClipInfo}, render::{Time, Renderer}, video::{Video, VideoSettings}, rgb8, api::rect::Rect};

#[derive(Default)]
struct MyClip {
    rect: Rect,
}

impl Clip for MyClip {
    fn init(&mut self, renderer: &mut Renderer) {
        self.my_rect.color = rgb8!(0xDA, 0x00, 0x37);
    }

    fn info(&self) -> ClipInfo {
        ClipInfo {
            name: Some("My Epic Clip".to_string()),
            duration: Duration::from_secs(2),
        }
    }

    fn render(&self, time: Time, renderer: &mut Renderer) {
        renderer.render(self.rect);
        println!("Rendering frame {} of My Epic Clip ({}/300)", time.clip_frame, time.video_frame)
    }
}

fn main() {
    env_logger::init();

    let mut video = Video::new(VideoSettings {
        duration: Duration::from_secs_f64(5.0),
        ..Default::default()
    });
    video.root().push_clip(1.0, MyClip::default());
    video.render(vide::quick_export::to("output.mp4"));
}

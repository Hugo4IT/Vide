use std::time::Duration;

use vide::{api::video::{Video, VideoSettings}, api::rect::Rect, keyframes, rgb8};

fn main() {
    env_logger::init();

    let mut video = Video::new(VideoSettings {
        duration: Duration::from_secs_f64(5.0),
        ..Default::default()
    });

    let root = video.root();
    let mut clip = root.new_clip(1.0..3.0)
        .effect(Rect {                                          // Solid rectangle
            position: keyframes!(
                initial (0.0, 800.0),                           // Frame 0  = (0.0, 800.0)
                15 => OUT_CUBIC => (0.0, 0.0),                  // Frame 15 = (0.0, 0.0) aka. Center
                                                                // Transition from frame 0 to 15 handled with EASE_OUT_CUBIC
            ),
            size: keyframes!(
                initial (300.0, 200.0),                         // Frame 0  = (300.0, 200.0)
                15 => OUT_CUBIC => (400.0, 300.0),              // Frame 15 = (400.0, 300.0)
                                                                // Transition from frame 0 to 15 handled with EASE_OUT_CUBIC
            ),
            color: keyframes!(
                initial rgb8!(0xda, 0x00, 0x37),                // Frame 0  = #da0037
                30 => LINEAR => rgb8!(0xda, 0x00, 0x37),        // Frame 30 = #da0037 (State holds for 30 frames)
                45 => OUT_QUADRATIC => rgb8!(0x00, 0xda, 0x37), // Frame 45 = #00da37 (EASE_OUT_QUADRATIC from frame 30 to 45)
            ),
        });

    video.render(vide::quick_export::to("output.mp4"));
}

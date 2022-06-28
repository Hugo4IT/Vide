use std::time::Duration;

use vide::{api::video::{Video, VideoSettings}, api::rect::Rect, keyframes, rgba8};

fn main() {
    env_logger::init();

    let mut video = Video::new(VideoSettings {
        duration: Duration::from_secs_f64(5.0),
        ..Default::default()
    });

    let root = video.root();
    root.new_clip(1.0..5.0)
        .effect(Rect {                                                  // Solid rectangle
            position: keyframes!(
                initial (0.0, -150.0),                                  // Frame 0  = (0.0, -150.0)
                30 => OUT_BACK => (0.0, 0.0),                           // Frame 30 = (0.0, 0.0) aka. Center
                                                                        // Transition from frame 0 to 30 handled with EASE_OUT_BACK
            ),
            size: keyframes!(
                initial (350.0, 250.0),                                 // Frame 0  = (350.0, 250.0)
                30 => OUT_CUBIC => (400.0, 300.0),                      // Frame 30 = (400.0, 300.0)
                                                                        // Transition from frame 0 to 30 handled with EASE_OUT_CUBIC
            ),
            color: keyframes!(
                initial rgba8!(0xda, 0x00, 0x37, 0x00),                 // Frame 0  = #da003700
                30 => OUT_QUADRATIC => rgba8!(0xda, 0x00, 0x37, 0xFF),  // Frame 30 = #da0037FF (EASE_OUT_QUADRATIC from frame 0 to 15 - fade in)
                45 => LINEAR => rgba8!(0xda, 0x00, 0x37, 0xFF),         // Frame 45 = #da0037FF (State holds for 30 frames)
                60 => IN_QUADRATIC => rgba8!(0x00, 0xda, 0x37, 0xFF),   // Frame 60 = #00da37FF (EASE_IN_QUADRATIC from frame 30 to 45 - change color)
            ),
        });

    video.render(vide::quick_export::to("output.mp4"));
}

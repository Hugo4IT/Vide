use std::time::Duration;

use vide::prelude::*;

fn main() {
    env_logger::init();

    let mut video = Video::new(VideoSettings {
        duration: Duration::from_secs_f64(5.0),
        ..Default::default()
    });

    // TODO: Seperate easing into new builder function

    let root = video.root();
    root.new_clip(1.0..5.0)
        .effect(Rect {
            position: Animation::new(60.0)
                .keyframe(Abs(0.0), LINEAR, (-200.0, -150.0))
                .keyframe(Rel(0.3), OUT_BACK, (-200.0, 0.0))
                .build(),
            size: Animation::new(60.0)
                .keyframe(Abs(0.0), LINEAR, (350.0, 250.0))
                .keyframe(Rel(0.3), OUT_CUBIC, (400.0, 300.0))
                .build(),
            color: Animation::new(60.0)
                .keyframe(Abs(0.0), LINEAR, rgba8!(0xda, 0x00, 0x37, 0x00))
                .keyframe(Rel(0.3), OUT_QUADRATIC, rgba8!(0xda, 0x00, 0x37, 0xFF))
                .hold(0.3)
                .keyframe(Rel(0.3), IN_QUADRATIC, rgba8!(0x00, 0xda, 0x37, 0xFF))
                .build(),
        });
    root.new_clip(1.0..3.0)
        .effect(Rect {
            position: Animation::new(60.0)
                .keyframe(Abs(0.0), LINEAR, (200.0, -150.0))
                .keyframe(Rel(0.3), OUT_BACK, (200.0, 0.0))
                .build(),
            size: Animation::new(60.0)
                .keyframe(Abs(0.0), LINEAR, (350.0, 250.0))
                .keyframe(Rel(0.3), OUT_CUBIC, (400.0, 300.0))
                .build(),
            color: Animation::new(60.0)
                .keyframe(Abs(0.0), LINEAR, rgba8!(0xda, 0x00, 0x37, 0x00))
                .keyframe(Rel(0.3), OUT_QUADRATIC, rgba8!(0xda, 0x00, 0x37, 0xFF))
                .hold(0.3)
                .keyframe(Rel(0.3), IN_QUADRATIC, rgba8!(0x00, 0xda, 0x37, 0xFF))
                .build(),
        })
        .effect(Rect {
            position: Animation::new(60.0)
                .keyframe(Abs(0.0), LINEAR, (0.0, -150.0))
                .keyframe(Rel(0.3), OUT_BACK, (0.0, 0.0))
                .build(),
            size: Animation::new(60.0)
                .keyframe(Abs(0.0), LINEAR, (350.0, 250.0))
                .keyframe(Rel(0.3), OUT_CUBIC, (400.0, 300.0))
                .build(),
            color: Animation::new(60.0)
                .keyframe(Abs(0.0), LINEAR, rgba8!(0xda, 0x00, 0x37, 0x00))
                .keyframe(Rel(0.3), OUT_QUADRATIC, rgba8!(0xda, 0x00, 0x37, 0xFF))
                .hold(0.3)
                .keyframe(Rel(0.3), IN_QUADRATIC, rgba8!(0x00, 0xda, 0x37, 0xFF))
                .build(),
        });

    video.render(vide::quick_export::to("output.mp4"));
}

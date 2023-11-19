use std::time::Duration;

use vide::prelude::*;

fn main() {
    env_logger::init();

    let mut video = Video::new(VideoSettings {
        duration: Duration::from_secs_f64(100.0),
        resolution: (1920, 1080),
        background_color: "#171717".into(),
        ..Default::default()
    });

    let root = video.root();
    root.new_clip(0.0..5.0).effect(Rect {
        position: Animation::new(60.0)
            .keyframe(Abs(0.0), ease::LINEAR, (0.0, -590.0))
            .keyframe(Rel(1.0), ease::IN_OUT_QUINTIC, (0.0, 0.0))
            .build(),
        size: Animation::new(60.0)
            .keyframe(Abs(0.0), ease::LINEAR, (100.0, 100.0))
            .hold(1.0)
            .keyframe(Rel(0.6), ease::IN_OUT_QUINTIC, (500.0, 128.0))
            .hold(3.0)
            .keyframe(Rel(0.6), ease::IN_BACK, (0.0, 164.0))
            .build(),
        color: unanimated!("#222222"),
    });

    video.render(vide::quick_export::to("output.mp4"));
}

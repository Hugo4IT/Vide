use std::time::Duration;

use vide::prelude::*;

#[rustfmt::skip]
fn main() {
    env_logger::init();

    let mut video = Video::new(VideoSettings {
        duration: Duration::from_secs_f64(5.0),
        ..Default::default()
    });

    let root = video.root();
    root.new_clip(1.0..5.0).effect(Rect {
        position: unanimated!((-300.0, 0.0)),
        size: unanimated!((200.0, 150.0)),
        color: Animation::new(60.0)
            .keyframe(Abs(0.0), ease::LINEAR,         "#da003700")
            .keyframe(Rel(0.3), ease::OUT_QUADRATIC,  "#da0037")
            .hold(0.3)
            .keyframe(Rel(0.3), ease::IN_QUADRATIC,   "#00da37")
            .build(),
    });
    root.new_clip(1.0..5.0).effect(Rect {
        position: unanimated!((0.0, 0.0)),
        size: unanimated!((200.0, 150.0)),
        color: Animation::new(60.0)
            .keyframe(Abs(0.0), ease::LINEAR,         "#da003700")
            .keyframe(Rel(0.3), ease::OUT_QUADRATIC,  "#da0037")
            .hold(0.3)
            .keyframe(Rel(0.3), ease::IN_QUADRATIC,   "#00da37")
            .build(),
    });
    root.new_clip(1.0..5.0).effect(Rect {
        position: unanimated!((300.0, 0.0)),
        size: unanimated!((200.0, 150.0)),
        color: Animation::new(60.0)
            .keyframe(Abs(0.0), ease::LINEAR,         "#da003700")
            .keyframe(Rel(0.3), ease::OUT_QUADRATIC,  "#da0037")
            .hold(0.3)
            .keyframe(Rel(0.3), ease::IN_QUADRATIC,   "#00da37")
            .build(),
    });

    video.render(vide::quick_export::to("output.mp4"));
}

use std::time::Duration;

use vide::prelude::*;

fn main() {
    env_logger::init();

    let mut video = Video::new(VideoSettings {
        duration: Duration::from_secs_f64(5.0),
        ..Default::default()
    });

    let easing_functions = vec![
        LINEAR,
        IN_QUADRATIC,
        IN_CUBIC,
        IN_QUARTIC,
        IN_QUINTIC,
        IN_EXPONENTIAL,
        OUT_QUADRATIC,
        OUT_CUBIC,
        OUT_QUARTIC,
        OUT_QUINTIC,
        OUT_EXPONENTIAL,
        IN_BACK,
        OUT_BACK,
        IN_OUT_BACK,
    ];

    let rect_size = 600.0 / easing_functions.len() as f32;
    let rect_seperation = 800.0 / easing_functions.len() as f32;
    let pos_left = -820.0 + rect_size * 0.5;
    let pos_right = 820.0 - rect_size * 0.5;
    for (i, &easing) in easing_functions.iter().enumerate() {
        let y_pos = 400.0 - rect_seperation * i as f32 + rect_size * 0.5;
        video.root().new_clip(0.0..5.0).effect(Rect {
            position: Animation::new(60.0)
                .keyframe(Abs(0.0), LINEAR, (pos_left, y_pos))
                .keyframe(Abs(2.0), easing, (pos_right, y_pos))
                .hold(0.5)
                .keyframe(Abs(4.5), easing, (pos_left, y_pos))
                .build(),
            size: unanimated!((rect_size, rect_size)),
            color: unanimated!(rgb8!(0xda, 0x00, 0x37)),
        });
    }

    video.render(vide::quick_export::to("output.mp4"));
}

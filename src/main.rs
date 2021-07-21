mod binary_tree;
mod split_tree;

use nannou::prelude::*;
use split_tree::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    tree: SplitTree,
    num_lines: u32,
    frames_per_cycle: u64,
    save_frame: bool,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .key_pressed(key_pressed)
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();
    Model {
        tree: SplitTree::random(10),
        num_lines: 15,
        frames_per_cycle: 900,
        save_frame: false,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let phase = (app.elapsed_frames() as f32 / model.frames_per_cycle as f32) * PI * 2.0;
    let displace = |split: &Split| Split {
        ratio: ((split.ratio * PI * 2.0) + phase).sin() * 0.4 + 0.5,
        direction: split.direction,
    };
    let rects = model
        .tree
        .map_nodes(&displace)
        .rectangles(app.window_rect());
    let draw = app.draw().scale(0.9);
    draw.background().color(BLACK);

    for rect in rects {
        let (a, b, c, d) = if rect.w() > rect.h() {
            (
                rect.top_left(),
                rect.top_right(),
                rect.bottom_left(),
                rect.bottom_right(),
            )
        } else {
            (
                rect.top_left(),
                rect.bottom_left(),
                rect.top_right(),
                rect.bottom_right(),
            )
        };
        let n =
            (model.num_lines as f32).min(5.0 * abs((rect.w() / rect.h()) - (rect.h() / rect.w())));
        if rect.w() > 10.0 && rect.h() > 10.0 {
            for i in 0..(n.ceil() as u32) {
                draw.line()
                    .weight(1.0)
                    .color(WHITE)
                    .start(Vec2::lerp(a, b, i as f32 / n as f32))
                    .end(Vec2::lerp(c, d, i as f32 / n as f32));
            }
        }
        draw.rect()
            .xy(rect.xy())
            .wh(rect.wh())
            .stroke(WHITE)
            .stroke_weight(1.0)
            .no_fill();
    }

    draw.to_frame(app, &frame).unwrap();

    // save this frame if f was pressed
    if model.save_frame {
        save_frame(app, &frame);
    }

    // uncomment to record looping video frames
    // if app.elapsed_frames() <= model.frames_per_cycle {
    //    let file_path = captured_frame_path(app, &frame);
    //    app.main_window().capture_frame(file_path);
    // } else {
    //     app.quit();
    // }
}

fn save_frame(app: &App, frame: &Frame) {
    let file_path = app
        .project_path()
        .expect("failed to locate `project_path`")
        .join("frames")
        .join(format!("frame-{:04}", frame.nth()))
        .with_extension("jpeg");
    app.main_window().capture_frame(file_path);
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.tree = SplitTree::random(10);
        }
        Key::R => {
            model.save_frame = !model.save_frame;
        }
        _ => {}
    }
}

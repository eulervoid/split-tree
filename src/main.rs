mod binary_tree;
mod split_tree;

use nannou::prelude::*;
use split_tree::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    tree: SplitTree,
    frames_per_cycle: u64,
    save_frame: bool,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .key_pressed(key_pressed)
        .size(1920, 1080)
        .view(view)
        .msaa_samples(4)
        .build()
        .unwrap();

    Model {
        tree: SplitTree::random(8),
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
    let draw = app.draw(); //.scale(0.9);
    draw.background().color(BLACK);

    for rect in rects {
        let [rw, rh] = (rect.wh() / frame.rect().wh()).to_array();
        let hue = rw / (rh + 0.5);
        let colors = [
            hsv(hue, 1.0, 0.8),
            hsv(hue + 0.5, 1.0, 0.8),
            hsv(hue + 0.5, 1.0, 0.8),
            hsv(hue, 1.0, 0.8),
        ];
        let points_colored: Vec<(Vec2, Hsv)> = rect
            .corners_iter()
            .enumerate()
            .map(|(i, corner)| (pt2(corner[0], corner[1]), colors[i]))
            .collect();
        draw.polygon().points_colored(points_colored);
        draw.rect()
            .xy(rect.xy())
            .wh(rect.wh())
            .no_fill()
            .stroke_weight(1.0)
            .stroke_color(rgba(1.0, 1.0, 1.0, 0.2));
    }

    draw.to_frame(app, &frame).unwrap();

    // save this frame if f was pressed
    if model.save_frame {
        save_frame(app, &frame);
    }

    // uncomment to record looping video frames
    // if app.elapsed_frames() <= model.frames_per_cycle {
    //     save_frame(app, &frame);
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

use nannou::prelude::*;

const POINT_SIZE: f32 = 2.;
const TIMER_INCREMENT: f32 = 0.02; //Speed of drawing
const DOT_SPACING: f32 = 1.;
const DOT_COUNT: usize = 80;

struct Model {
    timer: f32,
}

#[allow(dead_code)]
pub fn run() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model { timer: 0. }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.timer += TIMER_INCREMENT;
}

fn x1(t: f32) -> f32 {
    t.sin() * 100. + (t * 3.).cos() * 50. + -(t / 6.).sin() * 30.
}

fn y1(t: f32) -> f32 {
    t.cos() * 100. + (t / 4.).sin() * 200.
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let mut t = _model.timer;

    // Prepare to draw.
    let draw = _app.draw();

    for i in 0..DOT_COUNT {
        t += TIMER_INCREMENT * DOT_SPACING;

        let color_step = 255 / DOT_COUNT;
        let color = (i * color_step) as u8;

        draw.line()
            .color(rgb(color, color, color))
            .weight(POINT_SIZE)
            .start(vec2(x1(t), y1(t)))
            .end(vec2(-x1(t), y1(t)));
        draw.line()
            .color(rgb(color, color, color))
            .weight(POINT_SIZE)
            .start(vec2(-x1(t), y1(t)))
            .end(vec2(-x1(t), -y1(t)));
        draw.line()
            .color(rgb(color, color, color))
            .weight(POINT_SIZE)
            .start(vec2(-x1(t), -y1(t)))
            .end(vec2(x1(t), -y1(t)));
        draw.line()
            .color(rgb(color, color, color))
            .weight(POINT_SIZE)
            .start(vec2(x1(t), -y1(t)))
            .end(vec2(x1(t), y1(t)));
    }

    // clear background
    draw.background().color(BLACK);

    // Write to the window frame.
    draw.to_frame(_app, &_frame).unwrap();
}

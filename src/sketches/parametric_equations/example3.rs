use nannou::prelude::*;

const POINT_SIZE: f32 = 2.;
const TIMER_INCREMENT: f32 = 0.02; //Speed of drawing
const DOT_SPACING: f32 = 4.;
const DOT_COUNT: usize = 12;

struct Model {
    timer: f32,
}

pub fn example3() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model { timer: 0. }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.timer += TIMER_INCREMENT;
}

fn x1(t: f32) -> f32 {
    (t * 1.5).sin() * 100. + (t / 2.).cos() * 100.
}

fn y1(t: f32) -> f32 {
    (t * 1.5).cos() * 100. + (t / 8.).cos() * 200.
}

fn x2(t: f32) -> f32 {
    t.sin() * 150. + (t / 2.).cos() * 150.
}

fn y2(t: f32) -> f32 {
    t.cos() * 150. + -(t / 2.).cos() * 150.
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
            .start(vec2(x1(t), y1(t)))
            .end(vec2(x2(t), y2(t)))
            .weight(POINT_SIZE);

        draw.line()
            .color(rgb(color, 0, 0))
            .start(vec2(-x1(t), y1(t)))
            .end(vec2(-x2(t), y2(t)))
            .weight(POINT_SIZE);

        draw.line()
            .color(rgb(0, color, 0))
            .start(vec2(x1(t), -y1(t)))
            .end(vec2(x2(t), -y2(t)))
            .weight(POINT_SIZE);
        draw.line()
            .color(rgb(0, 0, color))
            .start(vec2(-x1(t), -y1(t)))
            .end(vec2(-x2(t), -y2(t)))
            .weight(POINT_SIZE);
    }

    // clear background
    draw.background().color(BLACK);

    // Write to the window frame.
    draw.to_frame(_app, &_frame).unwrap();
}

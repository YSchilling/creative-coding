use nannou::prelude::*;
use std::f32::consts::E;

const POINT_SIZE: f32 = 2.;
const TIMER_INCREMENT: f32 = 0.005; //Speed of drawing

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
    (t.sin() * (E.pow(t.cos()) - 2. * (4. * t).cos() - (t / 12.).sin().pow(5.))) * 100.
}

fn y1(t: f32) -> f32 {
    (t.cos() * (E.pow(t.cos()) - 2. * (4. * t).cos() - (t / 12.).sin().pow(5.))) * 100.
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let t = _model.timer;

    let higher_bound_blue = 0.7222222222;
    let lower_bound_blue = 0.4722222222;

    let hue = ((_model.timer / 4.).sin() * (higher_bound_blue - lower_bound_blue)).abs()
        + lower_bound_blue;

    // Prepare to draw.
    let draw = _app.draw();

    draw.ellipse()
        .color(hsv(hue, 1., 1.))
        .w_h(POINT_SIZE, POINT_SIZE)
        .x_y(x1(t), y1(t));

    // Write to the window frame.
    draw.to_frame(_app, &_frame).unwrap();
}

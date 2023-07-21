use nannou::prelude::*;

struct Thing {
    pos: Vec2,
}
impl Thing {
    fn new(pos: Vec2) -> Self {
        Thing { pos }
    }
}

struct Model {
    things: Vec<Thing>,
}

const N_THINGS: usize = 80;

#[allow(dead_code)]
pub fn run() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();

    let mut things = Vec::new();
    for _ in 0..N_THINGS {
        things.push(Thing::new(vec2(0., 0.)));
    }

    Model { things }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 60.;
    let things_count = model.things.len();

    for (i, thing) in model.things.iter_mut().enumerate() {
        thing.pos = vec2(
            (TAU / things_count as f32 * i as f32 + time).cos() * 100.,
            (TAU / things_count as f32 * i as f32 + time).sin() * 100.,
        );
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for thing in model.things.iter() {
        draw.ellipse().xy(thing.pos).radius(2.).color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

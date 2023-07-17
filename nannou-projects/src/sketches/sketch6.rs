use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

struct Thing {
    positions: Vec<Vec2>,
}
impl Thing {
    fn new(pos: Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(pos);
        Thing { positions }
    }
}

struct Model {
    things: Vec<Thing>,
    noise: Perlin,
}

const N_THINGS: usize = 5000;

#[allow(dead_code)]
pub fn run() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(512, 512).view(view).build().unwrap();

    let mut things = Vec::new();

    for _ in 0..N_THINGS {
        let thing = Thing::new(vec2(
            (random::<f32>() - 0.5) * 512.,
            (random::<f32>() - 0.5) * 512.,
        ));
        things.push(thing);
    }
    let noise = Perlin::new();
    Model { things, noise }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 20.0;
    let sn = 0.01 + time.cos() as f64 * 0.005;
    for thing in model.things.iter_mut() {
        thing.positions.clear();
        thing.positions.push(vec2(
            (random::<f32>() - 0.5) * 512.,
            (random::<f32>() - 0.5) * 512.,
        ));

        for _ in 0..50 {
            let last = thing.positions[0];
            let new = last
                + vec2(
                    model
                        .noise
                        .get([sn * last.x as f64, sn * last.y as f64, 0.0])
                        as f32,
                    model
                        .noise
                        .get([sn * last.x as f64, sn * last.y as f64, 1.0])
                        as f32,
                );
            thing.positions.insert(0, new);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    for thing in model.things.iter() {
        draw.polyline()
            .points(thing.positions.iter().cloned())
            .color(WHITE);
    }

    draw.rect().w_h(512., 512.).color(srgba(0.0, 0.0, 0.0, 0.1));

    //draw to frame
    draw.to_frame(app, &frame).unwrap();
}

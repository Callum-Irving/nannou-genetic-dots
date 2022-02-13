mod population;

use population::Population;

use nannou::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    // .size(WIDTH, HEIGHT)
    nannou::app(model).update(update).run()
}

struct Model {
    population: Population,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    Model {
        population: Population::new(1000, Vec2::new(0.0, -350.0), 400, Vec2::new(0.0, 350.0)),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.population.all_done() {
        model.population.next_generation();
    } else {
        model.population.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SNOW);

    model.population.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}

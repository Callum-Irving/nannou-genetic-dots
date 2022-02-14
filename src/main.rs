mod population;

use population::Population;

use nannou::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const POPULATION_SIZE: usize = 300;

// TODO: Add feature to run generataions without displaying each frame (quick mode)

fn main() {
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
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        population: Population::new(
            POPULATION_SIZE,
            Vec2::new(0.0, -350.0),
            400,
            Vec2::new(0.0, 350.0),
        ),
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

    let text = String::from("Gen: ")
        + &model.population.gen.to_string()
        + "\nMax steps: "
        + &model.population.max_steps.to_string();
    let win_rect = app.main_window().rect().pad_left(280.0).pad_top(30.0);
    draw.text(&text)
        .color(BLACK)
        .font_size(24)
        .wh(win_rect.wh())
        .xy(win_rect.top_left())
        .left_justify();

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, mode: &mut Model, key: Key) {
    match key {
        Key::R => {
            mode.population = Population::new(
                POPULATION_SIZE,
                Vec2::new(0.0, -350.0),
                400,
                Vec2::new(0.0, 350.0),
            );
        }
        _ => {}
    }
}

mod population;

use population::Population;

use nannou::prelude::*;
use nannou_egui::{egui, Egui};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

const POPULATION_SIZE: usize = 500;
const START: (f32, f32) = (0.0, -350.0);
const GOAL: (f32, f32) = (0.0, 350.0);

fn main() {
    nannou::app(model).update(update).run()
}

#[derive(PartialEq)]
enum AppStatus {
    Waiting,
    Running,
    Looping,
}

struct Model {
    population: Population,
    status: AppStatus,
    ui: Egui,
    wall_start: Option<Point2>,
    walls: Vec<(Point2, Point2)>,
}

fn model(app: &App) -> Model {
    let _main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .key_pressed(key_pressed)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .build()
        .unwrap();

    let ui_window = app
        .new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(170, 160)
        .view(ui_view)
        .raw_event(raw_ui_event)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let ui_window_ref = app.window(ui_window).unwrap();
    let ui = Egui::from_window(&ui_window_ref);

    Model {
        population: Population::new(POPULATION_SIZE, Vec2::from(START), 400, Vec2::from(GOAL)),
        status: AppStatus::Waiting,
        ui,
        wall_start: None,
        walls: vec![],
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_ui(model);
    if model.status == AppStatus::Looping {
        model.population.next_generation();
        while !model.population.all_done() {
            model.population.update(&model.walls);
        }
    } else if model.status == AppStatus::Running && !model.population.all_done() {
        model.population.update(&model.walls);
        if model.population.all_done() {
            model.status = AppStatus::Waiting;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SNOW);

    if let Some(start_point) = model.wall_start {
        draw.line()
            .start(start_point)
            .end(app.mouse.position())
            .weight(4.0)
            .color(GREEN);
    }
    for wall in model.walls.iter() {
        draw.line()
            .start(wall.0)
            .end(wall.1)
            .weight(4.0)
            .color(STEELBLUE);
    }

    model.population.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => {
            model.population =
                Population::new(POPULATION_SIZE, Vec2::from(START), 400, Vec2::from(GOAL));
        }
        Key::Space => {
            if model.status == AppStatus::Waiting {
                model.status = AppStatus::Running;
                model.population.next_generation();
                while !model.population.all_done() {
                    model.population.update(&model.walls);
                }
                model.status = AppStatus::Waiting;
            }
        }
        Key::S => {
            if model.status == AppStatus::Waiting {
                model.population.next_generation();
                model.status = AppStatus::Running;
            }
        }
        _ => {}
    }
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left {
        model.wall_start = Some(app.mouse.position());
    }
}

fn mouse_released(app: &App, model: &mut Model, button: MouseButton) {
    if button == MouseButton::Left && model.wall_start.is_some() {
        model.walls.push((model.wall_start.unwrap(), app.mouse.position()));
        model.wall_start = None;
    }
}

fn ui_view(_app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame(&frame).unwrap();
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update_ui(model: &mut Model) {
    let ctx = model.ui.begin_frame();
    egui::Window::new("Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.label("Generation: ".to_owned() + &model.population.gen.to_string());
            ui.label("Max steps: ".to_owned() + &model.population.max_steps.to_string());

            if model.status != AppStatus::Looping {
                if ui.button("Run slow generation").clicked() && model.status == AppStatus::Waiting
                {
                    model.population.next_generation();
                    model.status = AppStatus::Running;
                };
                if ui.button("Run fast generation").clicked() && model.status == AppStatus::Waiting
                {
                    model.status = AppStatus::Running;
                    model.population.next_generation();
                    while !model.population.all_done() {
                        model.population.update(&model.walls);
                    }
                    model.status = AppStatus::Waiting;
                };
                if ui.button("Loop fast generations").clicked()
                    && model.status == AppStatus::Waiting
                {
                    model.status = AppStatus::Looping;
                }
            } else {
                ui.add_enabled(false, egui::Button::new("Run slow generation"));
                ui.add_enabled(false, egui::Button::new("Run fast generation"));
                if ui.button("Stop looping").clicked() {
                    while !model.population.all_done() {
                        model.population.update(&model.walls);
                    }
                    model.status = AppStatus::Waiting;
                }
            }
        });
}

mod dot;

use self::dot::Dot;

use nannou::prelude::*;

pub struct Population {
    dots: Vec<Dot>,
    gen: usize,
    goal: Point2,
    initial_pos: Point2,
    max_steps: usize,
}

impl Population {
    pub fn new(num_dots: usize, initial_pos: Point2, num_steps: usize, goal: Point2) -> Self {
        let mut dots = Vec::with_capacity(num_dots);
        for _ in 0..num_dots {
            dots.push(Dot::new(initial_pos, num_steps));
        }

        Population {
            dots,
            gen: 0,
            goal,
            initial_pos,
            max_steps: num_steps,
        }
    }

    pub fn update(&mut self) {
        for dot in self.dots.iter_mut() {
            if dot.step > self.max_steps {
                dot.dead = true;
            } else {
                dot.update(self.goal);
            }
        }
    }

    pub fn all_done(&self) -> bool {
        for dot in self.dots.iter() {
            if !dot.dead && !dot.reached_goal {
                return false;
            }
        }

        return true;
    }

    pub fn next_generation(&mut self) {
        // Get best dot
        let best = self
            .dots
            .iter()
            .max_by(|a, b| {
                a.calculate_fitness(self.goal)
                    .partial_cmp(&b.calculate_fitness(self.goal))
                    .unwrap()
            })
            .unwrap();
        if best.reached_goal {
            self.max_steps = best.step;
        }
        let best = best.make_new(self.initial_pos);

        // Do tournament selection
        let mut new_dots = Vec::with_capacity(self.dots.len());
        new_dots.push(best);
        let fitness_sum = self
            .dots
            .iter()
            .fold(0_f32, |acc, dot| acc + dot.calculate_fitness(self.goal));

        for _ in 1..self.dots.len() {
            // Tournament selection
            new_dots.push(self.get_parent(fitness_sum));
        }

        self.dots = new_dots;
        self.gen += 1;
    }

    fn get_parent(&self, fitness_sum: f32) -> Dot {
        let chance = random_range(0.0, fitness_sum);
        let mut running_sum = 0.0;

        for dot in self.dots.iter() {
            running_sum += dot.calculate_fitness(self.goal);
            if running_sum > chance {
                return dot.make_new(self.initial_pos).mutate();
            }
        }

        panic!("Tournament selection failed");
    }

    pub fn draw(&self, draw: &Draw) {
        for dot in self.dots.iter() {
            dot.draw(&draw);
        }

        draw.ellipse()
            .x_y(self.goal.x, self.goal.y)
            .w_h(10.0, 10.0)
            .color(GREEN);
    }
}

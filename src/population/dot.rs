use nannou::prelude::*;

#[derive(Clone)]
pub struct Dot {
    pub step: usize,
    genome: Genome,
    pub dead: bool,
    pub reached_goal: bool,
    pos: Point2,
    vel: Vec2,
}

impl Dot {
    pub fn new(pos: Point2, num_steps: usize) -> Self {
        Dot {
            step: 0,
            genome: Genome::new(num_steps),
            dead: false,
            reached_goal: false,
            pos,
            vel: Vec2::ZERO,
        }
    }

    pub fn update(&mut self, goal: Point2) {
        if self.dead || self.reached_goal {
            return;
        }

        if self.step >= self.genome.directions.len() {
            self.dead = true;
            return;
        }

        self.vel += self.genome.directions[self.step];
        self.vel = self.vel.clamp_length_max(5.0);
        self.pos += self.vel;

        if self.pos.distance(goal) < 5.0 {
            self.reached_goal = true;
        } else if self.pos.x <= -400.0
            || self.pos.x >= 400.0
            || self.pos.y <= -400.0
            || self.pos.y >= 400.0
        {
            self.dead = true;
        }

        self.step += 1;
    }

    pub fn calculate_fitness(&self, goal: Point2) -> f32 {
        if self.reached_goal {
            return 1.0 / 25.0 + 1.0 / (self.step * self.step) as f32;
        } else {
            let dist = self.pos.distance(goal);
            return 1.0 / (dist * dist);
        }
    }

    pub fn make_new(&self, pos: Point2) -> Dot {
        Dot {
            step: 0,
            genome: self.genome.clone(),
            dead: false,
            reached_goal: false,
            pos,
            vel: Vec2::ZERO,
        }
    }

    pub fn mutate(mut self) -> Dot {
        self.genome = self.genome.mutate();
        self
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse().xy(self.pos).w_h(10.0, 10.0).color(RED);
    }
}

#[derive(Clone)]
struct Genome {
    directions: Vec<Vec2>,
}

const MUTATION_RATE: f32 = 0.1;

impl Genome {
    pub fn new(num_steps: usize) -> Self {
        let mut directions: Vec<Vec2> = Vec::with_capacity(num_steps);
        for _ in 0..num_steps {
            let angle = random_range(0.0, TAU);
            directions.push(Vec2::new(angle.cos(), angle.sin()));
        }
        Genome { directions }
    }

    pub fn mutate(&self) -> Self {
        let mut directions = self.directions.clone();
        for dir in directions.iter_mut() {
            let chance = random_range(0.0, 1.0);
            if chance < MUTATION_RATE {
                let angle = random_range(0.0, TAU);
                *dir = Vec2::new(angle.cos(), angle.sin());
            }
        }
        Genome { directions }
    }
}

use std::time::{Duration, Instant};

use egui::{Color32, Painter, layers::ShapeIdx};
use rand::Rng;

pub static SPEED: isize = 10;
static WIDTH: isize = 800;
static HEIGHT: isize = 600;
pub static SPACE_SIZE: isize = 20;

static FOOD_COLOR: Color32 = egui::Color32::RED;
pub static BACKGROUND_COLOR: Color32 = egui::Color32::BLACK;
static SNAKE_COLOR: Color32 = egui::Color32::GREEN;

pub type Coodinate = (isize, isize);

#[derive(PartialEq)]
pub enum Direction {
    Top,
    Right,
    Down,
    Left,
}

pub struct SnakeGame {
    pub width: isize,
    pub height: isize,
    pub score: isize,
    pub snake: Snake,
    pub food: Food,
    last_update: Instant,
}

impl SnakeGame {
    pub fn default() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
            score: 0,
            snake: Snake::new(),
            food: Food::new(),
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        let mut eat = false;
        self.snake.change_direction(ctx, eat);

        if self.last_update.elapsed() >= Duration::from_millis(1000 / SPEED as u64) {
            if self.snake.body[0] == self.food.position {
                eat = true;
                self.score += 1;
                self.food = Food::new();
            }

            self.snake.move_snake(eat);
            self.last_update = Instant::now();
        }
    }
}

pub struct Food {
    pub color: Color32,
    pub position: Coodinate,
}

impl Food {
    pub fn new() -> Self {
        let x = rand::thread_rng().gen_range(0..(WIDTH / SPACE_SIZE)) * SPACE_SIZE;
        let y = rand::thread_rng().gen_range(0..(HEIGHT / SPACE_SIZE)) * SPACE_SIZE;
        Self {
            color: FOOD_COLOR,
            position: (x, y),
        }
    }
}

pub struct Snake {
    body: Vec<Coodinate>,
    direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![(WIDTH / 2, HEIGHT / 2)],
            direction: Direction::Right,
        }
    }

    pub fn change_direction(&mut self, ctx: &egui::Context, eat: bool) {
        if ctx.input(|key| key.key_pressed(egui::Key::ArrowDown)) {
            if self.direction != Direction::Top {
                self.direction = Direction::Down;
                self.move_snake(eat);
            }
        } else if ctx.input(|key| key.key_pressed(egui::Key::ArrowUp)) {
            if self.direction != Direction::Down {
                self.direction = Direction::Top;
                self.move_snake(eat);
            }
        } else if ctx.input(|key| key.key_pressed(egui::Key::ArrowLeft)) {
            if self.direction != Direction::Right {
                self.direction = Direction::Left;
                self.move_snake(eat);
            }
        } else if ctx.input(|key| key.key_pressed(egui::Key::ArrowRight)) {
            if self.direction != Direction::Left {
                self.direction = Direction::Right;
                self.move_snake(eat);
            }
        }
    }

    pub fn move_snake(&mut self, eat: bool) {
        let head = self.body.first().unwrap();

        let new_head = match self.direction {
            Direction::Top => (head.0, head.1 - SPACE_SIZE),
            Direction::Right => (head.0 + SPACE_SIZE, head.1),
            Direction::Down => (head.0, head.1 + SPACE_SIZE),
            Direction::Left => (head.0 - SPACE_SIZE, head.1),
        };

        self.body.insert(0, new_head);

        if !eat {
            self.body.pop();
        }
    }

    pub fn render(&self, painter: &Painter) -> Vec<ShapeIdx> {
        let mut shapes: Vec<ShapeIdx> = Vec::new();

        for &coordinate in self.body.iter() {
            let rect = egui::Rect::from_min_size(
                egui::Pos2 {
                    x: coordinate.0 as f32,
                    y: coordinate.1 as f32,
                },
                egui::Vec2 {
                    x: SPACE_SIZE as f32,
                    y: SPACE_SIZE as f32,
                },
            );

            shapes.push(painter.rect(
                rect,
                0.0,
                SNAKE_COLOR,
                egui::Stroke::new(0.0, BACKGROUND_COLOR),
            ));
        }
        shapes
    }

    pub fn check_collision(&self) -> bool {
        let (x, y) = self.body[0];

        if self.body[1..].contains(&(x, y)) {
            return true;
        }

        x < 0 || x >= WIDTH || y < 0 || y >= HEIGHT
    }
}

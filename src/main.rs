//! Snake Rogue - A fun, addictive snake game with power-ups!
//! Run with: cargo run --release

use eframe::egui;
use rand::Rng;
use std::collections::VecDeque;

const GRID_SIZE: usize = 30;
const CELL_SIZE: f32 = 20.0;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
enum PowerUp {
    Speed,
    Slow,
    DoublePoints,
    Invincible,
    None,
}

#[derive(Clone)]
struct SnakeGame {
    snake: VecDeque<(usize, usize)>,
    direction: Direction,
    next_direction: Direction,
    food: (usize, usize),
    score: i32,
    high_score: i32,
    level: usize,
    game_over: bool,
    paused: bool,
    power_up: Option<PowerUp>,
    power_up_pos: Option<(usize, usize)>,
    power_up_timer: f32,
    speed_multiplier: f32,
    invincibility: bool,
    eating_animation: f32,
}

impl SnakeGame {
    fn new() -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((GRID_SIZE / 2, GRID_SIZE / 2));
        snake.push_back((GRID_SIZE / 2 - 1, GRID_SIZE / 2));
        snake.push_back((GRID_SIZE / 2 - 2, GRID_SIZE / 2));

        let mut rng = rand::thread_rng();
        let food = (rng.gen_range(1..GRID_SIZE - 1), rng.gen_range(1..GRID_SIZE - 1));

        Self {
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            food,
            score: 0,
            high_score: 0,
            level: 1,
            game_over: false,
            paused: false,
            power_up: None,
            power_up_pos: None,
            power_up_timer: 0.0,
            speed_multiplier: 1.0,
            invincibility: false,
            eating_animation: 0.0,
        }
    }

    fn reset(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }
        let mut snake = VecDeque::new();
        snake.push_back((GRID_SIZE / 2, GRID_SIZE / 2));
        snake.push_back((GRID_SIZE / 2 - 1, GRID_SIZE / 2));
        snake.push_back((GRID_SIZE / 2 - 2, GRID_SIZE / 2));

        let mut rng = rand::thread_rng();
        let food = (rng.gen_range(1..GRID_SIZE - 1), rng.gen_range(1..GRID_SIZE - 1));

        self.snake = snake;
        self.direction = Direction::Right;
        self.next_direction = Direction::Right;
        self.food = food;
        self.score = 0;
        self.level = 1;
        self.game_over = false;
        self.paused = false;
        self.power_up = None;
        self.power_up_pos = None;
        self.power_up_timer = 0.0;
        self.speed_multiplier = 1.0;
        self.invincibility = false;
    }

    fn spawn_power_up(&mut self) {
        let mut rng = rand::thread_rng();
        let pos = (rng.gen_range(1..GRID_SIZE - 1), rng.gen_range(1..GRID_SIZE - 1));

        // Don't spawn on snake or food
        if self.snake.contains(&pos) || pos == self.food {
            return;
        }

        let power_type = match rng.gen_range(0..5) {
            0 => PowerUp::Speed,
            1 => PowerUp::Slow,
            2 => PowerUp::DoublePoints,
            3 => PowerUp::Invincible,
            _ => PowerUp::None,
        };

        self.power_up = Some(power_type);
        self.power_up_pos = Some(pos);
        self.power_up_timer = 10.0; // 10 seconds
    }

    fn update(&mut self, dt: f32) {
        if self.game_over || self.paused {
            return;
        }

        self.direction = self.next_direction;

        // Update power-up timer
        if self.power_up_timer > 0.0 {
            self.power_up_timer -= dt;
            if self.power_up_timer <= 0.0 {
                self.power_up = None;
                self.power_up_pos = None;
                self.speed_multiplier = 1.0;
                self.invincibility = false;
            }
        }

        // Eating animation
        if self.eating_animation > 0.0 {
            self.eating_animation -= dt * 3.0;
        }

        // Move snake
        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1.saturating_sub(1)),
            Direction::Down => (head.0, (head.1 + 1).min(GRID_SIZE - 1)),
            Direction::Left => (head.0.saturating_sub(1), head.1),
            Direction::Right => ((head.0 + 1).min(GRID_SIZE - 1), head.1),
        };

        // Check collision with self (unless invincible)
        if !self.invincibility && self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        // Check food collision
        if new_head == self.food {
            let points = if self.power_up == Some(PowerUp::DoublePoints) { 20 } else { 10 };
            self.score += points;
            self.eating_animation = 1.0;

            // Level up every 100 points
            self.level = (self.score / 100) as usize + 1;
            self.speed_multiplier = 1.0 + (self.level as f32 * 0.1);

            // Spawn new food
            let mut rng = rand::thread_rng();
            let mut new_food = (rng.gen_range(1..GRID_SIZE - 1), rng.gen_range(1..GRID_SIZE - 1));
            while self.snake.contains(&new_food) {
                new_food = (rng.gen_range(1..GRID_SIZE - 1), rng.gen_range(1..GRID_SIZE - 1));
            }
            self.food = new_food;

            // Chance to spawn power-up
            if rand::thread_rng().gen_bool(0.3) && self.power_up.is_none() {
                self.spawn_power_up();
            }
        } else {
            self.snake.pop_back();
        }

        // Check power-up collision
        if let Some(power_pos) = self.power_up_pos {
            if new_head == power_pos {
                match self.power_up {
                    Some(PowerUp::Speed) => self.speed_multiplier *= 1.5,
                    Some(PowerUp::Slow) => self.speed_multiplier *= 0.7,
                    Some(PowerUp::DoublePoints) => {},
                    Some(PowerUp::Invincible) => self.invincibility = true,
                    Some(PowerUp::None) | None => {},
                }
                self.power_up = None;
                self.power_up_pos = None;
            }
        }
    }

    fn handle_input(&mut self, key: egui::Key) {
        self.next_direction = match key {
            egui::Key::ArrowUp | egui::Key::W if self.direction != Direction::Down => Direction::Up,
            egui::Key::ArrowDown | egui::Key::S if self.direction != Direction::Up => Direction::Down,
            egui::Key::ArrowLeft | egui::Key::A if self.direction != Direction::Right => Direction::Left,
            egui::Key::ArrowRight | egui::Key::D if self.direction != Direction::Left => Direction::Right,
            _ => self.next_direction,
        };
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 700.0])
            .with_min_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Snake Rogue 🐍",
        options,
        Box::new(|_cc| Ok(Box::new(SnakeApp::new()))),
    )
}

struct SnakeApp {
    game: SnakeGame,
    last_update: f64,
}

impl SnakeApp {
    fn new() -> Self {
        Self {
            game: SnakeGame::new(),
            last_update: 0.0,
        }
    }
}

impl eframe::App for SnakeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_time = ctx.input(|i| i.time);

        // Handle input
        ctx.input(|i| {
            for key in &i.keys_down {
                self.game.handle_input(*key);
            }
            if i.key_pressed(egui::Key::Space) {
                if self.game.game_over {
                    self.game.reset();
                } else {
                    self.game.paused = !self.game.paused;
                }
            }
            if i.key_pressed(egui::Key::R) {
                self.game.reset();
            }
        });

        // Update game at fixed rate (adjusted by speed multiplier)
        let update_interval = 0.15 / self.game.speed_multiplier;
        if current_time - self.last_update > update_interval as f64 {
            self.game.update(0.1);
            self.last_update = current_time;
        }

        // Request repaint for smooth animation
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🐍 Snake Rogue");
            ui.horizontal(|ui| {
                ui.label(format!("Score: {}", self.game.score));
                ui.label(format!("Level: {}", self.game.level));
                ui.label(format!("High Score: {}", self.game.high_score));
            });

            if self.game.paused {
                ui.centered_and_justified(|ui| {
                    ui.heading("⏸️ PAUSED");
                    ui.label("Press SPACE to continue");
                });
            } else if self.game.game_over {
                ui.centered_and_justified(|ui| {
                    ui.heading("💀 GAME OVER");
                    ui.label(format!("Final Score: {}", self.game.score));
                    ui.label("Press SPACE or R to restart");
                });
            }

            // Draw game
            let painter = ui.painter_at(ui.max_rect().clone());

            // Background
            painter.rect_filled(
                egui::Rect::from_min_size(egui::pos2(50.0, 100.0), egui::vec2(GRID_SIZE as f32 * CELL_SIZE, GRID_SIZE as f32 * CELL_SIZE)),
                0.0,
                egui::Color32::from_rgb(20, 20, 30),
            );

            // Grid
            for i in 0..=GRID_SIZE {
                let x = 50.0 + i as f32 * CELL_SIZE;
                let y_start = 100.0;
                let y_end = 100.0 + GRID_SIZE as f32 * CELL_SIZE;
                painter.line_segment(
                    [egui::pos2(x, y_start), egui::pos2(x, y_end)],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 50)),
                );
                let y = 100.0 + i as f32 * CELL_SIZE;
                let x_start = 50.0;
                let x_end = 50.0 + GRID_SIZE as f32 * CELL_SIZE;
                painter.line_segment(
                    [egui::pos2(x_start, y), egui::pos2(x_end, y)],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 50)),
                );
            }

            // Draw food with pulse effect
            let pulse = (current_time * 4.0).sin() * 0.3 + 0.7;
            let food_color = egui::Color32::from_rgb(
                (255.0 * pulse) as u8,
                100,
                100,
            );
            let (fx, fy) = self.game.food;
            let food_rect = egui::Rect::from_min_size(
                egui::pos2(50.0 + fx as f32 * CELL_SIZE + 2.0, 100.0 + fy as f32 * CELL_SIZE + 2.0),
                egui::vec2(CELL_SIZE - 4.0, CELL_SIZE - 4.0),
            );
            painter.rect_filled(food_rect, 5.0, food_color);

            // Draw power-up
            if let Some(pos) = self.game.power_up_pos {
                let (px, py) = pos;
                let power_color = match self.game.power_up {
                    Some(PowerUp::Speed) => egui::Color32::YELLOW,
                    Some(PowerUp::Slow) => egui::Color32::from_rgb(100, 200, 255),
                    Some(PowerUp::DoublePoints) => egui::Color32::GREEN,
                    Some(PowerUp::Invincible) => egui::Color32::GOLD,
                    Some(PowerUp::None) | None => egui::Color32::WHITE,
                };
                let power_rect = egui::Rect::from_min_size(
                    egui::pos2(50.0 + px as f32 * CELL_SIZE + 3.0, 100.0 + py as f32 * CELL_SIZE + 3.0),
                    egui::vec2(CELL_SIZE - 6.0, CELL_SIZE - 6.0),
                );
                painter.rect_filled(power_rect, 3.0, power_color);
            }

            // Draw snake
            for (i, (x, y)) in self.game.snake.iter().enumerate() {
                let is_head = i == 0;
                let alpha = if is_head { 1.0 } else { 1.0 - (i as f32 * 0.03).min(0.5) };

                let snake_color = if is_head {
                    if self.game.invincibility {
                        egui::Color32::GOLD
                    } else {
                        egui::Color32::from_rgb(100, 220, 100)
                    }
                } else {
                    egui::Color32::from_rgba_unmultiplied(80, 180, 80, (255.0 * alpha) as u8)
                };

                let size = if is_head { CELL_SIZE - 2.0 } else { CELL_SIZE - 4.0 };
                let offset = if is_head { 1.0 } else { 2.0 };

                let rect = egui::Rect::from_min_size(
                    egui::pos2(50.0 + *x as f32 * CELL_SIZE + offset, 100.0 + *y as f32 * CELL_SIZE + offset),
                    egui::vec2(size, size),
                );
                painter.rect_filled(rect, if is_head { 3.0 } else { 4.0 }, snake_color);

                // Eyes for head
                if is_head {
                    let eye_size = 3.0;
                    let (ex1, ey1, ex2, ey2) = match self.game.direction {
                        Direction::Up => (CELL_SIZE * 0.3, CELL_SIZE * 0.3, CELL_SIZE * 0.7, CELL_SIZE * 0.3),
                        Direction::Down => (CELL_SIZE * 0.3, CELL_SIZE * 0.7, CELL_SIZE * 0.7, CELL_SIZE * 0.7),
                        Direction::Left => (CELL_SIZE * 0.3, CELL_SIZE * 0.3, CELL_SIZE * 0.3, CELL_SIZE * 0.7),
                        Direction::Right => (CELL_SIZE * 0.7, CELL_SIZE * 0.3, CELL_SIZE * 0.7, CELL_SIZE * 0.7),
                    };
                    painter.circle_filled(
                        egui::pos2(50.0 + *x as f32 * CELL_SIZE + ex1, 100.0 + *y as f32 * CELL_SIZE + ey1),
                        eye_size,
                        egui::Color32::BLACK,
                    );
                    painter.circle_filled(
                        egui::pos2(50.0 + *x as f32 * CELL_SIZE + ex2, 100.0 + *y as f32 * CELL_SIZE + ey2),
                        eye_size,
                        egui::Color32::BLACK,
                    );
                }
            }

            // Power-up timer bar
            if self.game.power_up_timer > 0.0 {
                let bar_width = 200.0;
                let bar_height = 10.0;
                let bar_x = 50.0;
                let bar_y = 70.0;
                let progress = self.game.power_up_timer / 10.0;

                painter.rect_filled(
                    egui::Rect::from_min_size(egui::pos2(bar_x, bar_y), egui::vec2(bar_width, bar_height)),
                    2.0,
                    egui::Color32::from_rgb(50, 50, 50),
                );
                painter.rect_filled(
                    egui::Rect::from_min_size(egui::pos2(bar_x, bar_y), egui::vec2(bar_width * progress, bar_height)),
                    2.0,
                    egui::Color32::from_rgb(100, 200, 255),
                );
            }

            // Instructions
            ui.separator();
            ui.label("🎮 Controls: Arrow Keys / WASD to move");
            ui.label("⏸️ SPACE: Pause/Restart | R: Reset");
            ui.label("⚡ Power-ups: 🟨Speed 🟦Slow 🟩2xPts 🟧Invincible");
        });
    }
}

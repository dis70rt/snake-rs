use std::time::Duration;

use crate::game::{BACKGROUND_COLOR, SPACE_SIZE, SnakeGame};
use eframe::egui;

impl eframe::App for SnakeGame {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if self.snake.check_collision() {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Game Over");
                    ui.label(format!("Your score: {}", self.score));
                });
            });
        } else {
            self.snake.change_direction(ctx, false);
            self.update(ctx);

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.centered_and_justified(|ui| ui.heading(format!("score: {}", self.score)))
                });

                let painter = ui.painter();
                self.snake.render(painter);
                
                ctx.request_repaint_after(Duration::from_millis(16));

                painter.rect(
                    egui::Rect::from_min_size(
                        egui::Pos2::new(self.food.position.0 as f32, self.food.position.1 as f32),
                        egui::vec2(SPACE_SIZE as f32, SPACE_SIZE as f32),
                    ),
                    0.0,
                    self.food.color,
                    egui::Stroke::new(1.0, BACKGROUND_COLOR),
                )
            });
        }
    }
}

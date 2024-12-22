use game::SnakeGame;

mod game;
mod ui;

fn main() {
    let snakegame = SnakeGame::default();
    let options = eframe::NativeOptions {
        vsync: true,
        multisampling: 4,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(snakegame.width as f32, snakegame.height as f32)),
        hardware_acceleration: eframe::HardwareAcceleration::Required,
        renderer: eframe::Renderer::Glow,
        run_and_return: false,
        centered: true,
        ..Default::default()
    };

    eframe::run_native("snake-rs", options, Box::new(|_| Ok(Box::new(snakegame)))).unwrap();
}

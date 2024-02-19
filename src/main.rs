mod models;

fn main() {
    let mut game = models::game::Game::new();
    game.main_loop();
}

use console::Term;

mod models;

fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    let mut game = models::game::Game::new();
    game.main_loop();
}

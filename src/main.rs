use serde::{Serialize, Deserialize};
mod models;



#[derive(Serialize, Deserialize, Debug)]
enum Fame {
    Novice,
    Apprentice,
    Veteran,
    Champion,
    Hero,
    Legend
}



fn main() {
    let mut game = models::game::Game::new();
    game.main_loop();
}
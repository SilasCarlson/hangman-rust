mod player;
mod game;
mod phrase;

use game::Game;

fn main() {
    // Create the game and run it
    let mut game = Game::new();
    game.run();
}
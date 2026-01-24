mod player;
mod game;
mod phrase;

use game::Game;
use phrase::Phrase;
use player::Player;

fn main() {
    // Define vars
    let mut player_name = String::new();

    // Introduction
    clearscreen::clear().expect("Failed to clear screen");
    println!("Hello there! Welcome to my Hangman game.");

    // Get the player name
    println!("What is your name?");
    std::io::stdin().read_line(&mut player_name).expect("Failed to read line");

    // Create the game
    let mut game = Game::new(Player::new(player_name.trim().to_string(), 10), Phrase::get_random());

    // Start the game
    game.run();
}
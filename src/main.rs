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
    println!("Hello there! Welcome to my Hangman game.");

    // Get the player name
    println!("What is your name?");
    std::io::stdin().read_line(&mut player_name).expect("Failed to read line");

    // Create the game
    let mut game = Game::new(Player::new(player_name), Phrase::get_random());

    // Instructions
    println!("Welcome, {}, to my Hangman game.", game.get_player().get_name());
    println!("The category for today is: {}", game.get_phrase_category());

    // Start the game
    game.run();
}
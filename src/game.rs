use crate::phrase::Phrase;
use std::collections::HashSet;
use crate::player::Player;

pub struct Game {
    phrase: Phrase,
    player: Player,
    guessed_letters: HashSet<char>
}

impl Game {
    pub fn new() -> Game {
        // Define vars
        let mut player_name = String::new();

        // Introduction
        println!("Hello there! Welcome to my Hangman game.");

        // Get the player name
        println!("What is your name?");
        std::io::stdin().read_line(&mut player_name).expect("Failed to read line");

        // Create the game
        let game = Game {
            phrase: Phrase::get_random(),
            player: Player::new(String::from(player_name.trim())),
            guessed_letters: HashSet::new()
        };

        // Instructions
        println!("Welcome, {}, to my Hangman game.", player_name);
        println!("The category for today is: {}", game.get_phrase_category());

        // return
        game
    }

    pub fn run(&mut self) {
        let mut guess = String::new();

        while !self.guess_matches_word(&guess.trim().to_string()) {
            // Clear the user input before getting the new input
            guess.clear();

            // Get user input
            println!("{}", self.get_current_words());
            println!("Guess a letter");
            std::io::stdin().read_line(&mut guess).expect("Failed to read line!");

            // Only guess a letter if the inserted string is a char
            if guess.trim().len() == 1 {
                let first_char = guess.trim().chars().next().expect("You must type a string!");
                self.guess_letter(first_char.to_ascii_lowercase());
            }

            self.player.set_guesses(self.player.get_guesses() + 1);
        }
    }

    pub fn guess_letter(&mut self, guessed_letter: char) {
        if !self.guessed_letters.contains(&guessed_letter) {
            self.guessed_letters.insert(guessed_letter);
        }
    }

    pub fn guess_matches_word(&self, guess: &str) -> bool {
        guess.eq_ignore_ascii_case(self.phrase.get_words())
    }

    pub fn get_phrase_category(&self) -> &str {
        &self.phrase.get_category()
    }

    pub fn get_current_words(&self) -> String {
        self.phrase.get_words_with_hidden_letters(&self.guessed_letters)
    }
}
use crate::phrase::Phrase;
use std::collections::HashSet;
use crate::player::Player;

pub struct Game {
    phrase: Phrase,
    player: Player,
    guessed_letters: HashSet<char>
}

impl Game {
    pub fn new(player: Player, phrase: Phrase) -> Self {
        Game {
            phrase,
            player,
            guessed_letters: HashSet::new()
        }
    }

    pub fn run(&mut self) {
        let mut guess = String::new();

        while !self.guess_matches_word(guess.trim()) {
            // Clear the user input before getting the new input
            guess.clear();

            // Display the words to the player
            println!("{}", self.get_current_words());

            // Get the player input
            println!("Guess a letter");
            std::io::stdin().read_line(&mut guess).expect("Failed to read line!");

            // Only guess a letter if the inserted string is a char
            let guess_trimmed = guess.trim();
            if guess_trimmed.len() == 1 {
                let first_char = guess_trimmed.chars().next().expect("You must type a string!");
                self.guess_letter(first_char.to_ascii_lowercase());
            }

            // Increment guesses
            self.player.set_guesses(self.player.get_guesses() + 1);
        }

        // The player wins!
        println!("Congratulations! You guessed the phrase in {} guesses", self.player.get_guesses());
    }

    pub fn guess_letter(&mut self, guessed_letter: char) {
        // hashsets already check for duplicates so we don't need to check for
        // duplicates here
        self.guessed_letters.insert(guessed_letter);
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

    pub fn get_player(&self) -> &Player {
        &self.player
    }
}
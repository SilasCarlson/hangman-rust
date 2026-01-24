use crate::phrase::Phrase;
use std::collections::HashSet;
use crate::player::Player;

enum GameState {
    Initialization,
    PlayerGuess,
    PlayerWon,
    PlayerLost,
    RoundComplete,
    Quit
}

pub struct Game {
    phrase: Phrase,
    player: Player,
    guessed_letters: HashSet<char>,
    state: GameState
}

impl Game {
    /**
        Creates a new Game and returns it
    */
    pub fn new(player: Player, phrase: Phrase) -> Self {
        Game {
            phrase,
            player,
            guessed_letters: HashSet::new(),
            state: GameState::Initialization
        }
    }

    /**
        Function for the initialization game state, resets all necessary variables in order for the
        player to play again (if this is not the first time playing)
    */
    pub fn initialize(&mut self) {
        // Reset if this is NOT the first time playing
        // Reason we only do this for after the first time is because when the game is initialized
        // everything is good to go.
        if self.player.get_guesses() > 0 {
            self.player.reset_guesses();
            self.phrase.reset();
            self.guessed_letters.clear();
        }

        // Update state
        self.set_state(GameState::PlayerGuess);
    }

    /**
        Function for the player guess game state, displays the hidden words to the player and waits for
        their guess and then matches it to the phrase or appends it to the guessed letters hashset.
    */
    pub fn player_guess(&mut self) {
        // Initialize variables
        let mut guess = String::new();

        // Clear the screen and display the hidden words to the user
        clearscreen::clear().expect("failed to clear screen");
        println!("Category: {} - {}", self.phrase.get_category(), self.phrase.get_movie());
        println!("{}", self.phrase.get_words_with_hidden_letters(&self.guessed_letters));
        println!("You have guessed: {:?}", self.guessed_letters);

        // Get user input
        println!("Guess a letter ({}/{} guesses remaining)", self.player.get_maximum_allowed_guesses() - self.player.get_guesses(), self.player.get_maximum_allowed_guesses());
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");

        // If only a character was guessed then add it to the guessed letters
        // Otherwise we will assume the player tried to guess the phrase
        let trimmed_guess = guess.trim();
        if trimmed_guess.len() == 1 {
            // insert the character into the HashSet
            // duplication is already check by Rust, so we do not need to check for duplicates
            self.guessed_letters.insert(trimmed_guess.chars().next().expect("You must type a string!").to_ascii_lowercase());
        }

        // Increment player guesses
        self.player.increment_guesses();

        // Check to see if the player has won or not
        if self.phrase.all_letters_guessed(&self.guessed_letters) || self.phrase.guess_matches_words(trimmed_guess) {
            // The player wins!
            self.set_state(GameState::PlayerWon);
        } else if self.player.get_guesses() >= self.player.get_maximum_allowed_guesses() {
            // The player loses!
            self.set_state(GameState::PlayerLost);
        }
    }

    /**
        Function for the player won game state, display the win state to the player
    */
    pub fn player_won(&mut self) {
        // Increment player's wins
        self.player.increment_wins();

        // Display win screen and clear the screen
        clearscreen::clear().expect("failed to clear screen");
        println!("You won {}! It took you {} guesses.", self.player.get_name(), self.player.get_guesses());
        println!("The phrase was: {}", self.phrase.get_words());
        println!("You currently are {}-{} (wins-losses).", self.player.get_wins(), self.player.get_losses());

        // Update state
        self.set_state(GameState::RoundComplete);
    }

    /**
        Function for the player lost game state, display the win state to the player
    */
    pub fn player_lost(&mut self) {
        // Increment player's losses
        self.player.increment_losses();

        // Display the lost screen and clear the screen
        clearscreen::clear().expect("failed to clear screen");
        println!("You lost {}!", self.player.get_name());
        println!("The phrase was: {}", self.phrase.get_words());
        println!("You currently are {}-{} (wins-losses).", self.player.get_wins(), self.player.get_losses());

        // Update state
        self.set_state(GameState::RoundComplete);
    }

    /**
        Function for the round complete game state, asks if the player wants to play again, if yes
        then updates the game state for initialization otherwise sets the game state for quitting.
    */
    pub fn round_complete(&mut self) {
        // Initialize variables
        let mut response = String::new();

        // Get player's response
        println!("Would you like to play again y/n?");
        std::io::stdin().read_line(&mut response).expect("Failed to read line");

        // If the user typed a valid response to continue then continue otherwise slot the game to quit
        let trimmed_response = response.trim().to_ascii_lowercase();
        if trimmed_response == "y" || trimmed_response == "yes" {
            // user responded with yes, therefore update the state to the initialization
            self.set_state(GameState::Initialization);
        } else {
            // user wants to quit
            self.set_state(GameState::Quit);
        }
    }

    /**
        Updates the state of the Game
        Notes: we can add transitions between states here in the future if desired
    */
    fn set_state(&mut self, new_state: GameState) {
        self.state = new_state;
    }

    /**
        The main loop of the Game here. Uses a match statement and runs the corresponding function
        depending on the game's current state.
    */
    pub fn run(&mut self) {
        // This function will run until it reaches a break statement
        loop {
            match self.state {
                GameState::Initialization => self.initialize(),
                GameState::PlayerGuess => self.player_guess(),
                GameState::PlayerWon => self.player_won(),
                GameState::PlayerLost => self.player_lost(),
                GameState::RoundComplete => self.round_complete(),
                GameState::Quit => {
                    println!("Thanks for playing!");
                    break;
                }
            }
        }
    }
}
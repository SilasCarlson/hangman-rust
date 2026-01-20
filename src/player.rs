pub struct Player {
    name: String,
    guesses: i8
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            guesses: 0
        }
    }

    pub fn set_guesses(&mut self, guesses: i8) {
        self.guesses = guesses;
    }

    pub fn get_guesses(&self) -> i8 {
        self.guesses
    }
}
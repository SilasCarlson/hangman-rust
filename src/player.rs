pub struct Player {
    name: String,
    guesses: i8,
    maximum_allowed_guesses: i8,
    wins: i8,
    losses: i8
}

impl Player {
    /**
        Creates a new Player struct and returns it
    */
    pub fn new(name: String, maximum_allowed_guesses: i8) -> Self {
        Player {
            name,
            guesses: 0,
            maximum_allowed_guesses,
            wins: 0,
            losses: 0
        }
    }

    /**
        Returns the words variable
    */
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /**
        Increments the guesses variable by 1
    */
    pub fn increment_guesses(&mut self) {
        self.guesses += 1;
    }

    /**
        Resets the guesses to 0
    */
    pub fn reset_guesses(&mut self) {
        self.guesses = 0;
    }

    /**
        Returns the guesses variable
    */
    pub fn get_guesses(&self) -> i8 {
        self.guesses
    }

    /**
        Returns the maximum allowed guesses variable
    */
    pub fn get_maximum_allowed_guesses(&self) -> i8 {
        self.maximum_allowed_guesses
    }

    /**
        Returns the wins variable
    */
    pub fn get_wins(&self) -> i8 {
        self.wins
    }

    /**
        Returns the losses variable
    */
    pub fn get_losses(&self) -> i8 {
        self.losses
    }

    /**
        Increments the wins variable by 1
    */
    pub fn increment_wins(&mut self) {
        self.wins += 1;
    }

    /**
        Increments the losses variable by 1
    */
    pub fn increment_losses(&mut self) {
        self.losses += 1;
    }
}
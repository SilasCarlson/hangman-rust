use csv::Reader;
use rand::Rng;
use std::fs::File;
use std::collections::HashSet;

// implement the serde::Deserialize trait
#[derive(serde::Deserialize)]
pub struct Phrase {
    category: String,
    words: String,
    movie: String
}

impl Phrase {
    /**
        Returns a phrase with a random set of words provided by the phrases.csv file
    */
    pub fn get_random() -> Phrase {
        // Open the file and create a CSV reader to read it
        let file = File::open(String::from("data/phrases.csv")).expect("phrases.csv not found");
        let mut reader = Reader::from_reader(file);

        // Read all the records into a string vector
        let mut records: Vec<csv::StringRecord> = Vec::new();
        for line in reader.records() {
            records.push(line.expect("Error reading phrase records"));
        }

        // Generate a random index
        let mut rng = rand::rng();
        let random_index = rng.random_range(0..records.len());

        // Return
        let header = csv::StringRecord::from(vec![
            "category", "words", "movie"
        ]);
        let random_phrase: Phrase = records[random_index].deserialize(Some(&header)).expect("Error deserializing phrase");
        random_phrase
    }

    /**
        Resets the phrase, and updates it's variables to a random phrase available in the CSV file
    */
    pub fn reset(&mut self) {
        // get a random phrase
        let new_phrase = Phrase::get_random();

        // update this phrase with the new phrase's props
        self.category = new_phrase.category;
        self.words = new_phrase.words;
        self.movie = new_phrase.movie;
    }

    /**
        Returns the display string of all characters replaced with a _, while having all guessed letters not being replaced.
    */
    pub fn get_words_with_hidden_letters(&self, hidden_letters: &HashSet<char>) -> String {
        // Initialize Variables
        let mut words = String::new();

        // for every char in this phrase's words string
        // if the hidden letters has the currently selected char, then add that char
        // to the final string otherwise add an underscore (_)
        for char in self.words.chars() {
            if hidden_letters.contains(&char.to_ascii_lowercase()) {
                words.push(char);
                continue;
            }

            // default added char
            // - and ' will be added
            words.push(if char == '-' { '-' } else if char == '\'' { '\'' } else if char != ' ' { '_' } else { ' ' });
        }

        words
    }

    /**
        Returns if all the letters in the phrase are guessed using the provided HashSet
    */
    pub fn all_letters_guessed(&self, guessed_letters: &HashSet<char>) -> bool {
        // Get all the words chars and then filter them ensuring that only letters are passed
        // Then ensure that every character is present in the guessed_letters HashSet
        self.words.chars().filter(|char| char.is_alphabetic()).all(|char| guessed_letters.contains(&char.to_ascii_lowercase()))
    }

    /**
        Returns if the guess is the same as the phrase
    */
    pub fn guess_matches_words(&self, guess: &str) -> bool {
        // Lowercase both strings and ensure that they are equal
        // Return that boolean result
        self.words.to_ascii_lowercase() == guess.to_ascii_lowercase()
    }

    /**
        Returns the words variable
    */
    pub fn get_words(&self) -> &String {
        &self.words
    }

    /**
        Returns the category variable
    */
    pub fn get_category(&self) -> &String {
        &self.category
    }

    /**
        Returns the movie variable
    */
    pub fn get_movie(&self) -> &String {
        &self.movie
    }
}
use csv::Reader;
use rand::Rng;
use std::fs::File;
use std::collections::HashSet;

#[derive(serde::Deserialize)]
pub struct Phrase {
    category: String,
    words: String
}

impl Phrase {
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
            "category", "words", "number_of_words", "number_of_letters", "first_word_letters"
        ]);
        let random_phrase: Phrase = records[random_index].deserialize(Some(&header)).expect("Error deserializing phrase");
        random_phrase
    }

    pub fn get_words_with_hidden_letters(&self, hidden_letters: &HashSet<char>) -> String {
        let mut words = String::new();

        for char in self.words.chars() {
            if hidden_letters.contains(&char.to_ascii_lowercase()) {
                words.push(char);
                continue;
            }

            words.push(if char == '-' { '-' } else if char != ' ' { '_' } else { ' ' });
        }

        words
    }

    pub fn get_words(&self) -> &String {
        &self.words
    }

    pub fn get_category(&self) -> &String {
        &self.category
    }
}
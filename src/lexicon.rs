use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead; // required for BufReader::lines()??? I don't get.
use std::path::Path;

pub struct Lexicon {
    // If we are enforcing ascii, I assume there's a better way to do this than using Strings.
    words_by_length: Vec<HashSet<String>>,
}

//constructors
impl Lexicon {
    /// Empty Lexicon
    pub fn empty() -> Self {
        Self {
            words_by_length: vec![HashSet::new()],
        }
    }

    /// Lexicon from a list of words
    ///
    /// Will silently ignore non-ascii words.
    pub fn from_words(words: Vec<String>) -> Self {
        // get max word length
        let max_length = words.iter().map(|word| word.len()).max().unwrap_or(0);

        // fill the set for each length
        let mut words_by_length = vec![HashSet::new(); max_length + 1];
        for word in words {
            if !word.chars().all(|c| c.is_ascii()) {
                continue;
            }
            let word = word.to_ascii_uppercase();

            words_by_length[word.len()].insert(word);
        }

        Self { words_by_length }
    }

    /// Lexicon from a file.
    ///
    /// Reads the file contents to memory and passes to `Lexicon::from_words`.
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self, io::Error> {
        let file = File::open(filename)?;

        // I don't love reading everything into memory, but had a hard time doing anything
        // smarter. the lexicon has to fit in memory unless we get fancy, so this isn't
        // going to be too big.
        let words = io::BufReader::new(file).lines().collect::<Result<_, _>>()?;
        Ok(Self::from_words(words))
    }
}

impl Lexicon {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lexicon() {
        let lexicon = Lexicon::empty();
        // TODO something
    }

    fn from_words() {
        let words = vec!["cat".to_string(), "dog".to_string()];
        let lexicon = Lexicon::from_words(words);
        // TODO something
    }

    fn from_file() {
        let path = std::env::temp_dir().join("test_words.txt");
        std::fs::write(&path, "cat\ndog\nbear").unwrap();
        let lexicon = Lexicon::from_file(path);

        // TODO something
    }
}

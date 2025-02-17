use unicode_segmentation::UnicodeSegmentation;

use super::puzzle::Slot; // I don't love this dependency. Very open to other approaches.
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead; // required for BufReader::lines()??? I don't get.
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Lexicon {
    // If we are enforcing ascii, I assume there's a better way to do this than using Strings.
    words: Vec<HashSet<String>>,
    empty_set: HashSet<String>, // used for word lengths that aren't in the lexicon
}

//constructors
impl Lexicon {
    /// Empty Lexicon
    pub fn empty() -> Self {
        Self {
            words: vec![HashSet::new()],
            empty_set: HashSet::new(),
        }
    }

    /// Lexicon from a list of words
    ///
    /// Will silently ignore non-ascii words.
    pub fn from_words(words: Vec<String>) -> Self {
        // TODO what's the right way to generalize this to unicode? Do we even want to do that?
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

        Self {
            words: words_by_length,
            empty_set: HashSet::new(),
        }
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

// basic methods
impl Lexicon {
    /// Total number of words in the lexicon
    pub fn len(&self) -> usize {
        self.words.iter().map(|w| w.len()).sum()
    }

    /// HashSet of words of a given length
    pub fn words_by_length(&self, length: usize) -> &HashSet<String> {
        if length >= self.words.len() {
            &self.empty_set
        } else {
            &self.words[length]
        }
    }

    /// Possible answers for a given slot
    pub fn posible_answers(&self, slot: &Slot) -> Vec<String> {
        // should this be an iterator instead of a vector?
        let mut answers = vec![];
        for word in self.words_by_length(slot.len()) {
            let mut matches = true;
            let chars = UnicodeSegmentation::graphemes(word.as_str(), true);
            for (i, c) in chars.enumerate() {
                // THIS ASSUMES UNFILLED SQUARE ARE REPRESENTED BY A SPACE
                if slot[i] != ' ' && slot[i].to_string() != c {
                    matches = false;
                    break;
                }
            }

            if matches {
                answers.push(word.clone());
            }
        }
        answers
    }
}

impl fmt::Display for Lexicon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexicon with {} words", self.len())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_lexicon() {
        assert_eq!(Lexicon::empty().len(), 0);
        assert_eq!(Lexicon::empty().words_by_length(3).len(), 0);
    }

    #[test]
    fn test_lexicon_from_words() {
        let words = vec!["cat".to_string(), "dog".to_string()];
        let lexicon = Lexicon::from_words(words);
        assert_eq!(lexicon.len(), 2);
        assert_eq!(lexicon.words_by_length(3).len(), 2);
        assert_eq!(lexicon.words_by_length(4).len(), 0);
    }

    #[test]
    fn test_lexicon_from_file() {
        let path = std::env::temp_dir().join("test_words.txt");
        std::fs::write(&path, "cat\ndog\nbear").unwrap();
        let lexicon = Lexicon::from_file(path).unwrap();
        assert_eq!(lexicon.len(), 3);
        assert_eq!(lexicon.words_by_length(3).len(), 2);
        assert_eq!(lexicon.words_by_length(5).len(), 0);
    }

    // TODO test posible_answers
}

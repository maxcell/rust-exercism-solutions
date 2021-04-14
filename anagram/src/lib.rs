use std::collections::{HashMap, HashSet};


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let mut letter_count: HashMap<char, u32> = HashMap::new();
    let mut matches = HashSet::new();

    for letter in word.chars() {
        for lowercased_letter in letter.to_lowercase() {
            let counter = letter_count.entry(lowercased_letter).or_insert(0);
            *counter += 1;
        }
    }

    'possible_word_loop: for curr_word in possible_anagrams {
        // Counts the letters of the current anagram we're on
        if curr_word.len() != word.len() {
            continue 'possible_word_loop;
        }

        if *curr_word.to_lowercase() == word.to_lowercase() {
            continue 'possible_word_loop;
        }

        let mut ana_letter_count: HashMap<char, u32> = HashMap::new();
        for letter in curr_word.chars() {
            for lowercased_letter in letter.to_lowercase() {
                let counter = ana_letter_count.entry(lowercased_letter).or_insert(0);
                *counter += 1;
            }
        }

        for (curr_letter, curr_count) in ana_letter_count {
            match letter_count.get(&curr_letter) {
                Some(original_count) => {
                    if *original_count != curr_count {
                        continue 'possible_word_loop;
                    }
                }
                None => continue 'possible_word_loop,
            }
        }

        matches.insert(*curr_word);
    }

    matches
}

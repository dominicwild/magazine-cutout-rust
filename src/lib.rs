// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_word_counts = word_counts_of(magazine);

    for &word in note {
        match magazine_word_counts.get(word) {
            None | Some(0) => return false,
            Some(count) => magazine_word_counts.insert(word, count - 1),
        };
    }

    true
}

fn word_counts_of<'a>(magazine: &[&'a str]) -> HashMap<&'a str, i32> {
    let mut word_counts = HashMap::new();

    for &word in magazine {
        match word_counts.get(word) {
            Some(word_count) => word_counts.insert(word, word_count + 1),
            None => word_counts.insert(word, 1),
        };
    }

    word_counts
}

// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_word_counts = word_counts_of(magazine);

    for &word in note {
        match magazine_word_counts.entry(word).or_insert(0) {
            0 => return false,
            count => *count -= 1,
        };
    }

    true
}

fn word_counts_of<'a>(magazine: &[&'a str]) -> HashMap<&'a str, i32> {
    let mut word_counts = HashMap::new();

    for &word in magazine {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    word_counts
}

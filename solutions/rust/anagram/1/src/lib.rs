use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_chars = word.chars().flat_map(|ch| ch.to_lowercase()).collect::<Vec<_>>();
    let mut word_chars_sorted = word_chars.clone();
    word_chars_sorted.sort_unstable();

    possible_anagrams.iter().filter(|candidate| {
        let mut chars = candidate.chars().flat_map(|ch| ch.to_lowercase()).collect::<Vec<_>>();
        if word_chars == chars {
            return false;
        }
        chars.sort_unstable();
        chars == word_chars_sorted
    }).copied().collect()
}

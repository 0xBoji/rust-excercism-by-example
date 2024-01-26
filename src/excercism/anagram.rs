use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let normalized_word = normalize(word);
    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let normalized_candidate = normalize(candidate);
            normalized_word == normalized_candidate && word.to_lowercase() != candidate.to_lowercase()
        })
        .copied()
        .collect()
}

fn normalize(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars
}

pub fn run() {
    let word = "stone";
    let candidates = ["stone", "tones", "banana","cmm", "tons", "notes", "Seton"];
    let anagrams = anagrams_for(word, &candidates);

    println!("Anagrams of '{}': {:?}", word, anagrams);
}

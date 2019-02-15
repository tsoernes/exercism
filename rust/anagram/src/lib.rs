use std::collections::HashMap;
use std::collections::HashSet;

fn word_to_freqmap(word: &str) -> HashMap<char, u32> {
    let mut map: HashMap<char, u32> = HashMap::with_capacity(word.len());
    for ch in word.to_lowercase().chars() {
        if let Some(freq) = map.get_mut(&ch) {
            *freq += 1;
        } else {
            map.insert(ch, 1);
        }
    }
    map
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let mut anagrams: HashSet<&str> = HashSet::new();
    let target_map = word_to_freqmap(word);
    for wrd in possible_anagrams.iter() {
        if *wrd.to_lowercase() != lower_word {
            let word_map = word_to_freqmap(wrd);
            if word_map == target_map {
                anagrams.insert(wrd);
            }
        }
    }
    anagrams
}

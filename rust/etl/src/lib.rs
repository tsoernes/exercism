use std::collections::BTreeMap;
use std::iter::FromIterator;

pub fn transform(score_to_letters: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    BTreeMap::from_iter(score_to_letters.iter().flat_map(|(&score, letters)| {
        letters
            .iter()
            .map(move |&letter| (letter.to_ascii_lowercase(), score))
    }))
}

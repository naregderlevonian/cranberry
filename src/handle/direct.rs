use crate::alias::Alphabet;

pub fn translate(source: &String, alphabet: &Alphabet) -> String {
    source
        .chars()
        .map(|c| alphabet.get(&c).unwrap_or(&c.to_string()).to_string())
        .collect()
}

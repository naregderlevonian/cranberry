use crate::alias::Alphabet;
use crate::alias::Latin;

pub fn get() -> Alphabet {
    let mut alphabet = Alphabet::new();

    alphabet.insert('Е', Latin::from("Je"));
    alphabet.insert('е', Latin::from("je"));

    alphabet.insert('Ё', Latin::from("Jo"));
    alphabet.insert('ё', Latin::from("jo"));

    alphabet.insert('Ю', Latin::from("Ju"));
    alphabet.insert('ю', Latin::from("ju"));

    alphabet.insert('Я', Latin::from("Ja"));
    alphabet.insert('я', Latin::from("ja"));

    alphabet
}

use crate::alias::Alphabet;
use crate::alias::Latin;

pub fn get() -> Alphabet {
    let mut alphabet = Alphabet::new();

    alphabet.insert('Е', Latin::from("E"));
    alphabet.insert('е', Latin::from("e"));

    alphabet.insert('Ё', Latin::from("O"));
    alphabet.insert('ё', Latin::from("o"));

    alphabet.insert('Ю', Latin::from("U"));
    alphabet.insert('ю', Latin::from("u"));

    alphabet.insert('Я', Latin::from("A"));
    alphabet.insert('я', Latin::from("a"));

    alphabet
}

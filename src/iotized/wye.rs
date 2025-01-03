use crate::alias::Alphabet;
use crate::alias::Latin;

pub fn get() -> Alphabet {
    let mut alphabet = Alphabet::new();

    alphabet.insert('Е', Latin::from("Ye"));
    alphabet.insert('е', Latin::from("ye"));

    alphabet.insert('Ё', Latin::from("Yo"));
    alphabet.insert('ё', Latin::from("yo"));

    alphabet.insert('Ю', Latin::from("Yu"));
    alphabet.insert('ю', Latin::from("yu"));

    alphabet.insert('Я', Latin::from("Ya"));
    alphabet.insert('я', Latin::from("ya"));

    alphabet
} 

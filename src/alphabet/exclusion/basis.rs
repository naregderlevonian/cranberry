use crate::alias::Alphabet;
use crate::alphabetize;

pub fn get() -> Alphabet {
    alphabetize! {
        'Е' => "E",
        'е' => "e",

        'Ё' => "O",
        'ё' => "o",

        'Ю' => "U",
        'ю' => "u",

        'Я' => "A",
        'я' => "a",
    }
}

use crate::alias::Alphabet;
use crate::alphabetize;

pub fn get() -> Alphabet {
    alphabetize! {
        'Е' => "Ye",
        'е' => "ye",

        'Ё' => "Yo",
        'ё' => "yo",

        'Ю' => "Yu",
        'ю' => "yu",

        'Я' => "Ya",
        'я' => "ya",
    }
} 

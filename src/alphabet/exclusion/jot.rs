use crate::alias::Alphabet;
use crate::alphabetize;

pub fn get() -> Alphabet {
    alphabetize! {
        'Е' => "Je",
        'е' => "je",

        'Ё' => "Jo",
        'ё' => "jo",

        'Ю' => "Ju",
        'ю' => "ju",

        'Я' => "Ja",
        'я' => "ja",
    }
}

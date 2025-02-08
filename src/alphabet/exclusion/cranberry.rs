use crate::alias::Alphabet;
use crate::alphabetize;

pub fn get() -> Alphabet {
    alphabetize! {
        'Е' => "Je",
        'е' => "ȷe",

        'Ё' => "Jo",
        'ё' => "ȷo",

        'Ю' => "Ju",
        'ю' => "ȷu",

        'Я' => "Ja",
        'я' => "ȷa",
    }
}

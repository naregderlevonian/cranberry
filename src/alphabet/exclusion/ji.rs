use crate::alias::Alphabet;
use crate::alphabetize;

pub fn get() -> Alphabet {
    alphabetize! {
        'И' => "Ji",
        'и' => "ji",
    }
}

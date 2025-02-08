#[macro_export]
macro_rules! alphabetize {
    ($($cyr:literal => $lat:expr),* $(,)?) => {{
        use crate::alias::Alphabet;
        let mut alphabet = Alphabet::new();
        $(alphabet.insert($cyr, $lat.into());)*
        alphabet
    }};
}

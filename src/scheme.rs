use crate::alias::Alphabet;

pub struct Scheme {
    alphabet: Alphabet,
    handle: fn(&String, &Alphabet) -> String,
}

impl Scheme {
    pub fn create(alphabet: Alphabet, handle: fn(&String, &Alphabet) -> String) -> Self {
        Self { alphabet, handle }
    }

    pub fn process(&self, source: &String) -> String {
        (self.handle)(&source, &self.alphabet)
    }
}

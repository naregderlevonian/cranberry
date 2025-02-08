use crate::alias::Alphabet;
use crate::alias::Cyrillic;
use crate::alias::Latin;

enum Handler {
    Basic,
    Smart(Box<dyn Fn(&Alphabet, &Cyrillic, Option<&Cyrillic>, Option<&Cyrillic>) -> Latin>),
}

pub struct Engine {
    alphabet: Alphabet,
    handler: Handler,
}

impl Engine {
    pub fn basic(alphabet: Alphabet) -> Self {
        Engine {
            alphabet,
            handler: Handler::Basic,
        }
    }

    pub fn smart<F>(alphabet: Alphabet, process: F) -> Self
    where F: Fn(&Alphabet, &Cyrillic, Option<&Cyrillic>, Option<&Cyrillic>) -> Latin + 'static,
    {
        Engine {
            alphabet,
            handler: Handler::Smart(Box::new(process)),
        }
    }

    pub fn process(&self, source: &String) -> Latin {
        match &self.handler {
            Handler::Basic => self.transliterate(source),
            Handler::Smart(process) => self.latinize(source, process.as_ref()),
        }
    }

    fn transliterate(&self, source: &String) -> Latin {
        source
            .chars()
            .map(|c| self.alphabet.get(&c).unwrap_or(&c.to_string()).to_string())
            .collect()
    }

    fn latinize(&self, source: &String, process: &dyn Fn(&Alphabet, &Cyrillic, Option<&Cyrillic>, Option<&Cyrillic>) -> Latin) -> Latin {
        let chars: Vec<Cyrillic> = source.chars().collect();
        let mut result = String::new();

        for i in 0..chars.len() {
            let current = &chars[i];
            let prev = if i > 0 { Some(&chars[i - 1]) } else { None };
            let next = if i < chars.len() - 1 {
                Some(&chars[i + 1])
            } else {
                None
            };

            let translated = process(&self.alphabet, current, prev, next);
            result.push_str(&translated);
        }

        result
    }
}




use std::collections::HashMap;

pub struct Map {
    data: HashMap<char, String>,
    function: Box<dyn Fn(HashMap<char, String>, char, Option<char>, Option<char>) -> String>,
}

impl Map {
    pub fn new(
        data: HashMap<char, String>,
        function: Box<dyn Fn(HashMap<char, String>, char, Option<char>, Option<char>) -> String>,
    ) -> Self {
        Map { data, function }
    }

    pub fn get(&self) -> HashMap<char, String> {
        self.data.clone()
    }

    pub fn insert(&mut self, cyr: char, lat: String) {
        self.data.insert(cyr, lat);
    }

    pub fn process(&self, d: HashMap<char, String>, c: char, p: Option<char>, n: Option<char>) -> String {
        (self.function)(d, c, p, n)
    }
}

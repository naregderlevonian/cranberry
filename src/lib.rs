mod dictionary;
mod function;
mod map;
mod translate;

pub mod scheme;

use crate::map::Map;

pub fn translate(input: String, map: Map) -> String {
    let mut output = String::new();
    let chars: Vec<char> = input.chars().collect();

    for i in 0..chars.len() {
        let current = chars[i];

        let prev = if i > 0 { Some(chars[i - 1]) } else { None };
        let next = if i < chars.len() - 1 { Some(chars[i + 1]) } else { None };

        output.push_str(&map.process(map.get(), current, prev, next));
    }

    output
}



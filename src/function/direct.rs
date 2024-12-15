use std::collections::HashMap;

pub fn parse(data: HashMap<char, String>, current: char, _prev: Option<char>, _next: Option<char>) -> String {
    match data.get(&current) {
        Some(converted) => {
            converted.clone()
        },
        None => current.to_string(),
    }
}

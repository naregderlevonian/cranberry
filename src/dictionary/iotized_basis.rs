use std::collections::HashMap;

pub fn new() -> HashMap<char, String> {
    let mut map: HashMap<char, String> = HashMap::new();

    map.insert('Е', String::from("E"));
    map.insert('е', String::from("e"));

    map.insert('Ё', String::from("O"));
    map.insert('ё', String::from("o"));

    map.insert('Ю', String::from("U"));
    map.insert('ю', String::from("u"));

    map.insert('Я', String::from("A"));
    map.insert('я', String::from("a"));

    map
}

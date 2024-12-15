use std::collections::HashMap;

pub fn new() -> HashMap<char, String> {
    let mut map: HashMap<char, String> = HashMap::new();

    map.insert('Е', String::from("Je"));
    map.insert('е', String::from("je"));

    map.insert('Ё', String::from("Jo"));
    map.insert('ё', String::from("jo"));

    map.insert('Ю', String::from("Ju"));
    map.insert('ю', String::from("ju"));

    map.insert('Я', String::from("Ja"));
    map.insert('я', String::from("ja"));

    map
}

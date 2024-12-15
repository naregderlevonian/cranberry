use crate::dictionary;
use crate::function::direct;
use crate::map::Map;

pub fn new() -> Map {
    let mut dictionary = dictionary::alphabet::new();

    dictionary.insert('Е', String::from("E"));
    dictionary.insert('е', String::from("e"));

    dictionary.insert('Ё', String::from("Ё"));
    dictionary.insert('ё', String::from("ё"));

    dictionary.insert('Ж', String::from("Ž"));
    dictionary.insert('ж', String::from("ž"));

    dictionary.insert('Й', String::from("J"));
    dictionary.insert('й', String::from("j"));

    dictionary.insert('Х', String::from("H"));
    dictionary.insert('х', String::from("h"));

    dictionary.insert('Ц', String::from("C"));
    dictionary.insert('ц', String::from("c"));
    
    dictionary.insert('Ч', String::from("Č"));
    dictionary.insert('ч', String::from("č"));
    
    dictionary.insert('Ш', String::from("Š"));
    dictionary.insert('ш', String::from("š"));
    
    dictionary.insert('Щ', String::from("Ŝ"));
    dictionary.insert('щ', String::from("ŝ"));
    
    dictionary.insert('ъ', String::from("\""));
    dictionary.insert('Ъ', String::from("\""));

    dictionary.insert('Ы', String::from("Y"));
    dictionary.insert('ы', String::from("y"));

    dictionary.insert('Ь', String::from("\'"));
    dictionary.insert('ь', String::from("\'"));
    
    dictionary.insert('Э', String::from("È"));
    dictionary.insert('э', String::from("è"));

    dictionary.insert('Ю', String::from("Û"));
    dictionary.insert('ю', String::from("û"));
    
    dictionary.insert('Я', String::from("Â"));
    dictionary.insert('я', String::from("â"));

    Map::new(dictionary, Box::new(direct::parse))
}

use crate::dictionary;
use crate::function::jakovlev;
use crate::map::Map;

pub fn new() -> Map {
    let mut dictionary = dictionary::alphabet::new();

    dictionary.insert('Е', String::from("E"));
    dictionary.insert('е', String::from("e"));

    dictionary.insert('Ё', String::from("Ö"));
    dictionary.insert('ё', String::from("ö"));

    dictionary.insert('Ж', String::from("Ƶ"));
    dictionary.insert('ж', String::from("ƶ"));

    dictionary.insert('Й', String::from("J"));
    dictionary.insert('й', String::from("j"));

    dictionary.insert('Х', String::from("X"));
    dictionary.insert('х', String::from("x"));

    dictionary.insert('Ц', String::from("Ç"));
    dictionary.insert('ц', String::from("ç"));
    
    dictionary.insert('Ч', String::from("C"));
    dictionary.insert('ч', String::from("c"));
    
    dictionary.insert('Ш', String::from("Ş"));
    dictionary.insert('ш', String::from("ş"));
    
    dictionary.insert('Щ', String::from("Sc"));
    dictionary.insert('щ', String::from("sc"));
    
    dictionary.insert('ъ', String::from(""));
    dictionary.insert('Ъ', String::from(""));
    
    dictionary.insert('Ы', String::from("Y"));
    dictionary.insert('ы', String::from("y"));
    
    dictionary.insert('Ь', String::from("J"));
    dictionary.insert('ь', String::from("j"));
    
    dictionary.insert('Ю', String::from("Ü"));
    dictionary.insert('ю', String::from("ü"));
    
    dictionary.insert('Я', String::from("Ä"));
    dictionary.insert('я', String::from("ä"));

    Map::new(dictionary, Box::new(jakovlev::parse))
}

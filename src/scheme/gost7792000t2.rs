use crate::dictionary;
use crate::function::direct;
use crate::map::Map;

pub fn new() -> Map {
    let mut dictionary = dictionary::alphabet::new();

    dictionary.insert('Е', String::from("E"));
    dictionary.insert('е', String::from("e"));

    dictionary.insert('Ё', String::from("Yo"));
    dictionary.insert('ё', String::from("yo"));

    dictionary.insert('Ж', String::from("Zh"));
    dictionary.insert('ж', String::from("zh"));

    dictionary.insert('Й', String::from("J"));
    dictionary.insert('й', String::from("j"));

    dictionary.insert('Х', String::from("Kh"));
    dictionary.insert('х', String::from("kh"));

    dictionary.insert('Ц', String::from("C")); // ?? cz
    dictionary.insert('ц', String::from("c"));
    
    dictionary.insert('Ч', String::from("Ch"));
    dictionary.insert('ч', String::from("ch"));
    
    dictionary.insert('Ш', String::from("Sh"));
    dictionary.insert('ш', String::from("sh"));
    
    dictionary.insert('Щ', String::from("Shh"));
    dictionary.insert('щ', String::from("shh"));
    
    dictionary.insert('ъ', String::from("\""));
    dictionary.insert('Ъ', String::from("\""));

    dictionary.insert('Ы', String::from("Y"));
    dictionary.insert('ы', String::from("y"));

    dictionary.insert('Ь', String::from("\'"));
    dictionary.insert('ь', String::from("\'"));
    
    dictionary.insert('Э', String::from("E'"));
    dictionary.insert('э', String::from("e'"));

    dictionary.insert('Ю', String::from("Yu"));
    dictionary.insert('ю', String::from("yu"));
    
    dictionary.insert('Я', String::from("Ya"));
    dictionary.insert('я', String::from("ya"));

    Map::new(dictionary, Box::new(direct::parse))
}

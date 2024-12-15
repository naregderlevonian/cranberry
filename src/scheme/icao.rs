use crate::dictionary;
use crate::function::direct;
use crate::map::Map;

pub fn new() -> Map {
    let mut dictionary = dictionary::alphabet::new();

    dictionary.insert('Е', String::from("E"));
    dictionary.insert('е', String::from("e"));

    dictionary.insert('Ё', String::from("E"));
    dictionary.insert('ё', String::from("e"));

    dictionary.insert('Ж', String::from("Zh"));
    dictionary.insert('ж', String::from("zh"));

    dictionary.insert('Й', String::from("I"));
    dictionary.insert('й', String::from("i"));

    dictionary.insert('Х', String::from("Kh"));
    dictionary.insert('х', String::from("kh"));

    dictionary.insert('Ц', String::from("Ts"));
    dictionary.insert('ц', String::from("ts"));
    
    dictionary.insert('Ч', String::from("Ch"));
    dictionary.insert('ч', String::from("ch"));
    
    dictionary.insert('Ш', String::from("Sh"));
    dictionary.insert('ш', String::from("sh"));
    
    dictionary.insert('Щ', String::from("Shch"));
    dictionary.insert('щ', String::from("shch"));

    dictionary.insert('ъ', String::from("Ie"));
    dictionary.insert('Ъ', String::from("ie"));

    dictionary.insert('Ы', String::from("Y"));
    dictionary.insert('ы', String::from("y"));

    dictionary.insert('Ь', String::from(""));
    dictionary.insert('ь', String::from(""));
    
    dictionary.insert('Э', String::from("E"));
    dictionary.insert('э', String::from("e"));

    dictionary.insert('Ю', String::from("Iu"));
    dictionary.insert('ю', String::from("iu"));
    
    dictionary.insert('Я', String::from("Ia"));
    dictionary.insert('я', String::from("ia"));

    Map::new(dictionary, Box::new(direct::parse))
}

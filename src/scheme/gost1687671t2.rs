use crate::dictionary;
use crate::function::direct;
use crate::map::Map;

pub fn new() -> Map {
    let mut dictionary = dictionary::alphabet::new();

    dictionary.insert('Е', String::from("Je"));
    dictionary.insert('е', String::from("je"));

    dictionary.insert('Ё', String::from("Jo"));
    dictionary.insert('ё', String::from("jo"));

    dictionary.insert('Ж', String::from("Zh"));
    dictionary.insert('ж', String::from("zh"));

    dictionary.insert('Й', String::from("J"));
    dictionary.insert('й', String::from("j"));

    dictionary.insert('Х', String::from("Kh"));
    dictionary.insert('х', String::from("kh"));

    dictionary.insert('Ц', String::from("C"));
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
    
    dictionary.insert('Э', String::from("Eh"));
    dictionary.insert('э', String::from("eh"));

    dictionary.insert('Ю', String::from("Ju"));
    dictionary.insert('ю', String::from("ju"));
    
    dictionary.insert('Я', String::from("Ja"));
    dictionary.insert('я', String::from("ja"));

    Map::new(dictionary, Box::new(direct::parse))
}

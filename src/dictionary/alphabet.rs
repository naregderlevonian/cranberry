use std::collections::HashMap;

pub fn new() -> HashMap<char, String> {
    let mut map: HashMap<char, String> = HashMap::new();

    map.insert('А', String::from("A"));
    map.insert('а', String::from("a"));

    map.insert('Б', String::from("B"));
    map.insert('б', String::from("b"));

    map.insert('В', String::from("V"));
    map.insert('в', String::from("v"));

    map.insert('Г', String::from("G"));
    map.insert('г', String::from("g"));

    map.insert('Д', String::from("D"));
    map.insert('д', String::from("d"));

    map.insert('Е', String::from("Ė"));
    map.insert('е', String::from("ė"));

    map.insert('Ё', String::from("Ȯ"));
    map.insert('ё', String::from("ȯ"));

    map.insert('Ж', String::from("Zh"));
    map.insert('ж', String::from("zh"));

    map.insert('З', String::from("Z"));
    map.insert('з', String::from("z"));

    map.insert('И', String::from("I"));
    map.insert('и', String::from("i"));

    map.insert('Й', String::from("J"));
    map.insert('й', String::from("j"));

    map.insert('К', String::from("K"));
    map.insert('к', String::from("k"));

    map.insert('Л', String::from("L"));
    map.insert('л', String::from("l"));

    map.insert('М', String::from("M"));
    map.insert('м', String::from("m"));

    map.insert('Н', String::from("N"));
    map.insert('н', String::from("n"));

    map.insert('О', String::from("O"));
    map.insert('о', String::from("o"));

    map.insert('П', String::from("P"));
    map.insert('п', String::from("p"));

    map.insert('Р', String::from("R"));
    map.insert('р', String::from("r"));

    map.insert('С', String::from("S"));
    map.insert('с', String::from("s"));

    map.insert('Т', String::from("T"));
    map.insert('т', String::from("t"));

    map.insert('У', String::from("U"));
    map.insert('у', String::from("u"));

    map.insert('Ф', String::from("F"));
    map.insert('ф', String::from("f"));

    map.insert('Х', String::from("X"));
    map.insert('х', String::from("x"));

    map.insert('Ц', String::from("C"));
    map.insert('ц', String::from("c"));

    map.insert('Ч', String::from("Ch"));
    map.insert('ч', String::from("ch"));

    map.insert('Ш', String::from("Sh"));
    map.insert('ш', String::from("sh"));

    map.insert('Щ', String::from("Sch"));
    map.insert('щ', String::from("sch"));

    map.insert('Ъ', String::from(""));
    map.insert('ъ', String::from(""));

    map.insert('Ы', String::from("Y"));
    map.insert('ы', String::from("y"));

    map.insert('Ь', String::from("I"));
    map.insert('ь', String::from("ı"));

    map.insert('Э', String::from("E"));
    map.insert('э', String::from("e"));

    map.insert('Ю', String::from("U̇"));
    map.insert('ю', String::from("u̇"));

    map.insert('Я', String::from("Ȧ"));
    map.insert('я', String::from("ȧ"));

    map
}

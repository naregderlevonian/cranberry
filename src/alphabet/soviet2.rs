use crate::alias::Alphabet;
use crate::alias::Latin;

pub fn get() -> Alphabet {
    let mut alphabet = Alphabet::new();

    alphabet.insert('А', Latin::from("A"));
    alphabet.insert('а', Latin::from("a"));

    alphabet.insert('Б', Latin::from("B"));
    alphabet.insert('б', Latin::from("b"));

    alphabet.insert('В', Latin::from("V"));
    alphabet.insert('в', Latin::from("v"));

    alphabet.insert('Г', Latin::from("G"));
    alphabet.insert('г', Latin::from("g"));

    alphabet.insert('Д', Latin::from("D"));
    alphabet.insert('д', Latin::from("d"));

    alphabet.insert('Е', Latin::from("E"));
    alphabet.insert('е', Latin::from("e"));

    alphabet.insert('Ё', Latin::from("Ö"));
    alphabet.insert('ё', Latin::from("ö"));

    alphabet.insert('Ж', Latin::from("Ƶ"));
    alphabet.insert('ж', Latin::from("ƶ"));

    alphabet.insert('З', Latin::from("Z"));
    alphabet.insert('з', Latin::from("z"));

    alphabet.insert('И', Latin::from("I"));
    alphabet.insert('и', Latin::from("i"));

    alphabet.insert('Й', Latin::from("J"));
    alphabet.insert('й', Latin::from("j"));

    alphabet.insert('К', Latin::from("K"));
    alphabet.insert('к', Latin::from("k"));

    alphabet.insert('Л', Latin::from("L"));
    alphabet.insert('л', Latin::from("l"));

    alphabet.insert('М', Latin::from("M"));
    alphabet.insert('м', Latin::from("m"));

    alphabet.insert('Н', Latin::from("N"));
    alphabet.insert('н', Latin::from("n"));

    alphabet.insert('О', Latin::from("O"));
    alphabet.insert('о', Latin::from("o"));

    alphabet.insert('П', Latin::from("P"));
    alphabet.insert('п', Latin::from("p"));

    alphabet.insert('Р', Latin::from("R"));
    alphabet.insert('р', Latin::from("r"));

    alphabet.insert('С', Latin::from("S"));
    alphabet.insert('с', Latin::from("s"));

    alphabet.insert('Т', Latin::from("T"));
    alphabet.insert('т', Latin::from("t"));

    alphabet.insert('У', Latin::from("U"));
    alphabet.insert('у', Latin::from("u"));

    alphabet.insert('Ф', Latin::from("F"));
    alphabet.insert('ф', Latin::from("f"));

    alphabet.insert('Х', Latin::from("X"));
    alphabet.insert('х', Latin::from("x"));

    alphabet.insert('Ц', Latin::from("Ç"));
    alphabet.insert('ц', Latin::from("ç"));

    alphabet.insert('Ч', Latin::from("C"));
    alphabet.insert('ч', Latin::from("c"));

    alphabet.insert('Ш', Latin::from("Ş"));
    alphabet.insert('ш', Latin::from("ş"));

    alphabet.insert('Щ', Latin::from("Sc"));
    alphabet.insert('щ', Latin::from("sc"));

    alphabet.insert('Ъ', Latin::from(""));
    alphabet.insert('ъ', Latin::from(""));

    alphabet.insert('Ы', Latin::from("Y"));
    alphabet.insert('ы', Latin::from("y"));

    alphabet.insert('Ь', Latin::from("J"));
    alphabet.insert('ь', Latin::from("j"));

    alphabet.insert('Э', Latin::from("E"));
    alphabet.insert('э', Latin::from("e"));

    alphabet.insert('Ю', Latin::from("Ü"));
    alphabet.insert('ю', Latin::from("ü"));

    alphabet.insert('Я', Latin::from("Ä"));
    alphabet.insert('я', Latin::from("ä"));

    alphabet
}

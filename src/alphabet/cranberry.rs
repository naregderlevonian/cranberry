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

    alphabet.insert('Е', Latin::from("Ė"));
    alphabet.insert('е', Latin::from("ė"));

    alphabet.insert('Ё', Latin::from("Ȯ"));
    alphabet.insert('ё', Latin::from("ȯ"));

    alphabet.insert('Ж', Latin::from("Ż"));
    alphabet.insert('ж', Latin::from("ż"));

    alphabet.insert('З', Latin::from("Z"));
    alphabet.insert('з', Latin::from("z"));

    alphabet.insert('И', Latin::from("İ"));
    alphabet.insert('и', Latin::from("i"));

    alphabet.insert('Й', Latin::from("J"));
    alphabet.insert('й', Latin::from("ȷ"));

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

    alphabet.insert('Ц', Latin::from("C"));
    alphabet.insert('ц', Latin::from("c"));

    alphabet.insert('Ч', Latin::from("Ċ"));
    alphabet.insert('ч', Latin::from("ċ"));

    alphabet.insert('Ш', Latin::from("Ṡ"));
    alphabet.insert('ш', Latin::from("ṡ"));

    alphabet.insert('Щ', Latin::from("S̈"));
    alphabet.insert('щ', Latin::from("s̈"));

    alphabet.insert('Ъ', Latin::from(""));
    alphabet.insert('ъ', Latin::from(""));

    alphabet.insert('Ы', Latin::from("I"));
    alphabet.insert('ы', Latin::from("ı"));

    alphabet.insert('Ь', Latin::from("J"));
    alphabet.insert('ь', Latin::from("ȷ"));

    alphabet.insert('Э', Latin::from("E"));
    alphabet.insert('э', Latin::from("e"));

    alphabet.insert('Ю', Latin::from("U̇"));
    alphabet.insert('ю', Latin::from("u̇"));

    alphabet.insert('Я', Latin::from("Ȧ"));
    alphabet.insert('я', Latin::from("ȧ"));

    alphabet
}

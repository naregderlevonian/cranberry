
#[test]
fn pangram() {
    let expected = "V čaŝah ûga žil by citrus? Da, no fal'šivyj èkzemplǎr!";

    let source = "В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!";
    let fact = cranberry::Engine::ISO1995.init().process(&source.to_string());

    assert_eq!(expected, fact);
}

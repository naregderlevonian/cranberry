
#[test]
fn pangram() {
    let expected = "V čaščach juga žil by citrus? Da, no fal'šivyj èkzempljar!";

    let source = "В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!";
    let fact = cranberry::Engine::ISO1968Alt1.init().process(&source.to_string());

    assert_eq!(expected, fact);
}

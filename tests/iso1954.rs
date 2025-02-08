
#[test]
fn pangram() {
    let expected = "V čaščah juga žil by citrus? Da, no fal'šivyj èkzempljar!";

    let source = "В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!";
    let fact = cranberry::Scheme::ISO1954.init().process(&source.to_string());

    assert_eq!(expected, fact);
}

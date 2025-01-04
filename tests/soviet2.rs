
#[test]
fn pangram() {
    let expected = "V cascax juga ƶil by çitrus? Da, no faljşivyj ekzemplär!";

    let source = "В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!";
    let fact = cranberry::engine::Engine::Soviet2.init().process(&source.to_string());

    assert_eq!(expected, fact);
}


#[test]
fn pangram() {
    let expected = "V cascax juga ƶil bь çitrus? Da, no faljşivьj ekzemplər!";

    let source = "В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!";
    let fact = cranberry::engine::Engine::Soviet3.init().process(&source.to_string());

    assert_eq!(expected, fact);
}

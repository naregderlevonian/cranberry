
#[test]
fn pangram() {
    let expected = "V ċas̈ax ȷuga żil bı citrus? Da, no falȷṡivıȷ ekzėmplȧr!";

    let source = "В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!";
    let fact = cranberry::engine::Engine::Cranberry.init().process(&source.to_string());

    assert_eq!(expected, fact);
}

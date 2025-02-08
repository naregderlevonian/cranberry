
#[test]
fn pangram() {
    let expected = "V chashchakh yuga zhil by tsitrus? Da, no fal'shivyĭ èkzemplyar!";

    let source = "В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!";
    let fact = cranberry::Scheme::ISO1968Alt2.init().process(&source.to_string());

    assert_eq!(expected, fact);
}

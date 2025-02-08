
#[test]
fn pangram() {
    let expected = "V chashchakh i͡uga zhil by t͡sitrus? Da, no fal'shivyĭ ėkzempli͡ar!";

    let source = "В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!";
    let fact = cranberry::Scheme::ALALC.init().process(&source.to_string());

    assert_eq!(expected, fact);
}

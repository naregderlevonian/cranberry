**Cranberry** is a versatile Rust library designed specifically for the transliteration of Russian Cyrillic characters into Latin script. While various romanization standards exist, none have achieved universal acceptance, leading to inconsistency across different applications.

Future updates to Cranberry are expected to expand its capabilities to include all existing romanization schemes, providing users with a comprehensive tool for accurate and consistent transliteration. The library emphasizes flexibility and ease of use, making it suitable for both academic and practical applications.

## Installation

You can add Cranberry to your Rust project by including the following line in your `Cargo.toml` file:

```toml
[dependencies]
cranberry = "0.4.0"
```

Alternatively, you can use the command line to add it to your project:

```bash
cargo add cranberry
```

## Usage

Here’s a simple example of how to use the Cranberry library for transliterating text:

```rust
use cranberry;

fn main() {
    let expected = "V ċas̈ax ȷuga żil bı citrus? Da, no falȷṡivıȷ ekzėmplȧr!";

    let source = "В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!";
    let fact = cranberry::Engine::Cranberry.init().process(&source.to_string());

    assert_eq!(expected, fact);
}
```

## Supported Engines

The library currently supports nine transliteration engines, including the Cranberry engine (developed by the author), three variants based on the Soviet Latin alphabet proposed by Professor Yakovlev’s commission during the USSR era, and five international standard systems compliant with ISO 9.

Here is a table with all the engines and an example of the transformed pangram text. 

| Engine | Scheme | Pangram |
|-|-|-|
| ```cranberry::Engine::Cranberry``` | **Cranberry** | V ċas̈ax ȷuga żil bı citrus? Da, no falȷṡivıȷ ekzėmplȧr! |
| ```cranberry::Engine::Soviet1``` | **Soviet #1** | V cascax juga ƶil by çitrus? Da, no falíşivyj ekzemplár! |
| ```cranberry::Engine::Soviet2``` | **Soviet #2** | V cascax juga ƶil by çitrus? Da, no faljşivyj ekzemplär! |
| ```cranberry::Engine::Soviet3``` | **Soviet #3** | V cascax juga ƶil bь çitrus? Da, no faljşivьj ekzemplər! |
| ```cranberry::Engine::ISO1954``` | **ISO 9 1954** | V čaščah juga žil by citrus? Da, no fal'šivyj èkzempljar! |
| ```cranberry::Engine::ISO1968Base``` | **ISO 9 1968 Base** | V čaŝah ûga žil by citrus? Da, no fal'šivyj èkzemplǎr! |
| ```cranberry::Engine::ISO1968Alt1``` | **ISO 9 1968 Alternative #1** | V čaščach juga žil by citrus? Da, no fal'šivyj èkzempljar! |
| ```cranberry::Engine::ISO1968Alt2``` | **ISO 9 1968 Alternative #2** | V chashchakh yuga zhil by tsitrus? Da, no fal'shivyĭ èkzemplyar! |
| ```cranberry::Engine::ISO1995``` | **ISO 9 1995** | V čaŝah ûga žil by citrus? Da, no fal'šivyj èkzemplǎr! |

## License

This project is licensed under the [MIT License](LICENSE).

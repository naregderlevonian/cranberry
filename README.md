**Cranberry** is a versatile Rust library for transliterating Russian Cyrillic text to Latin script, addressing the challenge of inconsistent romanization standards across different applications. The library provides multiple transliteration schemes while maintaining exceptional performance and ease of integration.

[License: MIT](LICENSE)
[Crates.io](https://crates.io/crates/cranberry)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cranberry = "0.5.0"
```

Or use cargo:

```bash
cargo add cranberry
```

## Usage

```rust
use cranberry;

fn main() {
    let expected = "V ċas̈ax ȷuga żil bı citrus? Da, no falȷṡivıȷ ekzėmplȧr!";

    let source = "В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!";
    let fact = cranberry::Scheme::Cranberry.init().process(&source.to_string());

    assert_eq!(expected, fact);
}
```

## Engines

Cranberry currently supports the following transliteration engines:

| Engine | Scheme | Pangram |
|-|-|-|
| ```cranberry::Scheme::Cranberry``` | **Cranberry** | V ċas̈ax ȷuga żil bı citrus? Da, no falȷṡivıȷ ekzėmplȧr! |
| ```cranberry::Scheme::Soviet1``` | **Soviet #1** | V cascax juga ƶil by çitrus? Da, no falíşivyj ekzemplár! |
| ```cranberry::Scheme::Soviet2``` | **Soviet #2** | V cascax juga ƶil by çitrus? Da, no faljşivyj ekzemplär! |
| ```cranberry::Scheme::Soviet3``` | **Soviet #3** | V cascax juga ƶil bь çitrus? Da, no faljşivьj ekzemplər! |
| ```cranberry::Scheme::ISO1954``` | **ISO 9 1954** | V čaščah juga žil by citrus? Da, no fal'šivyj èkzempljar! |
| ```cranberry::Scheme::ISO1968Base``` | **ISO 9 1968 Base** | V čaŝah ûga žil by citrus? Da, no fal'šivyj èkzemplǎr! |
| ```cranberry::Scheme::ISO1968Alt1``` | **ISO 9 1968 Alternative #1** | V čaščach juga žil by citrus? Da, no fal'šivyj èkzempljar! |
| ```cranberry::Scheme::ISO1968Alt2``` | **ISO 9 1968 Alternative #2** | V chashchakh yuga zhil by tsitrus? Da, no fal'shivyĭ èkzemplyar! |
| ```cranberry::Scheme::ISO1995``` | **ISO 9 1995** | V čaŝah ûga žil by citrus? Da, no fal'šivyj èkzemplǎr! |
| ```cranberry::Scheme::ALALC``` | **ALA-LC** | V chashchakh i͡uga zhil by t͡sitrus? Da, no fal'shivyĭ ėkzempli͡ar! |

## Roadmap

- **Enginges**
    * Scientific 1905 (1925)
    * Scientific 1939
    * Scientific 1951-57
    * OST 8483
    * GOST 168876-71
    * ST SEV 1362
    * ISO 9:1995, GOST 7.79-2000
    * GOST 52290-2004
    * GOST 52535.1-2006
    * UNGEGN
    * BS 2979:1958
    * BGN/PCGN
    * Passport 1997
    * Passport 2010
    * Passport 2013 (ICAO)

May be:

- **Bidirectional Support**. Latin-to-Cyrillic conversion capabilities.
- **Custom Schemas**. User-defined mapping configurations.
- **Standalone Application**. Command-line tool, WebAssembly playground or GTK+ GUI.

## License

This project is licensed under the [MIT License](LICENSE).

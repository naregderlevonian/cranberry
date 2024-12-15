**Cranberry** is a Rust library for transliterating Russian Cyrillic characters into Latin script.

## Features

- Converts Russian text from Cyrillic to Latin script based on a set of defined rules.
- Supports multiple mapping schemes for different transliteration standards.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
cranberry = "0.1.1"
```

or you can add a library to your project with the command:

```bash
cargo add cranberry
```

## Usage

Here’s a basic example of how to use the library for transliterating text:

```rust
use cranberry::translate;
use cranberry::scheme::core;

fn main() {
    let text = String::from("В чащах юга жил бы цитрус? Да, но фальшивый экземпляр!");
    let result = translate::from(text, scheme::core::new());
    println!("{}", result);
}
```

Result:

```
V chaschax juga zhil by citrus? Da, no falıshivyj ekzėmplȧr!
```

## Supported Schemes

| Scheme | Code | Descriptoion |
|--------------------|--------------------|--------------------|
| Default | ```cranberry::scheme::core``` | An author's system. |
| Yakovlev #1 | ```cranberry::scheme::jakovlev1``` | Latinisation in the Soviet Union #1. |
| Yakovlev #2 | ```cranberry::scheme::jakovlev2``` | Latinisation in the Soviet Union #2. |
| Yakovlev #3 | ```cranberry::scheme::jakovlev3``` | Latinisation in the Soviet Union #3. |
| Scholary | ```cranberry::scheme::scholary``` | Scientific transliteration of Cyrillic. |

> **Note:** There are plans to expand the set of supported schemes to encompass all available

## License

This project is licensed under the MIT License.

## Learn more

- [Russian Latin alphabet](https://en.wikipedia.org/wiki/Russian_Latin_alphabet)
- [Romanization of Russian](https://en.wikipedia.org/wiki/Romanization_of_Russian)
- [Russian Latin alphabet](https://en.wikipedia.org/wiki/Russian_Latin_alphabet)
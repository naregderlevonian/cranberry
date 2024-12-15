**Cranberry** is a Rust library for transliterating Russian Cyrillic characters into Latin script.

> **Warning:** This repository is currently in an unstable state. Some features may be incomplete or work poor, and there are no tests implemented at this time.

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

### An author's system

```rust
cranberry::scheme::core;
```

### Russian alphabet by Professor N. F. Yakovlev

Eliminates diacritical marks, separate from the letters:

```rust
cranberry::scheme::jakovlev1;
```

Strives for maximum use of its own Latin characters in printing houses:

```rust
cranberry::scheme::jakovlev2;
```

On the basis of the "New Turkic Alphabet":

```rust
cranberry::scheme::jakovlev3;
```

### Scholary

**Scholary** (or scientific transliteration) is primarily used in linguistics for Slavic languages:

```rust
cranberry::scheme::scholary;
```

In another version, instead of the Cyrillic letter "x" (kh), the native digraph "ch" was used.

### ISO

**ISO/R 9 1968** was the adoption of the scientific transliteration:

```rust
cranberry::scheme::iso1968;
```

**ISO/R 9:1995** is a universal system that ensures a one-to-one correspondence between character for transliteration and also permits reverse transliteration:

```rust
cranberry::scheme::iso1995;
```

> **Note:** There are plans to expand the set of supported schemes to encompass all available

### GOST

Devised by the National Administration for Geodesy and Cartography of the Soviet Union.

**GOST 16876-7**1 based on the scientific transliteration system and contains:

- Table with one Latin char with diacritics:

```rust
cranberry::scheme::gost1687671t1a;
```

- Table with two Latin char with diacritics:

```rust
cranberry::scheme::gost1687671t1b;
```

- Table with one or many Latin char without diacritics:

```rust
cranberry::scheme::gost1687671t2;
```

**GOST 7.79-2000 System A** equal to **ISO/R 9:1995**:

```rust
cranberry::scheme::gost7792000t1;
```

**GOST 7.79-2000 System B** equal to **GOST 16876-7** with one or many Latin char without diacritics:

```rust
cranberry::scheme::gost7792000t2;
```

**GOST R 52535.1-2006** defines technical requirements and standards for Russian international passports.

### United Nations System

**UN 1987** equal to **GOST 16876-7**:

```rust
cranberry::scheme::un1987;
```

### American Library Association and Library of Congress

**ALA-LC** are intended for bibliographic cataloguing, and used in British, US and Canadian libraries.

```rust
cranberry::scheme::alalc;
```

### International Civil Aviation Organization

**ALA-LC** are intended for bibliographic cataloguing, and used in British, US and Canadian libraries.

```rust
cranberry::scheme::icao;
```


## License

This project is licensed under the MIT License.
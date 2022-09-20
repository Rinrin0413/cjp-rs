# cjp.rs ![Latest release version](https://img.shields.io/github/v/release/Rinrin0413/cjp-rs?color=007722&label=Latest%20release&style=flat-square) [![Codecov](https://img.shields.io/codecov/c/github/Rinrin0413/cjp-rs?color=%#27b340&logo=Codecov&style=flat-square)](https://app.codecov.io/gh/Rinrin0413/cjp-rs)

cjp.rs is a Rust library for converting strings to 怪レい日本语(correct Japanese).

# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
cjp = "0.1.0"
```

# Examples

You can convert string to 怪レい日本语 by importing the trait `cjp::AsCJp` and calling the method `cjp` on the string.

```rust
use cjp::AsCJp;

fn main() {
    let s = "貴方は怪しい日本語を使うことが出来る。".to_string();
    println!("{}", s.cjp()); //< 贵様は怪レい日本语を使ラこと力゛出來ゑ ⸰ 

    let s = "優秀の人材はタピオカに投資して西川口に豪邸を建てる。";
    println!("{}", s.cjp()); //< 优秀の人材は夕匕才力に投资レて酉川口にごラていを建てゑ ⸰ 
}
```

And see the [docs](https://docs.rs/cjp/).

[![GPL-3.0](https://img.shields.io/github/license/Rinrin0413/cjp-rs?color=%23BD0102&style=for-the-badge)](./LICENSE.md)

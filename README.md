# cjp.rs ![Latest release version](https://img.shields.io/github/v/release/Rinrin0413/cjp-rs?color=007722&label=Latest%20release&style=flat-square) [![Codecov](https://img.shields.io/codecov/c/github/Rinrin0413/cjp-rs?color=%#27b340&logo=Codecov&style=flat-square)](https://app.codecov.io/gh/Rinrin0413/cjp-rs)

> [!CAUTION]
>
> <details>
> <summary>⚠ Important Caution - 注意事項 ⚠</summary>
> <div>
>
> ## 日本語:
> 怪レい日本语（あやしいにほんご）とは、誤訳などによって通常の日本語から大きく逸脱したもの。または意図的にそのように改変した日本語。実用性は皆無であるが、怪レい日本语に含まれるユーモアからしか得られない栄養もある(ありません)。
>
> ## For non-native speakers of Japanese:
> "怪レい日本語" (Ayashī Nihongo) refers to Japanese language that deviates significantly from standard grammar and vocabulary, often due to mistranslations or other errors. In other words, **"怪レい日本語" is NOT correct Japanese language**, so those unfamiliar with Japanese language may greatly misunderstand its meaning if they attempt to read it.
>
> </div>
> </details>

<br />

cjp.rs is a Rust library for converting strings to 怪レい日本语(Ayashī Nihongo).

## Installation

Run the following Cargo command in your project directory:

```sh
cargo add cjp
```

## Examples

You can convert strings to 怪レい日本语(Ayashī Nihongo) by importing the `cjp::AsCjp` trait and calling its method `cjp` on a string.

```rust
use cjp:AsCjp;

fn main() {
    let s = "貴方は怪しい日本語を使うことが出来る。".to_string();
    println!("{}", s.cjp()); //< 贵樣は怪レい日本语を使ラこと力゛出來ゑ ⸰ 

    let s = "優秀の人材はタピオカに投資して西川口に豪邸を建てる。";
    println!("{}", s.cjp()); //< 优秀の人材は夕匕才力に投资レて酉川口にごラていを建てゑ ⸰ 
}
```

And see the [docs](https://docs.rs/cjp).

[![MIT](https://img.shields.io/github/license/Rinrin0413/cjp-rs?color=%23A11D32&style=for-the-badge)](./LICENSE)

#![allow(clippy::needless_doctest_main)]
//! ![Latest release version](https://img.shields.io/github/v/release/Rinrin0413/cjp-rs?color=007722&label=Latest%20release&style=flat-square) [![Codecov](https://img.shields.io/codecov/c/github/Rinrin0413/cjp-rs?color=%#27b340&logo=Codecov&style=flat-square)](https://app.codecov.io/gh/Rinrin0413/cjp-rs)
//!
//! cjp.rs is a Rust library for converting strings to 怪レい日本语(correct Japanese).
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! cjp = "0.1.0"
//! ```
//!
//! # Examples
//!
//! You can convert string to 怪レい日本语 by importing the trait [`AsCJp`] and calling the method [`AsCJp::cjp`] on the string.
//!
//! ```rust
//! use cjp::AsCJp;
//!
//! fn main() {
//!     let s = "貴方は怪しい日本語を使うことが出来る。".to_string();
//!     println!("{}", s.cjp()); //< 贵様は怪レい日本语を使ラこと力゛出來ゑ ⸰
//!
//!     let s = "優秀の人材はタピオカに投資して西川口に豪邸を建てる。";
//!     println!("{}", s.cjp()); //< 优秀の人材は夕匕才力に投资レて酉川口にごラていを建てゑ ⸰
//! }
//! ```
//!
//! And see the [docs](https://docs.rs/cjp/).
//!
//! [![GPL-3.0](https://img.shields.io/github/license/Rinrin0413/cjp-rs?color=%23BD0102&style=for-the-badge)](https://docs.rs/crate/cjp/latest/source/LICENSE.md)

use crate::dict::Dict;

/// A trait that can convert strings to the 怪レい日本语(correct Japanese).
///
/// 贵樣ばこゐㇳレ亻ㇳて怪レい日本语に変換ずゑことが出來ゑ.
pub trait AsCJp {
    /// Converts this string to 怪レい日本语.
    fn cjp(self) -> String;
}

impl AsCJp for String {
    /// Converts this string to 怪レい日本语.
    ///
    /// # Examples
    ///
    /// ```
    /// use cjp::AsCJp;
    ///
    /// let s = "貴方は怪しい日本語を使うことが出来る。".to_string();
    /// assert_eq!(s.cjp(), "贵様は怪レい日本语を使ラこと力゛出來ゑ ⸰ ");
    /// ```
    fn cjp(self) -> String {
        Dict::build().convert(self)
    }
}

impl AsCJp for &str {
    /// Converts this string to 怪レい日本语.
    ///
    /// # Examples
    ///
    /// ```
    /// use cjp::AsCJp;
    ///
    // let s = "優秀の人材はタピオカに投資して西川口に豪邸を建てる。";
    // assert_eq!(s.cjp(), "优秀の人材は夕匕才力に投资レて酉川口にごラていを建てゑ ⸰ ");
    /// ```
    fn cjp(self) -> String {
        Dict::build().convert(self.to_string())
    }
}

mod dict;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_as_cjp() {
        let s = "貴方は怪しい日本語を使うことが出来る。".to_string();
        assert_eq!(s.cjp(), "贵様は怪レい日本语を使ラこと力゛出來ゑ ⸰ ");
    }

    #[test]
    fn str_as_cjp() {
        let s = "優秀の人材はタピオカに投資して西川口に豪邸を建てる。";
        assert_eq!(
            s.cjp(),
            "优秀の人材は夕匕才力に投资レて酉川口にごラていを建てゑ ⸰ "
        );
    }
}

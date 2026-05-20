/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Qubit Unicode
//!
//! Low-level Unicode, UTF-8, UTF-16, and ASCII utilities for Rust.
//!
//! This crate provides small namespace enums that mirror the low-level text
//! helpers used in Qubit's Java common library while keeping Rust's scalar
//! value and slice-based APIs explicit.
//!
//! UTF-8 validation follows the well-formed byte sequence rules in
//! [Unicode Standard, Table 3-7] and the equivalent [RFC 3629] syntax.
//!
//! [Unicode Standard, Table 3-7]: https://www.unicode.org/versions/latest/core-spec/chapter-3/#G7404
//! [RFC 3629]: https://datatracker.ietf.org/doc/html/rfc3629

mod ascii;
mod ascii_folding;
mod parsing_position;
pub mod prelude;
mod unicode;
mod unicode_error;
mod unicode_error_kind;
mod utf16;
mod utf8;

pub use ascii::Ascii;
pub use parsing_position::ParsingPosition;
pub use unicode::Unicode;
pub use unicode_error::{
    UnicodeError,
    UnicodeResult,
};
pub use unicode_error_kind::UnicodeErrorKind;
pub use utf8::Utf8;
pub use utf16::Utf16;

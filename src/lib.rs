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
//! Low-level Unicode constants, character classification helpers, and text codec
//! primitives for UTF-8, UTF-16, UTF-32, and ASCII-oriented code.
//!
//! This crate deliberately stays below `std::io::Read` and `std::io::Write`.
//! Concrete text I/O adapters are expected to own buffering, EOF handling, line
//! endings, and `std::io::Error` mapping while using the codecs from this crate
//! for strict buffer-level encoding and decoding.

mod ascii;
mod ascii_folding;
mod byte_order;
mod codecs;
pub mod prelude;
mod text_coding_error;
mod text_decoder;
mod text_encoder;
mod text_encoding;
mod unicode;
mod unicode_bom;
mod utf16;
mod utf32;
mod utf8;

pub use ascii::Ascii;
pub use byte_order::ByteOrder;
pub use codecs::{
    Utf8Codec,
    Utf8Decoder,
    Utf8Encoder,
    Utf16ByteCodec,
    Utf16ByteDecoder,
    Utf16ByteEncoder,
    Utf16U16Codec,
    Utf16U16Decoder,
    Utf16U16Encoder,
    Utf32ByteCodec,
    Utf32ByteDecoder,
    Utf32ByteEncoder,
    Utf32U32Codec,
    Utf32U32Decoder,
    Utf32U32Encoder,
};
pub use text_coding_error::{
    TextCodingError,
    TextCodingResult,
    TextDecodingError,
    TextDecodingErrorKind,
    TextDecodingResult,
    TextEncodingError,
    TextEncodingErrorKind,
    TextEncodingResult,
};
pub use text_decoder::{
    DecodeResult,
    Decoded,
    NeedMore,
    TextDecoder,
};
pub use text_encoder::{
    TextCodec,
    TextEncoder,
};
pub use text_encoding::TextEncoding;
pub use unicode::Unicode;
pub use unicode_bom::UnicodeBom;
pub use utf8::Utf8;
pub use utf16::Utf16;
pub use utf32::Utf32;

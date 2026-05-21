/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use crate::{
    DecodeResult,
    TextDecoder,
    TextDecodingResult,
    TextEncoder,
    TextEncoding,
    TextEncodingResult,
    Utf8,
};

use super::helpers;

/// Combined UTF-8 byte-buffer codec.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf8Codec;

impl TextDecoder<u8> for Utf8Codec {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::Utf8
    }

    fn max_units_per_char(&self) -> usize {
        Utf8::MAX_UNITS_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u8]) -> TextDecodingResult<DecodeResult<char>> {
        helpers::decode_utf8_prefix(input)
    }
}

impl TextEncoder<u8> for Utf8Codec {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::Utf8
    }

    fn max_units_per_char(&self) -> usize {
        Utf8::MAX_UNITS_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u8]) -> TextEncodingResult<usize> {
        helpers::encode_utf8_char(ch, output)
    }
}

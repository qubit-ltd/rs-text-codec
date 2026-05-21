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
    Utf32,
};

use super::helpers;

/// Combined UTF-32 `u32` code-unit codec.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf32U32Codec;

impl TextDecoder<u32> for Utf32U32Codec {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::Utf32
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u32]) -> TextDecodingResult<DecodeResult<char>> {
        helpers::decode_utf32_units_prefix(input)
    }
}

impl TextEncoder<u32> for Utf32U32Codec {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::Utf32
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u32]) -> TextEncodingResult<usize> {
        helpers::encode_utf32_units_char(ch, output)
    }
}

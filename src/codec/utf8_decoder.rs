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
    DecodeStatus,
    TextDecoder,
    TextDecodingResult,
    TextEncoding,
    Utf8,
};

use super::helpers;

/// Decoder for UTF-8 byte buffers.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf8Decoder;

impl TextDecoder<u8> for Utf8Decoder {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::UTF_8
    }

    fn max_units_per_char(&self) -> usize {
        Utf8::MAX_UNITS_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u8]) -> TextDecodingResult<DecodeStatus<char>> {
        helpers::decode_utf8_prefix(input)
    }
}

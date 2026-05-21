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
    TextEncoder,
    TextEncoding,
    TextEncodingResult,
    Utf16,
};

use super::helpers;

/// Encoder for UTF-16 `u16` code-unit buffers.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf16U16Encoder;

impl TextEncoder<u16> for Utf16U16Encoder {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::UTF_16
    }

    fn max_units_per_char(&self) -> usize {
        Utf16::MAX_UNITS_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u16]) -> TextEncodingResult<usize> {
        helpers::encode_utf16_units_char(ch, output)
    }
}

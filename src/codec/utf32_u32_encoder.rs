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
    Utf32,
};

use super::helpers;

/// Encoder for UTF-32 `u32` code-unit buffers.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Utf32U32Encoder;

impl TextEncoder<u32> for Utf32U32Encoder {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::UTF_32
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_UNITS_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u32]) -> TextEncodingResult<usize> {
        helpers::encode_utf32_units_char(ch, output)
    }
}

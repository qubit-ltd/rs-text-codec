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
    ByteOrder,
    TextEncoder,
    TextEncoding,
    TextEncodingResult,
    Utf32,
};

use super::helpers;

/// Encoder for byte-serialized UTF-32 buffers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Utf32ByteEncoder {
    byte_order: ByteOrder,
}

impl Utf32ByteEncoder {
    /// Creates a byte-serialized UTF-32 encoder.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used to serialize UTF-32 units.
    ///
    /// # Returns
    ///
    /// Returns a UTF-32 byte encoder.
    #[must_use]
    pub const fn new(byte_order: ByteOrder) -> Self {
        Self { byte_order }
    }

    /// Returns the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns the byte order used by this encoder.
    #[must_use]
    pub const fn byte_order(self) -> ByteOrder {
        self.byte_order
    }
}

impl TextEncoder<u8> for Utf32ByteEncoder {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::Utf32
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_BYTES_PER_CHAR
    }

    fn encode_char(&self, ch: char, output: &mut [u8]) -> TextEncodingResult<usize> {
        helpers::encode_utf32_bytes_char(ch, output, self.byte_order)
    }
}

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
    DecodeResult,
    TextDecoder,
    TextDecodingResult,
    TextEncoding,
    Utf32,
};

use super::helpers;

/// Decoder for byte-serialized UTF-32 buffers.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Utf32ByteDecoder {
    byte_order: ByteOrder,
}

impl Utf32ByteDecoder {
    /// Creates a byte-serialized UTF-32 decoder.
    ///
    /// # Parameters
    ///
    /// - `byte_order`: The byte order used by the input bytes.
    ///
    /// # Returns
    ///
    /// Returns a UTF-32 byte decoder.
    #[must_use]
    pub const fn new(byte_order: ByteOrder) -> Self {
        Self { byte_order }
    }

    /// Returns the configured byte order.
    ///
    /// # Returns
    ///
    /// Returns the byte order used by this decoder.
    #[must_use]
    pub const fn byte_order(self) -> ByteOrder {
        self.byte_order
    }
}

impl TextDecoder<u8> for Utf32ByteDecoder {
    fn encoding(&self) -> TextEncoding {
        TextEncoding::Utf32
    }

    fn max_units_per_char(&self) -> usize {
        Utf32::MAX_BYTES_PER_CHAR
    }

    fn decode_prefix(&self, input: &[u8]) -> TextDecodingResult<DecodeResult<char>> {
        helpers::decode_utf32_bytes_prefix(input, self.byte_order)
    }
}

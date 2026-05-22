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
    Leb128DecodeError,
    Leb128DecodeErrorKind,
};

const MAX_LEB128_BYTES: usize = 19;

/// Buffer-level codec for unsigned and signed LEB128 integers.
///
/// The codec does not perform `std::io::Read` or `std::io::Write`; it works
/// directly on caller-owned byte slices and reports how many bytes were consumed
/// or written.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Leb128Codec {
    strict: bool,
}

impl Leb128Codec {
    /// Creates a non-strict LEB128 codec.
    ///
    /// # Returns
    ///
    /// Returns a codec that accepts non-canonical encodings when decoding.
    #[inline]
    pub const fn new() -> Self {
        Self { strict: false }
    }

    /// Creates a LEB128 codec with an explicit canonical decoding policy.
    ///
    /// # Parameters
    ///
    /// - `strict`: Whether to reject non-canonical encodings.
    ///
    /// # Returns
    ///
    /// Returns a codec configured with the supplied policy.
    #[inline]
    pub const fn with_strict(strict: bool) -> Self {
        Self { strict }
    }

    /// Reports whether strict canonical decoding is enabled.
    #[must_use]
    #[inline]
    pub const fn strict(self) -> bool {
        self.strict
    }

    /// Updates the canonical decoding policy.
    ///
    /// # Parameters
    ///
    /// - `strict`: Whether to reject non-canonical encodings.
    #[inline]
    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }

    /// Reads an unsigned LEB128 `u16` at `index`.
    #[inline]
    pub fn read_uleb_u16_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(u16, usize)>, Leb128DecodeError> {
        self.read_uleb_at(input, index, u16::BITS)
            .map(|decoded| decoded.map(|(value, consumed)| (value as u16, consumed)))
    }

    /// Reads an unsigned LEB128 `u32` at `index`.
    #[inline]
    pub fn read_uleb_u32_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(u32, usize)>, Leb128DecodeError> {
        self.read_uleb_at(input, index, u32::BITS)
            .map(|decoded| decoded.map(|(value, consumed)| (value as u32, consumed)))
    }

    /// Reads an unsigned LEB128 `u64` at `index`.
    #[inline]
    pub fn read_uleb_u64_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(u64, usize)>, Leb128DecodeError> {
        self.read_uleb_at(input, index, u64::BITS)
            .map(|decoded| decoded.map(|(value, consumed)| (value as u64, consumed)))
    }

    /// Reads an unsigned LEB128 `u128` at `index`.
    #[inline]
    pub fn read_uleb_u128_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(u128, usize)>, Leb128DecodeError> {
        self.read_uleb_at(input, index, u128::BITS)
    }

    /// Reads a signed LEB128 `i16` at `index`.
    #[inline]
    pub fn read_sleb_i16_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i16, usize)>, Leb128DecodeError> {
        self.read_sleb_at(input, index, i16::BITS)
            .map(|decoded| decoded.map(|(value, consumed)| (value as i16, consumed)))
    }

    /// Reads a signed LEB128 `i32` at `index`.
    #[inline]
    pub fn read_sleb_i32_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i32, usize)>, Leb128DecodeError> {
        self.read_sleb_at(input, index, i32::BITS)
            .map(|decoded| decoded.map(|(value, consumed)| (value as i32, consumed)))
    }

    /// Reads a signed LEB128 `i64` at `index`.
    #[inline]
    pub fn read_sleb_i64_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i64, usize)>, Leb128DecodeError> {
        self.read_sleb_at(input, index, i64::BITS)
            .map(|decoded| decoded.map(|(value, consumed)| (value as i64, consumed)))
    }

    /// Reads a signed LEB128 `i128` at `index`.
    #[inline]
    pub fn read_sleb_i128_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i128, usize)>, Leb128DecodeError> {
        self.read_sleb_at(input, index, i128::BITS)
    }

    /// Writes an unsigned LEB128 `u16` at `index`.
    #[inline]
    pub fn write_uleb_u16_at(self, output: &mut [u8], index: usize, value: u16) -> Option<usize> {
        self.write_uleb_at(output, index, value as u128)
    }

    /// Writes an unsigned LEB128 `u32` at `index`.
    #[inline]
    pub fn write_uleb_u32_at(self, output: &mut [u8], index: usize, value: u32) -> Option<usize> {
        self.write_uleb_at(output, index, value as u128)
    }

    /// Writes an unsigned LEB128 `u64` at `index`.
    #[inline]
    pub fn write_uleb_u64_at(self, output: &mut [u8], index: usize, value: u64) -> Option<usize> {
        self.write_uleb_at(output, index, value as u128)
    }

    /// Writes an unsigned LEB128 `u128` at `index`.
    #[inline]
    pub fn write_uleb_u128_at(self, output: &mut [u8], index: usize, value: u128) -> Option<usize> {
        self.write_uleb_at(output, index, value)
    }

    /// Writes a signed LEB128 `i16` at `index`.
    #[inline]
    pub fn write_sleb_i16_at(self, output: &mut [u8], index: usize, value: i16) -> Option<usize> {
        self.write_sleb_at(output, index, value as i128)
    }

    /// Writes a signed LEB128 `i32` at `index`.
    #[inline]
    pub fn write_sleb_i32_at(self, output: &mut [u8], index: usize, value: i32) -> Option<usize> {
        self.write_sleb_at(output, index, value as i128)
    }

    /// Writes a signed LEB128 `i64` at `index`.
    #[inline]
    pub fn write_sleb_i64_at(self, output: &mut [u8], index: usize, value: i64) -> Option<usize> {
        self.write_sleb_at(output, index, value as i128)
    }

    /// Writes a signed LEB128 `i128` at `index`.
    #[inline]
    pub fn write_sleb_i128_at(self, output: &mut [u8], index: usize, value: i128) -> Option<usize> {
        self.write_sleb_at(output, index, value)
    }

    /// Reads an unsigned LEB128 value with `bits` target width.
    fn read_uleb_at(
        self,
        input: &[u8],
        index: usize,
        bits: u32,
    ) -> Result<Option<(u128, usize)>, Leb128DecodeError> {
        if index > input.len() {
            return Err(Leb128DecodeError::new(
                Leb128DecodeErrorKind::Malformed,
                index,
            ));
        }
        let Some((value, consumed)) = read_uleb_raw(input, index, bits)? else {
            return Ok(None);
        };
        if self.strict && !is_canonical_uleb(value, &input[index..index + consumed]) {
            return Err(Leb128DecodeError::new(
                Leb128DecodeErrorKind::NonCanonical,
                index,
            ));
        }
        Ok(Some((value, consumed)))
    }

    /// Reads a signed LEB128 value with `bits` target width.
    fn read_sleb_at(
        self,
        input: &[u8],
        index: usize,
        bits: u32,
    ) -> Result<Option<(i128, usize)>, Leb128DecodeError> {
        if index > input.len() {
            return Err(Leb128DecodeError::new(
                Leb128DecodeErrorKind::Malformed,
                index,
            ));
        }
        let Some((value, consumed)) = read_sleb_raw(input, index, bits)? else {
            return Ok(None);
        };
        if self.strict && !is_canonical_sleb(value, &input[index..index + consumed]) {
            return Err(Leb128DecodeError::new(
                Leb128DecodeErrorKind::NonCanonical,
                index,
            ));
        }
        Ok(Some((value, consumed)))
    }

    /// Writes an unsigned LEB128 value at `index`.
    fn write_uleb_at(self, output: &mut [u8], index: usize, value: u128) -> Option<usize> {
        let mut encoded = [0_u8; MAX_LEB128_BYTES];
        let count = encode_uleb(value, &mut encoded);
        if output.len().saturating_sub(index) < count {
            return None;
        }
        output[index..index + count].copy_from_slice(&encoded[..count]);
        Some(count)
    }

    /// Writes a signed LEB128 value at `index`.
    fn write_sleb_at(self, output: &mut [u8], index: usize, value: i128) -> Option<usize> {
        let mut encoded = [0_u8; MAX_LEB128_BYTES];
        let count = encode_sleb(value, &mut encoded);
        if output.len().saturating_sub(index) < count {
            return None;
        }
        output[index..index + count].copy_from_slice(&encoded[..count]);
        Some(count)
    }
}

impl Default for Leb128Codec {
    /// Creates the default non-strict LEB128 codec.
    fn default() -> Self {
        Self::new()
    }
}

/// Reads an unsigned LEB128 value without canonical validation.
fn read_uleb_raw(
    input: &[u8],
    index: usize,
    bits: u32,
) -> Result<Option<(u128, usize)>, Leb128DecodeError> {
    let max_bytes = bits.div_ceil(7) as usize;
    let final_payload_bits = bits - ((max_bytes as u32 - 1) * 7);
    let max_last_payload = ((1u16 << final_payload_bits) - 1) as u8;
    let mut value = 0u128;

    for offset in 0..max_bytes {
        let byte_index = index + offset;
        let Some(&byte) = input.get(byte_index) else {
            return Ok(None);
        };
        let payload = byte & 0x7f;
        if offset == max_bytes - 1 && payload > max_last_payload {
            return Err(Leb128DecodeError::new(
                Leb128DecodeErrorKind::Malformed,
                byte_index,
            ));
        }
        value |= (payload as u128) << (offset * 7);
        if byte & 0x80 == 0 {
            return Ok(Some((value, offset + 1)));
        }
    }
    Err(Leb128DecodeError::new(
        Leb128DecodeErrorKind::Malformed,
        index + max_bytes.saturating_sub(1),
    ))
}

/// Reads a signed LEB128 value without canonical validation.
fn read_sleb_raw(
    input: &[u8],
    index: usize,
    bits: u32,
) -> Result<Option<(i128, usize)>, Leb128DecodeError> {
    let max_bytes = bits.div_ceil(7) as usize;
    let mut value = 0i128;
    let mut shift = 0u32;

    for offset in 0..max_bytes {
        let byte_index = index + offset;
        let Some(&byte) = input.get(byte_index) else {
            return Ok(None);
        };
        let payload = byte & 0x7f;
        if is_too_wide_signed_final_payload(payload, offset as u32, bits) {
            return Err(Leb128DecodeError::new(
                Leb128DecodeErrorKind::Malformed,
                byte_index,
            ));
        }
        value |= (payload as i128) << shift;
        shift += 7;
        if byte & 0x80 == 0 {
            if shift < i128::BITS && byte & 0x40 != 0 {
                value |= (!0i128) << shift;
            }
            return Ok(Some((value, offset + 1)));
        }
    }
    Err(Leb128DecodeError::new(
        Leb128DecodeErrorKind::Malformed,
        index + max_bytes.saturating_sub(1),
    ))
}

/// Checks whether a final signed payload byte exceeds the target width.
fn is_too_wide_signed_final_payload(payload: u8, offset: u32, bits: u32) -> bool {
    let max_bytes = bits.div_ceil(7);
    if offset != max_bytes - 1 {
        return false;
    }
    let used_bits = bits - offset * 7;
    let sign_mask = 1u8 << (used_bits - 1);
    let used_mask = (1u8 << used_bits) - 1;
    let unused_mask = 0x7f_u8 & !used_mask;
    let unused_bits = payload & unused_mask;
    if payload & sign_mask == 0 {
        unused_bits != 0
    } else {
        unused_bits != unused_mask
    }
}

/// Checks whether bytes are the canonical unsigned encoding for `value`.
fn is_canonical_uleb(value: u128, bytes: &[u8]) -> bool {
    let mut expected = [0_u8; MAX_LEB128_BYTES];
    let count = encode_uleb(value, &mut expected);
    &expected[..count] == bytes
}

/// Checks whether bytes are the canonical signed encoding for `value`.
fn is_canonical_sleb(value: i128, bytes: &[u8]) -> bool {
    let mut expected = [0_u8; MAX_LEB128_BYTES];
    let count = encode_sleb(value, &mut expected);
    &expected[..count] == bytes
}

/// Encodes an unsigned LEB128 value into `output`.
fn encode_uleb(mut value: u128, output: &mut [u8; MAX_LEB128_BYTES]) -> usize {
    let mut index = 0;
    while value > 0x7f {
        output[index] = ((value as u8) & 0x7f) | 0x80;
        value >>= 7;
        index += 1;
    }
    output[index] = value as u8;
    index + 1
}

/// Encodes a signed LEB128 value into `output`.
fn encode_sleb(value: i128, output: &mut [u8; MAX_LEB128_BYTES]) -> usize {
    let mut remaining = value;
    let mut index = 0;
    loop {
        let byte = (remaining as u8) & 0x7f;
        remaining >>= 7;
        let is_done = (remaining == 0 && byte & 0x40 == 0) || (remaining == -1 && byte & 0x40 != 0);
        output[index] = if is_done { byte } else { byte | 0x80 };
        index += 1;
        if is_done {
            return index;
        }
    }
}

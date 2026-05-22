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
    Leb128Codec,
    Leb128DecodeError,
};

/// Buffer-level codec for ZigZag signed integers.
///
/// ZigZag maps signed integers to unsigned integers before writing the payload
/// as unsigned LEB128. This keeps small negative values compact.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZigZagCodec {
    leb128: Leb128Codec,
}

impl ZigZagCodec {
    /// Creates a non-strict ZigZag codec.
    #[inline]
    pub const fn new() -> Self {
        Self {
            leb128: Leb128Codec::new(),
        }
    }

    /// Creates a ZigZag codec with an explicit LEB128 canonical decoding policy.
    ///
    /// # Parameters
    ///
    /// - `strict`: Whether to reject non-canonical LEB128 payloads.
    #[inline]
    pub const fn with_strict(strict: bool) -> Self {
        Self {
            leb128: Leb128Codec::with_strict(strict),
        }
    }

    /// Reports whether strict canonical decoding is enabled.
    #[must_use]
    #[inline]
    pub const fn strict(self) -> bool {
        self.leb128.strict()
    }

    /// Updates the LEB128 canonical decoding policy.
    #[inline]
    pub fn set_strict(&mut self, strict: bool) {
        self.leb128.set_strict(strict);
    }

    /// Encodes an `i16` value into its ZigZag `u16` representation.
    #[must_use]
    #[inline]
    pub const fn encode_i16(value: i16) -> u16 {
        ((value << 1) ^ (value >> 15)) as u16
    }

    /// Decodes a ZigZag `u16` value into `i16`.
    #[must_use]
    #[inline]
    pub const fn decode_u16(value: u16) -> i16 {
        ((value >> 1) as i16) ^ (-((value & 1) as i16))
    }

    /// Encodes an `i32` value into its ZigZag `u32` representation.
    #[must_use]
    #[inline]
    pub const fn encode_i32(value: i32) -> u32 {
        ((value << 1) ^ (value >> 31)) as u32
    }

    /// Decodes a ZigZag `u32` value into `i32`.
    #[must_use]
    #[inline]
    pub const fn decode_u32(value: u32) -> i32 {
        ((value >> 1) as i32) ^ (-((value & 1) as i32))
    }

    /// Encodes an `i64` value into its ZigZag `u64` representation.
    #[must_use]
    #[inline]
    pub const fn encode_i64(value: i64) -> u64 {
        ((value << 1) ^ (value >> 63)) as u64
    }

    /// Decodes a ZigZag `u64` value into `i64`.
    #[must_use]
    #[inline]
    pub const fn decode_u64(value: u64) -> i64 {
        ((value >> 1) as i64) ^ (-((value & 1) as i64))
    }

    /// Encodes an `i128` value into its ZigZag `u128` representation.
    #[must_use]
    #[inline]
    pub const fn encode_i128(value: i128) -> u128 {
        ((value << 1) ^ (value >> 127)) as u128
    }

    /// Decodes a ZigZag `u128` value into `i128`.
    #[must_use]
    #[inline]
    pub const fn decode_u128(value: u128) -> i128 {
        ((value >> 1) as i128) ^ (-((value & 1) as i128))
    }

    /// Reads a ZigZag encoded `i16` at `index`.
    #[inline]
    pub fn read_i16_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i16, usize)>, Leb128DecodeError> {
        self.leb128
            .read_uleb_u16_at(input, index)
            .map(|decoded| decoded.map(|(value, consumed)| (Self::decode_u16(value), consumed)))
    }

    /// Reads a ZigZag encoded `i32` at `index`.
    #[inline]
    pub fn read_i32_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i32, usize)>, Leb128DecodeError> {
        self.leb128
            .read_uleb_u32_at(input, index)
            .map(|decoded| decoded.map(|(value, consumed)| (Self::decode_u32(value), consumed)))
    }

    /// Reads a ZigZag encoded `i64` at `index`.
    #[inline]
    pub fn read_i64_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i64, usize)>, Leb128DecodeError> {
        self.leb128
            .read_uleb_u64_at(input, index)
            .map(|decoded| decoded.map(|(value, consumed)| (Self::decode_u64(value), consumed)))
    }

    /// Reads a ZigZag encoded `i128` at `index`.
    #[inline]
    pub fn read_i128_at(
        self,
        input: &[u8],
        index: usize,
    ) -> Result<Option<(i128, usize)>, Leb128DecodeError> {
        self.leb128
            .read_uleb_u128_at(input, index)
            .map(|decoded| decoded.map(|(value, consumed)| (Self::decode_u128(value), consumed)))
    }

    /// Writes a ZigZag encoded `i16` at `index`.
    #[inline]
    pub fn write_i16_at(self, output: &mut [u8], index: usize, value: i16) -> Option<usize> {
        self.leb128
            .write_uleb_u16_at(output, index, Self::encode_i16(value))
    }

    /// Writes a ZigZag encoded `i32` at `index`.
    #[inline]
    pub fn write_i32_at(self, output: &mut [u8], index: usize, value: i32) -> Option<usize> {
        self.leb128
            .write_uleb_u32_at(output, index, Self::encode_i32(value))
    }

    /// Writes a ZigZag encoded `i64` at `index`.
    #[inline]
    pub fn write_i64_at(self, output: &mut [u8], index: usize, value: i64) -> Option<usize> {
        self.leb128
            .write_uleb_u64_at(output, index, Self::encode_i64(value))
    }

    /// Writes a ZigZag encoded `i128` at `index`.
    #[inline]
    pub fn write_i128_at(self, output: &mut [u8], index: usize, value: i128) -> Option<usize> {
        self.leb128
            .write_uleb_u128_at(output, index, Self::encode_i128(value))
    }
}

impl Default for ZigZagCodec {
    /// Creates the default non-strict ZigZag codec.
    fn default() -> Self {
        Self::new()
    }
}

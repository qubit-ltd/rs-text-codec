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
    ParsingPosition,
    Unicode,
    UnicodeError,
    UnicodeErrorKind,
    UnicodeResult,
};

/// Namespace for low-level UTF-8 helpers.
pub enum Utf8 {}

impl Utf8 {
    /// Maximum number of UTF-8 code units needed for a scalar value.
    pub const MAX_CODE_UNIT_COUNT: usize = 4;

    /// Minimum leading UTF-8 byte.
    pub const MIN_LEADING: u8 = 0xc2;

    /// Maximum leading UTF-8 byte.
    pub const MAX_LEADING: u8 = 0xf4;

    /// Mask for trailing UTF-8 bytes.
    pub const TRAILING_MASK: u8 = 0xc0;

    /// Pattern for trailing UTF-8 bytes.
    pub const TRAILING_PATTERN: u8 = 0x80;

    /// Maximum code point encoded by one UTF-8 byte.
    pub const MAX_ONE_CODE_UNIT: u32 = 0x7f;

    /// Maximum code point encoded by two UTF-8 bytes.
    pub const MAX_TWO_CODE_UNIT: u32 = 0x7ff;

    /// Maximum code point encoded by three UTF-8 bytes.
    pub const MAX_THREE_CODE_UNIT: u32 = 0xffff;

    /// Maximum code point encoded by four UTF-8 bytes.
    pub const MAX_FOUR_CODE_UNIT: u32 = Unicode::UNICODE_MAX;

    /// Returns `true` if the byte encodes a scalar value by itself.
    #[must_use]
    pub const fn is_single(ch: u8) -> bool {
        ch <= Self::MAX_ONE_CODE_UNIT as u8
    }

    /// Returns `true` if the byte can be a leading UTF-8 byte.
    #[must_use]
    pub const fn is_leading(ch: u8) -> bool {
        ch >= Self::MIN_LEADING && ch <= Self::MAX_LEADING
    }

    /// Returns `true` if the byte is a trailing UTF-8 byte.
    #[must_use]
    pub const fn is_trailing(ch: u8) -> bool {
        (ch & Self::TRAILING_MASK) == Self::TRAILING_PATTERN
    }

    /// Returns the number of trailing bytes required by a leading byte.
    #[must_use]
    pub const fn trailing_count(ch: u8) -> Option<usize> {
        if ch >= 0xc2 && ch <= 0xdf {
            Some(1)
        } else if ch >= 0xe0 && ch <= 0xef {
            Some(2)
        } else if ch >= 0xf0 && ch <= 0xf4 {
            Some(3)
        } else {
            None
        }
    }

    /// Returns the number of UTF-8 bytes needed for a scalar value.
    #[must_use]
    pub const fn code_unit_count(code_point: u32) -> Option<usize> {
        if code_point > Unicode::UNICODE_MAX || Unicode::is_surrogate(code_point as i32) {
            None
        } else if code_point <= Self::MAX_ONE_CODE_UNIT {
            Some(1)
        } else if code_point <= Self::MAX_TWO_CODE_UNIT {
            Some(2)
        } else if code_point <= Self::MAX_THREE_CODE_UNIT {
            Some(3)
        } else {
            Some(4)
        }
    }

    /// Moves a cursor from a trailing byte to the start of its code point.
    pub fn set_to_start(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        start_index: usize,
    ) -> UnicodeResult<usize> {
        let index = pos.index();
        assert!(start_index <= index && index < buffer.len());
        if !Self::is_trailing(buffer[index]) {
            return Ok(0);
        }
        for candidate in (start_index..index).rev() {
            let byte = buffer[candidate];
            if Self::is_leading(byte) {
                let expected = Self::trailing_count(byte).expect("leading byte has trailing count");
                let actual = index - candidate;
                if actual <= expected {
                    pos.set_index(candidate);
                    return Ok(actual);
                }
                return Self::fail(pos, candidate, UnicodeErrorKind::MalformedUnicode);
            }
            if !Self::is_trailing(byte) {
                return Self::fail(pos, candidate, UnicodeErrorKind::MalformedUnicode);
            }
            if index - candidate >= Self::MAX_CODE_UNIT_COUNT - 1 {
                return Self::fail(pos, candidate, UnicodeErrorKind::MalformedUnicode);
            }
        }
        Self::fail(pos, start_index, UnicodeErrorKind::IncompleteUnicode)
    }

    /// Moves a cursor from a leading byte to the terminal byte of its code point.
    pub fn set_to_terminal(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        let index = pos.index();
        assert!(end_index <= buffer.len() && index <= end_index);
        if index == end_index || !Self::is_leading(buffer[index]) {
            return Ok(0);
        }
        let trailing_count = Self::trailing_count(buffer[index]).expect("leading byte");
        let terminal_index = index + trailing_count;
        if terminal_index >= end_index {
            return Self::fail(pos, end_index, UnicodeErrorKind::IncompleteUnicode);
        }
        for (current, byte) in buffer
            .iter()
            .enumerate()
            .take(terminal_index + 1)
            .skip(index + 1)
        {
            if !Self::is_trailing(*byte) {
                return Self::fail(pos, current, UnicodeErrorKind::MalformedUnicode);
            }
        }
        pos.set_index(terminal_index);
        Ok(trailing_count)
    }

    /// Advances the cursor over one UTF-8 code point.
    pub fn forward(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        let old_index = pos.index();
        match Self::get_next(pos, buffer, end_index)? {
            Some(_) => Ok(pos.index() - old_index),
            None => Ok(0),
        }
    }

    /// Moves the cursor backward over one UTF-8 code point.
    pub fn backward(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        start_index: usize,
    ) -> UnicodeResult<usize> {
        let index = pos.index();
        assert!(start_index <= index && index <= buffer.len());
        if index == start_index {
            return Ok(0);
        }
        let previous_index = index - 1;
        let previous = buffer[previous_index];
        if Self::is_single(previous) {
            pos.set_index(previous_index);
            return Ok(1);
        }
        if !Self::is_trailing(previous) {
            return Self::fail(pos, previous_index, UnicodeErrorKind::MalformedUnicode);
        }
        for candidate in (start_index..previous_index).rev() {
            let byte = buffer[candidate];
            if Self::is_leading(byte) {
                let expected = Self::trailing_count(byte).expect("leading byte") + 1;
                let actual = index - candidate;
                if actual == expected {
                    pos.set_index(candidate);
                    return Ok(actual);
                }
                return Self::fail(pos, candidate, UnicodeErrorKind::MalformedUnicode);
            }
            if !Self::is_trailing(byte) {
                return Self::fail(pos, candidate, UnicodeErrorKind::MalformedUnicode);
            }
            if index - candidate >= Self::MAX_CODE_UNIT_COUNT {
                return Self::fail(pos, candidate, UnicodeErrorKind::MalformedUnicode);
            }
        }
        Self::fail(pos, start_index, UnicodeErrorKind::IncompleteUnicode)
    }

    /// Reads the next UTF-8 code point and advances the cursor.
    pub fn get_next(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        end_index: usize,
    ) -> UnicodeResult<Option<char>> {
        let index = pos.index();
        assert!(end_index <= buffer.len() && index <= end_index);
        if index == end_index {
            return Ok(None);
        }
        let first = buffer[index];
        if Self::is_single(first) {
            pos.set_index(index + 1);
            return Ok(Some(first as char));
        }
        let count = match Self::trailing_count(first) {
            Some(count) => count + 1,
            None => return Self::fail(pos, index, UnicodeErrorKind::MalformedUnicode),
        };
        let next_index = index + count;
        if next_index > end_index {
            return Self::fail(pos, end_index, UnicodeErrorKind::IncompleteUnicode);
        }
        for (current, byte) in buffer.iter().enumerate().take(next_index).skip(index + 1) {
            if !Self::is_trailing(*byte) {
                return Self::fail(pos, current, UnicodeErrorKind::MalformedUnicode);
            }
        }
        match std::str::from_utf8(&buffer[index..next_index]) {
            Ok(text) => {
                let ch = text.chars().next().expect("validated non-empty UTF-8");
                pos.set_index(next_index);
                Ok(Some(ch))
            }
            Err(error) => {
                let error_index = index + error.valid_up_to() + error.error_len().unwrap_or(0);
                let kind = if error.error_len().is_none() {
                    UnicodeErrorKind::IncompleteUnicode
                } else {
                    UnicodeErrorKind::MalformedUnicode
                };
                Self::fail(pos, error_index, kind)
            }
        }
    }

    /// Reads the previous UTF-8 code point and moves the cursor to its start.
    pub fn get_previous(
        pos: &mut ParsingPosition,
        buffer: &[u8],
        start_index: usize,
    ) -> UnicodeResult<Option<char>> {
        let old_index = pos.index();
        let count = Self::backward(pos, buffer, start_index)?;
        if count == 0 {
            return Ok(None);
        }
        let new_index = pos.index();
        let mut read_pos = ParsingPosition::new(new_index);
        match Self::get_next(&mut read_pos, buffer, old_index) {
            Ok(Some(ch)) => {
                pos.set_index(new_index);
                Ok(Some(ch))
            }
            Ok(None) => unreachable!("backward moved over a code point"),
            Err(error) => {
                pos.reset(old_index);
                pos.set_error(error.index(), error.kind());
                Err(error)
            }
        }
    }

    /// Encodes a scalar value into a UTF-8 output buffer.
    pub fn put(
        code_point: u32,
        index: usize,
        buffer: &mut [u8],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        assert!(end_index <= buffer.len() && index <= end_index);
        let ch = match char::from_u32(code_point) {
            Some(ch) => ch,
            None => {
                return Err(UnicodeError::new(UnicodeErrorKind::MalformedUnicode, index));
            }
        };
        let count = ch.len_utf8();
        if index + count > end_index {
            return Err(UnicodeError::new(
                UnicodeErrorKind::BufferOverflow,
                end_index,
            ));
        }
        ch.encode_utf8(&mut buffer[index..index + count]);
        Ok(count)
    }

    /// Records an error on the cursor and returns it.
    fn fail<T>(
        pos: &mut ParsingPosition,
        index: usize,
        kind: UnicodeErrorKind,
    ) -> UnicodeResult<T> {
        pos.set_error(index, kind);
        Err(UnicodeError::new(kind, index))
    }
}

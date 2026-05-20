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

/// Namespace for low-level UTF-16 helpers.
pub enum Utf16 {}

impl Utf16 {
    /// Maximum number of UTF-16 code units needed for a scalar value.
    pub const MAX_CODE_UNIT_COUNT: usize = 2;

    /// Returns `true` if the code unit encodes a scalar value by itself.
    #[must_use]
    pub const fn is_single(ch: u16) -> bool {
        !Self::is_surrogate(ch)
    }

    /// Returns `true` if the code unit is a high surrogate.
    #[must_use]
    pub const fn is_leading(ch: u16) -> bool {
        Unicode::is_high_surrogate(ch as i32)
    }

    /// Returns `true` if the code unit is a low surrogate.
    #[must_use]
    pub const fn is_trailing(ch: u16) -> bool {
        Unicode::is_low_surrogate(ch as i32)
    }

    /// Returns `true` if the code unit is any UTF-16 surrogate.
    #[must_use]
    pub const fn is_surrogate(ch: u16) -> bool {
        Unicode::is_surrogate(ch as i32)
    }

    /// Returns `true` if the code units form a surrogate pair.
    #[must_use]
    pub const fn is_surrogate_pair(high: u16, low: u16) -> bool {
        Unicode::is_surrogate_pair(high, low)
    }

    /// Composes a UTF-16 surrogate pair into a Unicode code point.
    #[must_use]
    pub const fn compose(high: u16, low: u16) -> Option<u32> {
        Unicode::compose_surrogate_pair(high, low)
    }

    /// Decomposes a supplementary code point into its high surrogate.
    #[must_use]
    pub const fn decompose_high(code_point: u32) -> Option<u16> {
        Unicode::decompose_high_surrogate(code_point)
    }

    /// Decomposes a supplementary code point into its low surrogate.
    #[must_use]
    pub const fn decompose_low(code_point: u32) -> Option<u16> {
        Unicode::decompose_low_surrogate(code_point)
    }

    /// Returns the number of trailing UTF-16 code units for this leading unit.
    #[must_use]
    pub const fn trailing_count(ch: u16) -> usize {
        if Self::is_leading(ch) { 1 } else { 0 }
    }

    /// Returns the number of UTF-16 code units needed for a scalar value.
    #[must_use]
    pub const fn code_unit_count(code_point: u32) -> Option<usize> {
        if code_point > Unicode::UNICODE_MAX || Unicode::is_surrogate(code_point as i32) {
            None
        } else if code_point >= Unicode::SUPPLEMENTARY_MIN {
            Some(2)
        } else {
            Some(1)
        }
    }

    /// Moves a cursor from a trailing unit to the start of its code point.
    pub fn set_to_start(
        pos: &mut ParsingPosition,
        buffer: &[u16],
        start_index: usize,
    ) -> UnicodeResult<usize> {
        let index = pos.index();
        assert!(start_index <= index && index < buffer.len());
        if !Self::is_trailing(buffer[index]) {
            return Ok(0);
        }
        if index == start_index {
            return Self::fail(pos, index, UnicodeErrorKind::IncompleteUnicode);
        }
        let leading_index = index - 1;
        if Self::is_leading(buffer[leading_index]) {
            pos.set_index(leading_index);
            Ok(1)
        } else {
            Self::fail(pos, leading_index, UnicodeErrorKind::MalformedUnicode)
        }
    }

    /// Moves a cursor from a leading unit to the terminal unit of its code point.
    pub fn set_to_terminal(
        pos: &mut ParsingPosition,
        buffer: &[u16],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        let index = pos.index();
        assert!(end_index <= buffer.len() && index <= end_index);
        if index == end_index || !Self::is_leading(buffer[index]) {
            return Ok(0);
        }
        let trailing_index = index + 1;
        if trailing_index >= end_index {
            return Self::fail(pos, trailing_index, UnicodeErrorKind::IncompleteUnicode);
        }
        if Self::is_trailing(buffer[trailing_index]) {
            pos.set_index(trailing_index);
            Ok(1)
        } else {
            Self::fail(pos, trailing_index, UnicodeErrorKind::MalformedUnicode)
        }
    }

    /// Advances the cursor over one UTF-16 code point.
    pub fn forward(
        pos: &mut ParsingPosition,
        buffer: &[u16],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        let old_index = pos.index();
        match Self::get_next(pos, buffer, end_index)? {
            Some(_) => Ok(pos.index() - old_index),
            None => Ok(0),
        }
    }

    /// Moves the cursor backward over one UTF-16 code point.
    pub fn backward(
        pos: &mut ParsingPosition,
        buffer: &[u16],
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
            Ok(1)
        } else if !Self::is_trailing(previous) {
            Self::fail(pos, previous_index, UnicodeErrorKind::MalformedUnicode)
        } else if previous_index == start_index {
            Self::fail(pos, previous_index, UnicodeErrorKind::IncompleteUnicode)
        } else {
            let leading_index = previous_index - 1;
            if Self::is_leading(buffer[leading_index]) {
                pos.set_index(leading_index);
                Ok(2)
            } else {
                Self::fail(pos, leading_index, UnicodeErrorKind::MalformedUnicode)
            }
        }
    }

    /// Reads the next UTF-16 code point and advances the cursor.
    pub fn get_next(
        pos: &mut ParsingPosition,
        buffer: &[u16],
        end_index: usize,
    ) -> UnicodeResult<Option<char>> {
        let index = pos.index();
        assert!(end_index <= buffer.len() && index <= end_index);
        if index == end_index {
            return Ok(None);
        }
        let unit = buffer[index];
        if Self::is_single(unit) {
            let ch = char::from_u32(unit as u32)
                .expect("non-surrogate UTF-16 code unit must be a scalar value");
            pos.set_index(index + 1);
            Ok(Some(ch))
        } else if !Self::is_leading(unit) {
            Self::fail(pos, index, UnicodeErrorKind::MalformedUnicode)
        } else {
            let trailing_index = index + 1;
            if trailing_index >= end_index {
                return Self::fail(pos, trailing_index, UnicodeErrorKind::IncompleteUnicode);
            }
            let trailing = buffer[trailing_index];
            if let Some(code_point) = Self::compose(unit, trailing) {
                let ch = char::from_u32(code_point)
                    .expect("composed surrogate pair must be a scalar value");
                pos.set_index(trailing_index + 1);
                Ok(Some(ch))
            } else {
                Self::fail(pos, trailing_index, UnicodeErrorKind::MalformedUnicode)
            }
        }
    }

    /// Reads the previous UTF-16 code point and moves the cursor to its start.
    pub fn get_previous(
        pos: &mut ParsingPosition,
        buffer: &[u16],
        start_index: usize,
    ) -> UnicodeResult<Option<char>> {
        let old_index = pos.index();
        let count = Self::backward(pos, buffer, start_index)?;
        if count == 0 {
            return Ok(None);
        }
        let new_index = pos.index();
        let ch = if count == 1 {
            char::from_u32(buffer[new_index] as u32)
                .expect("non-surrogate UTF-16 code unit must be a scalar value")
        } else {
            let code_point = Self::compose(buffer[new_index], buffer[old_index - 1])
                .expect("backward validated the surrogate pair");
            char::from_u32(code_point).expect("composed surrogate pair must be a scalar value")
        };
        pos.set_index(new_index);
        Ok(Some(ch))
    }

    /// Encodes a scalar value into a UTF-16 output buffer.
    pub fn put(
        code_point: u32,
        index: usize,
        buffer: &mut [u16],
        end_index: usize,
    ) -> UnicodeResult<usize> {
        assert!(end_index <= buffer.len() && index <= end_index);
        let count = match Self::code_unit_count(code_point) {
            Some(count) => count,
            None => {
                return Err(UnicodeError::new(UnicodeErrorKind::MalformedUnicode, index));
            }
        };
        if index + count > end_index {
            return Err(UnicodeError::new(
                UnicodeErrorKind::BufferOverflow,
                end_index,
            ));
        }
        if count == 1 {
            buffer[index] = code_point as u16;
        } else {
            buffer[index] =
                Self::decompose_high(code_point).expect("supplementary code point has high");
            buffer[index + 1] =
                Self::decompose_low(code_point).expect("supplementary code point has low");
        }
        Ok(count)
    }

    /// Escapes a scalar value as Java/JavaScript UTF-16 `\uXXXX` escape text.
    #[must_use]
    pub fn escape(code_point: u32) -> Option<String> {
        let count = Self::code_unit_count(code_point)?;
        if count == 1 {
            Some(format!("\\u{code_point:04X}"))
        } else {
            let high = Self::decompose_high(code_point)?;
            let low = Self::decompose_low(code_point)?;
            Some(format!("\\u{high:04X}\\u{low:04X}"))
        }
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

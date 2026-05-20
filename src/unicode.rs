/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
/// Namespace for raw Unicode code point helpers.
pub enum Unicode {}

impl Unicode {
    /// Maximum valid ASCII code point.
    pub const ASCII_MAX: u32 = 0x7f;

    /// Maximum valid Latin-1 code point.
    pub const LATIN1_MAX: u32 = 0xff;

    /// Maximum valid Unicode code point.
    pub const UNICODE_MAX: u32 = 0x10ffff;

    /// Minimum supplementary code point.
    pub const SUPPLEMENTARY_MIN: u32 = 0x10000;

    /// Minimum high-surrogate code unit value.
    pub const HIGH_SURROGATE_MIN: u16 = 0xd800;

    /// Maximum high-surrogate code unit value.
    pub const HIGH_SURROGATE_MAX: u16 = 0xdbff;

    /// Minimum low-surrogate code unit value.
    pub const LOW_SURROGATE_MIN: u16 = 0xdc00;

    /// Maximum low-surrogate code unit value.
    pub const LOW_SURROGATE_MAX: u16 = 0xdfff;

    /// Minimum surrogate code unit value.
    pub const SURROGATE_MIN: u16 = Self::HIGH_SURROGATE_MIN;

    /// Maximum surrogate code unit value.
    pub const SURROGATE_MAX: u16 = Self::LOW_SURROGATE_MAX;

    /// Number of bits shifted when composing or decomposing surrogate pairs.
    pub const HIGH_SURROGATE_SHIFT: u32 = 10;

    /// Mask used to decompose the low surrogate payload.
    pub const SURROGATE_DECOMPOSE_MASK: u32 = (1 << Self::HIGH_SURROGATE_SHIFT) - 1;

    /// Number of bits shifted to obtain a Unicode plane.
    pub const PLANE_SHIFT: u32 = 16;

    /// Returns `true` if the value is a valid ASCII code point.
    #[must_use]
    pub const fn is_valid_ascii(code_point: i32) -> bool {
        code_point >= 0 && (code_point as u32) <= Self::ASCII_MAX
    }

    /// Returns `true` if the value is a valid Latin-1 code point.
    #[must_use]
    pub const fn is_valid_latin1(code_point: i32) -> bool {
        code_point >= 0 && (code_point as u32) <= Self::LATIN1_MAX
    }

    /// Returns `true` if the value is in the Unicode code point range.
    #[must_use]
    pub const fn is_valid_unicode(code_point: i32) -> bool {
        code_point >= 0 && (code_point as u32) <= Self::UNICODE_MAX
    }

    /// Returns `true` if the value is in the basic multilingual plane.
    #[must_use]
    pub const fn is_bmp(code_point: i32) -> bool {
        code_point >= 0 && (code_point as u32) < Self::SUPPLEMENTARY_MIN
    }

    /// Returns `true` if the value is a supplementary Unicode code point.
    #[must_use]
    pub const fn is_supplementary(code_point: u32) -> bool {
        code_point >= Self::SUPPLEMENTARY_MIN && code_point <= Self::UNICODE_MAX
    }

    /// Returns `true` if the value is a UTF-16 high surrogate.
    #[must_use]
    pub const fn is_high_surrogate(code_point: i32) -> bool {
        code_point >= Self::HIGH_SURROGATE_MIN as i32
            && code_point <= Self::HIGH_SURROGATE_MAX as i32
    }

    /// Returns `true` if the value is a UTF-16 low surrogate.
    #[must_use]
    pub const fn is_low_surrogate(code_point: i32) -> bool {
        code_point >= Self::LOW_SURROGATE_MIN as i32 && code_point <= Self::LOW_SURROGATE_MAX as i32
    }

    /// Returns `true` if the value is any UTF-16 surrogate.
    #[must_use]
    pub const fn is_surrogate(code_point: i32) -> bool {
        code_point >= Self::SURROGATE_MIN as i32 && code_point <= Self::SURROGATE_MAX as i32
    }

    /// Returns `true` if the two code units form a valid UTF-16 surrogate pair.
    #[must_use]
    pub const fn is_surrogate_pair(high: u16, low: u16) -> bool {
        Self::is_high_surrogate(high as i32) && Self::is_low_surrogate(low as i32)
    }

    /// Composes a UTF-16 surrogate pair into a Unicode code point.
    #[must_use]
    pub const fn compose_surrogate_pair(high: u16, low: u16) -> Option<u32> {
        if Self::is_surrogate_pair(high, low) {
            let high_payload = (high as u32) - (Self::HIGH_SURROGATE_MIN as u32);
            let low_payload = (low as u32) - (Self::LOW_SURROGATE_MIN as u32);
            Some(
                (high_payload << Self::HIGH_SURROGATE_SHIFT)
                    + low_payload
                    + Self::SUPPLEMENTARY_MIN,
            )
        } else {
            None
        }
    }

    /// Decomposes a supplementary code point into its high surrogate.
    #[must_use]
    pub const fn decompose_high_surrogate(code_point: u32) -> Option<u16> {
        if Self::is_supplementary(code_point) {
            Some(
                (((code_point - Self::SUPPLEMENTARY_MIN) >> Self::HIGH_SURROGATE_SHIFT)
                    + Self::HIGH_SURROGATE_MIN as u32) as u16,
            )
        } else {
            None
        }
    }

    /// Decomposes a supplementary code point into its low surrogate.
    #[must_use]
    pub const fn decompose_low_surrogate(code_point: u32) -> Option<u16> {
        if Self::is_supplementary(code_point) {
            Some(
                (((code_point - Self::SUPPLEMENTARY_MIN) & Self::SURROGATE_DECOMPOSE_MASK)
                    + Self::LOW_SURROGATE_MIN as u32) as u16,
            )
        } else {
            None
        }
    }

    /// Returns the Unicode plane containing the code point.
    #[must_use]
    pub const fn plane(code_point: u32) -> Option<u32> {
        if code_point <= Self::UNICODE_MAX {
            Some(code_point >> Self::PLANE_SHIFT)
        } else {
            None
        }
    }

    /// Escapes a code point as a Java-style Unicode escape.
    #[must_use]
    pub fn escape(code_point: u32) -> Option<String> {
        if code_point > Self::UNICODE_MAX {
            None
        } else if code_point > 0xffff {
            Some(format!("\\u{code_point:X}"))
        } else {
            Some(format!("\\u{code_point:04X}"))
        }
    }
}

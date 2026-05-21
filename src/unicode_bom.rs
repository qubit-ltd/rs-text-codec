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
    TextEncoding,
};

/// Unicode byte order marks supported by this crate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UnicodeBom {
    /// UTF-8 byte order mark.
    Utf8,

    /// UTF-16 big-endian byte order mark.
    Utf16BigEndian,

    /// UTF-16 little-endian byte order mark.
    Utf16LittleEndian,

    /// UTF-32 big-endian byte order mark.
    Utf32BigEndian,

    /// UTF-32 little-endian byte order mark.
    Utf32LittleEndian,
}

impl UnicodeBom {
    /// Detects a Unicode byte order mark at the beginning of `bytes`.
    ///
    /// # Parameters
    ///
    /// - `bytes`: The byte buffer to inspect.
    ///
    /// # Returns
    ///
    /// Returns the detected BOM, or `None` if no supported BOM prefix is present.
    #[must_use]
    pub fn detect(bytes: &[u8]) -> Option<Self> {
        if bytes.starts_with(&[0x00, 0x00, 0xfe, 0xff]) {
            Some(Self::Utf32BigEndian)
        } else if bytes.starts_with(&[0xff, 0xfe, 0x00, 0x00]) {
            Some(Self::Utf32LittleEndian)
        } else if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
            Some(Self::Utf8)
        } else if bytes.starts_with(&[0xfe, 0xff]) {
            Some(Self::Utf16BigEndian)
        } else if bytes.starts_with(&[0xff, 0xfe]) {
            Some(Self::Utf16LittleEndian)
        } else {
            None
        }
    }

    /// Returns the bytes that represent this BOM.
    ///
    /// # Returns
    ///
    /// Returns a static byte slice containing the BOM bytes.
    #[must_use]
    pub const fn bytes(self) -> &'static [u8] {
        match self {
            Self::Utf8 => &[0xef, 0xbb, 0xbf],
            Self::Utf16BigEndian => &[0xfe, 0xff],
            Self::Utf16LittleEndian => &[0xff, 0xfe],
            Self::Utf32BigEndian => &[0x00, 0x00, 0xfe, 0xff],
            Self::Utf32LittleEndian => &[0xff, 0xfe, 0x00, 0x00],
        }
    }

    /// Returns the byte length of this BOM.
    ///
    /// # Returns
    ///
    /// Returns the number of bytes in this BOM.
    #[must_use]
    pub const fn byte_len(self) -> usize {
        match self {
            Self::Utf8 => 3,
            Self::Utf16BigEndian | Self::Utf16LittleEndian => 2,
            Self::Utf32BigEndian | Self::Utf32LittleEndian => 4,
        }
    }

    /// Returns the Unicode encoding indicated by this BOM.
    ///
    /// # Returns
    ///
    /// Returns the corresponding [`TextEncoding`].
    #[must_use]
    pub const fn encoding(self) -> TextEncoding {
        match self {
            Self::Utf8 => TextEncoding::Utf8,
            Self::Utf16BigEndian | Self::Utf16LittleEndian => TextEncoding::Utf16,
            Self::Utf32BigEndian | Self::Utf32LittleEndian => TextEncoding::Utf32,
        }
    }

    /// Returns the byte order indicated by this BOM when applicable.
    ///
    /// # Returns
    ///
    /// Returns `Some(ByteOrder)` for UTF-16 and UTF-32 BOMs. Returns `None` for
    /// UTF-8 because byte order does not apply.
    #[must_use]
    pub const fn byte_order(self) -> Option<ByteOrder> {
        match self {
            Self::Utf8 => None,
            Self::Utf16BigEndian | Self::Utf32BigEndian => Some(ByteOrder::BigEndian),
            Self::Utf16LittleEndian | Self::Utf32LittleEndian => Some(ByteOrder::LittleEndian),
        }
    }
}

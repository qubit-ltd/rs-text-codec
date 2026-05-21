/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use core::fmt;

/// Identifies the text encoding associated with a codec or error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TextEncoding {
    /// US-ASCII text.
    Ascii,

    /// UTF-8 text.
    Utf8,

    /// UTF-16 text.
    Utf16,

    /// UTF-32 text.
    Utf32,

    /// A named encoding outside the built-in Unicode codecs.
    Named(&'static str),
}

impl TextEncoding {
    /// Returns a human-readable encoding label.
    ///
    /// # Returns
    ///
    /// Returns the canonical built-in label or the name stored in
    /// [`TextEncoding::Named`].
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Ascii => "ASCII",
            Self::Utf8 => "UTF-8",
            Self::Utf16 => "UTF-16",
            Self::Utf32 => "UTF-32",
            Self::Named(name) => name,
        }
    }
}

impl fmt::Display for TextEncoding {
    /// Formats this encoding label.
    ///
    /// # Parameters
    ///
    /// - `formatter`: The formatter receiving the label.
    ///
    /// # Errors
    ///
    /// Returns any formatting error reported by `formatter`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

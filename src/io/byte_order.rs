/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
/// Byte order used when interpreting multi-byte binary values.
///
/// `ByteOrder` is intentionally only a configuration enum. Buffer-oriented
/// read and write operations live on [`crate::BinaryCodec`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ByteOrder {
    /// Most significant byte first.
    BigEndian,

    /// Least significant byte first.
    LittleEndian,
}

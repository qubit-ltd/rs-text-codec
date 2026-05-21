/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use super::{
    Decoded,
    NeedMore,
};

/// Result of attempting to decode the first character from a buffer prefix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DecodeResult<T> {
    /// A complete value was decoded from the prefix.
    Complete(Decoded<T>),

    /// The current prefix is well-formed so far but incomplete.
    NeedMore(NeedMore),
}

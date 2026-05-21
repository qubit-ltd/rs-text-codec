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
    TextDecoder,
    TextEncoder,
};

/// Combined text encoder and decoder for the same storage unit type.
pub trait TextCodec<T>: TextEncoder<T> + TextDecoder<T> {}

impl<T, C> TextCodec<T> for C where C: TextEncoder<T> + TextDecoder<T> {}

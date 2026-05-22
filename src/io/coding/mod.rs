/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0.
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Generic, charset-independent coding traits and status types.

mod binary_codec;
mod coder;
mod coder_progress;
mod coder_status;

pub use binary_codec::BinaryCodec;
pub use coder::Coder;
pub use coder_progress::CoderProgress;
pub use coder_status::CoderStatus;

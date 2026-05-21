/*******************************************************************************
 *
 *    Copyright (c) 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0.
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
mod byte_order;
mod text_encoding;
mod unicode_bom;

pub use byte_order::ByteOrder;
pub use text_encoding::TextEncoding;
pub use unicode_bom::UnicodeBom;

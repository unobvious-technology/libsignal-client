//
// Copyright 2020-2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

#![allow(clippy::missing_safety_doc)]
#![deny(clippy::unwrap_used)]

#[cfg(not(any(feature = "ffi", feature = "jni", feature = "node", feature = "wasm")))]
compile_error!("Feature \"ffi\", \"jni\", \"node\", or \"wasm\" must be enabled for this crate.");

#[cfg(feature = "ffi")]
#[macro_use]
pub mod ffi;

#[cfg(feature = "jni")]
#[macro_use]
pub mod jni;

#[cfg(feature = "node")]
#[macro_use]
pub mod node;

#[cfg(feature = "wasm")]
#[macro_use]
pub mod wasm;

#[macro_use]
mod support;

pub mod crypto;
pub mod protocol;

// Desktop does not make use of device transfer certificates
#[cfg(any(feature = "jni", feature = "ffi"))]
pub mod device_transfer;

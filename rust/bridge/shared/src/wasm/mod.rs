//
// Copyright 2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use wasm_bindgen::prelude::*;
// use libsignal_protocol::*;
// use std::convert::TryFrom;
// use std::ops::Deref;

// macro_rules! wasm_register {
//     ( $name:ident ) => {
//         paste! {
//             #[no_mangle] // necessary because we are linking as a cdylib
//             #[allow(non_upper_case_globals)]
//             #[linkme::distributed_slice(crate::wasm::LIBSIGNAL_FNS)]
//         }
//     };
// }

#[macro_use]
mod convert;
pub use convert::*;

// /// A function pointer referring to a Neon-based Node entry point.
// #[doc(hidden)]
// pub(crate) type JsFn = for<'a> fn(FunctionContext<'a>) -> Result<'a, JsValue>;

// #[doc(hidden)]
// #[linkme::distributed_slice]
// pub(crate) static LIBSIGNAL_FNS: [(&'static str, JsFn)] = [..];

/// Implementation of [`bridge_deserialize`](crate::support::bridge_deserialize) for Node.
macro_rules! wasm_bridge_deserialize {
    ( $typ:ident::$fn:path as false ) => {};
    ( $typ:ident::$fn:path as $wasm_name:ident ) => {
        paste! {
            #[allow(non_snake_case, clippy::redundant_closure)]
            #[doc = "ts: export function " $wasm_name "_Deserialize(buffer: Buffer): " $typ]
            pub fn [<wasm_ $wasm_name _Deserialize>](
                // mut cx: node::FunctionContext
            ) {
                // let buffer = cx.argument::<node::JsBuffer>(0)?;
                // let obj: Result<$typ> =
                //     node::with_buffer_contents(&mut cx, buffer, |buf| $typ::$fn(buf));
                // match obj {
                //     Ok(obj) => node::ResultTypeInfo::convert_into(obj, &mut cx),
                //     Err(err) => {
                //         let module = cx.this();
                //         node::SignalNodeError::throw(
                //             err,
                //             &mut cx,
                //             module,
                //             stringify!([<$node_name "_Deserialize">]))
                //     }
                // }
            }

            // wasm_register!([<$node_name _Deserialize>]);
        }
    };
    ( $typ:ident::$fn:path ) => {
        wasm_bridge_deserialize!($typ::$fn as $typ);
    };
}

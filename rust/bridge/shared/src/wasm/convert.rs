//
// Copyright 2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//
// use paste::paste;
// use super::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::*;

/// Converts result values from their Rust form to their JavaScript form.
///
/// `ResultTypeInfo` is used to implement the `bridge_fn` macro, but can also be used outside it.
///
/// ```no_run
/// # use libsignal_bridge::node::*;
/// # use neon::prelude::*;
/// # struct Foo;
/// # impl<'a> ResultTypeInfo<'a> for Foo {
/// #     type ResultType = JsNumber;
/// #     fn convert_into(self, _cx: &mut impl Context<'a>) -> JsResult<'a, Self::ResultType> {
/// #         unimplemented!()
/// #     }
/// # }
/// # fn test(mut cx: FunctionContext) -> NeonResult<()> {
/// #     let rust_result = Foo;
/// let js_result = rust_result.convert_into(&mut cx)?;
/// #     Ok(())
/// # }
/// ```
///
/// Implementers should also see the `jni_result_type` macro in `convert.rs`.
// pub trait ResultTypeInfo<'a>: Sized {
//     /// The JavaScript form of the result (e.g. `JsNumber`).
//     type ResultType: wasm_bindgen::JsValue;
//     /// Converts the data in `self` to the JavaScript type, similar to `try_into()`.
//     fn convert_into(self, cx: &mut impl Context<'a>) -> Result<'a, Self::ResultType>;
// }

// impl<'a> ResultTypeInfo<'a> for bool {
//     type ResultType = bool;
//     fn convert_into(self, cx: &mut impl Context<'a>) -> NeonResult<Handle<'a, Self::ResultType>> {
//         Ok(cx.boolean(self))
//     }
// }

/// Implementation of [`bridge_handle`](crate::support::bridge_handle) for Node.
macro_rules! wasm_bridge_handle {
    ( $typ:ty as false $(, $($_:tt)*)? ) => {};
    ( $typ:ty as $wasm_name:ident ) => {
        paste! {
            // #[wasm_bindgen]
            // pub struct $wasm_name {}
            // #[doc = "ts: interface " $typ " { readonly __type: unique symbol; }"]
            // impl<'a> for $typ {
            //     type ResultType = wasm_bindgen::JsValue;
            //     // fn convert_into(
            //     //     self,
            //     //     cx: &mut impl node::Context<'a>,
            //     // ) -> node::NeonResult<node::Handle<'a, Self::ResultType>> {
            //     //     node::return_boxed_object(cx, Ok(self))
            //     // }
            // }
        }
    };
    ( $typ:ty as $wasm_name:ident, mut = true ) => {
        paste! {
            // #[no_mangle]
            // #[wasm_bindgen]
            // pub struct [<Wasm $wasm_name>] {}
            // #[doc = "ts: interface " $typ " { readonly __type: unique symbol; }"]
            // impl<'a> for $typ {
            // //     type ResultType = node::JsValue;
            // //     fn convert_into(
            // //         self,
            // //         cx: &mut impl node::Context<'a>,
            // //     ) -> node::NeonResult<node::Handle<'a, Self::ResultType>> {
            // //         node::return_boxed_object(cx, Ok(std::cell::RefCell::new(self)))
            // //     }
            // }
        }
    };
    ( $typ:ty $(, mut = $_:tt)?) => {
        paste! {
            wasm_bridge_handle!($typ as $typ $(, mut = $_)?);
        }
    };
}

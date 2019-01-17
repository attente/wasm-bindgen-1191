#[cfg(feature = "bindings")]
extern crate wasm_bindgen;
#[cfg(feature = "bindings")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "bindings", wasm_bindgen)]
pub struct Foo {
}

#[cfg_attr(feature = "bindings", wasm_bindgen)]
impl Foo {
    #[cfg_attr(feature = "bindings", wasm_bindgen(constructor))]
    pub fn new() -> Foo {
        Foo {}
    }
}

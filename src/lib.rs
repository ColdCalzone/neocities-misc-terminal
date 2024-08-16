#![allow(dead_code)]
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod terminal;
mod utils;

#[cfg(test)]
mod tests;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn greet() {}

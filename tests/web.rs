//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate hello;
use hello::Accumulator;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
pub fn test_add() {
    let mut acc = Accumulator {current: 0};
    acc.add(2);
    acc.add(3);
    assert_eq!(acc.current, 5);
}
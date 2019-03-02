#![allow(unused_imports)]
#![allow(unused_variables)]
#![cfg(target_arch = "wasm32")]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate bellman;
extern crate pairing;
extern crate rand;
extern crate ff;

mod utils;
mod cube;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct Accumulator {
    pub current: u32
}

#[wasm_bindgen]
impl Accumulator {
    pub fn add(&mut self, num: u32) -> u32 {
    	self.current += num;
    	self.current
    }
}
#![allow(unused_imports)]
#![allow(unused_variables)]
#![cfg(target_arch = "wasm32")]

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate bellman;
extern crate sapling_crypto;
extern crate rand;
extern crate web_sys;

mod utils;
mod hasher;
mod bit_iterator;
mod pedersen;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate sudoku_wasm;

use wasm_bindgen_test::*;

use sudoku_wasm::Soduku;

wasm_bindgen_test_configure!(run_in_browser);


#[cfg(test)]
pub fn dd()  {
    let mut soduku = Soduku::new();


}


#[wasm_bindgen_test]
fn pass() {
    let soduku = Soduku::new();


    
    println!("{:?}", soduku.render());

    assert_eq!(1 + 1, 2);



    assert_eq!(1 + 1, 3);
}

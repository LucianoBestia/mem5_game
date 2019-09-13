//! jaascriptimport.rs - one single module to import javascript functions and objects
//! this are examples how to call a javascript function from rust

use wasm_bindgen::prelude::*;

///in the block extern "C" are the descriptions of imported javascript
#[wasm_bindgen(module = "/js/javascriptdofullscreen.js")]
extern "C" {
    fn javascriptdofullscreen();

}
///do full screen function imported from javascript
pub fn do_fullscreen() {
    javascriptdofullscreen();
}


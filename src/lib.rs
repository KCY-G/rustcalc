mod calc;
mod keys;
mod save;

use calc::Body;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Body>::new().mount_to_body();
}

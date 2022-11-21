use wasm_bindgen::prelude::wasm_bindgen;
// use wasm_bindgen::prelude::wasm_bindgen;

use base64::{decode, encode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
// use web_sys::console::log_1 as log; 1 => will log 1 value, so on

// Macro attribute => a feature for running macros functions => #[macro_name]
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    let base64_to_vector = decode(encoded_file).unwrap();

    let mut image = load_from_memory(&base64_to_vector).unwrap();

    image = image.grayscale();

    let mut buffer = vec![];

    image.write_to(&mut buffer, Png).unwrap();

    let encoded_image = encode(&mut buffer);

    let data_url = format!("data:image/png;base64,{}", encoded_image);

    data_url
}

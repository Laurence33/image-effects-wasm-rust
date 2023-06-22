use base64::decode;
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&"Grayscale called".into());
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());
    let mut image = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());
    image = image.grayscale();
    log(&"Grayscale applied".into());

    let mut buffer = vec![];
    image.write_to(&mut buffer, Png).unwrap();

    log(&"Image written to buffer".into());
}

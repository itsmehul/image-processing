use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;

// Publicize fx to JS
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&encoded_file.into());

    // base64 to binary
    let base64_to_vec = general_purpose::STANDARD.decode(encoded_file).unwrap();

    log(&"Image decoded".into());

    // binary to dynamic image
    let mut img = load_from_memory(&base64_to_vec).unwrap();

    log(&"Image loaded".into());

    img = img.grayscale();

    log(&"Image converted to grayscale".into());

    let buff = vec![];
    let mut cursor = Cursor::new(buff);

    // dynamic image to png
    img.write_to(&mut cursor, Png).unwrap();

    log(&"Image converted to PNG".into());

    let encoded_image = general_purpose::STANDARD.encode(&cursor.into_inner());
    let data_url = format!("data:image/png;base64,{}", encoded_image);

    data_url
}

#[cfg(test)]
mod tests {}

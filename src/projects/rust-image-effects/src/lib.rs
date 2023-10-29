use base64::{decode, encode};
use image::load_from_memory;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn gray_scale(encoded_file: &str) -> String {
    // log(&encoded_file.into());

    log(&"Gray Scale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut image = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    image = image.grayscale();
    log(&"Gray Scale applied".into());

    let mut buffer = vec![];
    image.write_to(&mut buffer, Png).unwrap();
    log(&"New image created".into());

    let encoded_img = encode(&buffer);
    log(&"New image encoded".into());

    let data_url = format!("data:image/png;base64,{}", encoded_img);

    data_url
}

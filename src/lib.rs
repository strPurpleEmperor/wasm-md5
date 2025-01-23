use md5::{Md5, Digest};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn calculate_md5(data: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[wasm_bindgen]
pub fn calculate_md5_from_chunks(chunk_iter: js_sys::Array) -> String {
    let mut hasher = Md5::new();
    for chunk in chunk_iter.iter() {
        if let Ok(chunk_bytes) = chunk.dyn_into::<js_sys::Uint8Array>() {
            let chunk_vec = chunk_bytes.to_vec();
            hasher.update(&chunk_vec);
        } else {
            panic!("Invalid chunk type")
        }
    }
    let result = hasher.finalize();
    format!("{:x}", result)
}

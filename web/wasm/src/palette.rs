use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub struct Palette {
    pub active: usize,
    pub colors: Vec<u8>,
}

impl Palette {
    pub fn new() -> Palette {
        let mut colors = Vec::with_capacity(256 * 3);
        for _ in 0..768 {
            colors.push(255);
        }
        Palette { active: 0, colors }
    }
}

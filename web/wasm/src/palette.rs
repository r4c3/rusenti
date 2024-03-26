use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Palette {
    active: u8,
    colors: [u8; 256 * 3],
}

#[wasm_bindgen]
impl Palette {
    pub fn new() -> Palette {
        Palette {
            active: 0,
            colors: [0; 256 * 3],
        }
    }

    /// Set a color locally in the Palette struct.
    /// Use the exported set_color function to call set_color_naive, update bitmap mappings, and
    /// force re-render.
    #[wasm_bindgen]
    pub fn set_color_naive(&mut self, index: u8, r: u8, g: u8, b: u8) {
        let index = index as usize;
        self.colors[index * 3] = r;
        self.colors[index * 3 + 1] = g;
        self.colors[index * 3 + 2] = b;
    }

    /// Set the color actively selected by the pen tool.
    #[wasm_bindgen]
    pub fn set_active(&mut self, index: u8) {
        self.active = index;
    }
}

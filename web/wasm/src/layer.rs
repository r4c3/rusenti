use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Layer {
    pixels: Vec<u8>,
    opacity: u8,
    visible: bool,
    size: (u16, u16),
    offset: (u16, u16),
}

#[wasm_bindgen]
impl Layer {
    pub fn new(width: u16, height: u16) -> Layer {
        Layer {
            pixels: vec![0; (width * height * 3) as usize],
            opacity: 255,
            visible: true,
            size: (width, height),
            offset: (
                8192 / 2 - (width / 2) as u16,
                8192 / 2 - (height / 2) as u16,
            ),
        }
    }

    pub fn set_pixel(&mut self, pixel_index: usize, palette_index: u8) {
        self.pixels[pixel_index] = palette_index;
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }

    pub fn set_opacity(&mut self, opacity: u8) {
        self.opacity = opacity;
    }

    pub fn get_opacity(&self) -> u8 {
        self.opacity
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn get_pixels(&self) -> *const u8 {
        self.pixels.as_ptr()
    }
}

#[wasm_bindgen]
pub struct LayerManager {
    layers: Vec<Layer>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl LayerManager {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> LayerManager {
        LayerManager {
            layers: Vec::new(),
            width,
            height,
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn remove_layer(&mut self, index: usize) {
        self.layers.remove(index);
    }

    pub fn get_layer_data(&self, index: usize) -> Option<Uint8Array> {
        self.layers.get(index).map(|layer| {
            let pixels = layer.pixels.clone();
            Uint8Array::from(pixels.as_slice())
        })
    }

    pub fn set_layer_data(&mut self, index: usize, data: Uint8Array) {
        if let Some(layer) = self.layers.get_mut(index) {
            layer.pixels = data.to_vec();
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

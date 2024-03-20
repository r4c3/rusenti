use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}

struct Viewport {
    offset: (f64, f64),
    zoom: f64,
}

#[wasm_bindgen]
pub struct World {
    html_canvas: HtmlCanvasElement,
    render_context: CanvasRenderingContext2d,
    viewport: Viewport,
    layer_manger: LayerManager,
}

#[wasm_bindgen]
impl World {}

#[wasm_bindgen]
pub struct Layer {
    pixels: Vec<u8>,
    width: u16,
    height: u16,
    opacity: u8,
    visible: bool,
}

#[wasm_bindgen]
impl Layer {
    pub fn new(width: u16, height: u16) -> Layer {
        Layer {
            pixels: vec![0; (width * height * 4) as usize],
            width,
            height,
            opacity: 255,
            visible: true,
        }
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, r: u8, g: u8, b: u8, a: u8) {
        let index = (y * self.width + x) as usize * 4;
        self.pixels[index] = r;
        self.pixels[index + 1] = g;
        self.pixels[index + 2] = b;
        self.pixels[index + 3] = a;
    }

    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
}

#[wasm_bindgen]
pub struct LayerManager {
    layers: Vec<Layer>,
}

#[wasm_bindgen]
impl LayerManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> LayerManager {
        LayerManager { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn remove_layer(&mut self, index: usize) {
        self.layers.remove(index);
    }
}

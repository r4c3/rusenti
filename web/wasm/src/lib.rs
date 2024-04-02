mod layer;
mod palette;
mod viewport;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub struct World {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    viewport: viewport::Viewport,
    layer_manager: layer::LayerManager,
    palette: palette::Palette,
    rendering: bool,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(bitmap_width: i32, bitmap_height: i32) -> Result<World, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let mut world = World {
            canvas,
            context,
            viewport: viewport::Viewport::new(),
            layer_manager: layer::LayerManager::new(bitmap_width, bitmap_height),
            palette: palette::Palette::new(),
            rendering: false,
        };

        world.layer_manager.add_layer();

        Ok(world)
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        if self.rendering {
            return;
        }
        let mut prev_r = 0;
        let mut prev_g = 0;
        let mut prev_b = 0;
        self.rendering = true;
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
        for x in 0..(self.layer_manager.width) {
            for y in 0..(self.layer_manager.height) {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                for layer in &self.layer_manager.layers {
                    if !layer.visible {
                        continue;
                    }

                    let layer_x = x - layer.offset_x;
                    let layer_y = y - layer.offset_y;

                    // skip if out of bounds
                    if layer_x < 0 || layer_x > layer.width || layer_y < 0 || layer_y > layer.height
                    {
                        continue;
                    }

                    let pixel_index = (layer_x + layer_y * layer.width) as usize;
                    let palette_index = layer.pixels[pixel_index];
                    let layer_r = self.palette.colors[palette_index as usize * 3];
                    let layer_g = self.palette.colors[palette_index as usize * 3 + 1];
                    let layer_b = self.palette.colors[palette_index as usize * 3 + 2];

                    match layer.opacity {
                        0 => continue,
                        255 => {
                            r = layer_r;
                            g = layer_g;
                            b = layer_b;
                        }
                        a => {
                            r = (a * (layer_r) + (255 - a) * r) as u8 / 255;
                            g = (a * (layer_b) + (255 - a) * g) as u8 / 255;
                            b = (a * (layer_b) + (255 - a) * b) as u8 / 255;
                        }
                    }
                }
                if r != prev_r || g != prev_g || b != prev_b {
                    self.context
                        .set_fill_style(&JsValue::from_str(&format!("rgb({}, {}, {})", r, g, b)));
                    prev_r = r;
                    prev_b = b;
                    prev_g = g;
                }
                self.context.fill_rect(
                    (x) as f64 * self.viewport.zoom + self.viewport.offset.0 as f64,
                    (y) as f64 * self.viewport.zoom + self.viewport.offset.1 as f64,
                    self.viewport.zoom,
                    self.viewport.zoom,
                );
            }
        }
        self.rendering = false;
    }

    #[wasm_bindgen]
    pub fn set_palette_color(&mut self, palette_index: usize, r: u8, g: u8, b: u8) {
        self.palette.colors[palette_index * 3] = r;
        self.palette.colors[palette_index * 3 + 1] = g;
        self.palette.colors[palette_index * 3 + 2] = b;
    }
    #[wasm_bindgen]
    pub fn set_palette_active(&mut self, palette_index: usize) {
        self.palette.active = palette_index;
    }

    #[wasm_bindgen]
    pub fn pan(&mut self, dx: i32, dy: i32) {
        self.viewport.offset.0 += dx;
        self.viewport.offset.1 += dy;
    }

    #[wasm_bindgen]
    pub fn zoom(&mut self, dz: f64) {
        self.viewport.zoom *= dz;
    }

    #[wasm_bindgen]
    pub fn color_pixel(&mut self, mut x: i32, mut y: i32) {
        x = x - self.viewport.offset.0;
        y = y - self.viewport.offset.1;
        x = ((x as f64) / self.viewport.zoom) as i32;
        y = ((y as f64) / self.viewport.zoom) as i32;
        let curr_layer: &mut layer::Layer = self
            .layer_manager
            .layers
            .get_mut(self.layer_manager.active_layer)
            .unwrap();
        let layer_x = x - curr_layer.offset_x;
        let layer_y = y - curr_layer.offset_y;
        if layer_x < 0 || layer_x > curr_layer.width || layer_y < 0 || layer_y > curr_layer.height {
            return;
        }
        let pixel_index = (layer_x + layer_y * curr_layer.width) as usize;
        curr_layer.pixels[pixel_index] = self.palette.active as u8;
        self.render();
    }
}

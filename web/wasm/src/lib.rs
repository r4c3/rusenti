mod layer;
mod palette;
mod viewport;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
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

    let world = World::new(context)?;

    Ok(())
}

#[wasm_bindgen]
pub struct World {
    context: CanvasRenderingContext2d,
    viewport: viewport::Viewport,
    layer_manager: layer::LayerManager,
    palette: palette::Palette,
}

#[wasm_bindgen]
impl World {
    #[wasm_bindgen(constructor)]
    pub fn new(context: CanvasRenderingContext2d) -> Result<World, JsValue> {
        Ok(World {
            context,
            viewport: viewport::Viewport::new(),
            layer_manager: layer::LayerManager::new(8192, 8192),
            palette: palette::Palette::new(),
        })
    }

    pub fn render(&self) {}
}

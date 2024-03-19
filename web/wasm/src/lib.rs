use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let viewfinder = Viewfinder::new("canvas")?;
    viewfinder.render();
    Ok(())
}

#[wasm_bindgen]
pub struct Viewfinder {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    viewport_x: f64, // X offset of the viewport in the world
    viewport_y: f64, // Y offset of the viewport in the world
    zoom: f64,       // Zoom level of the viewport
    pixels: Vec<u8>,
}

#[wasm_bindgen]
impl Viewfinder {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<Viewfinder, JsValue> {
        let window = window().ok_or("should have a window in this context")?;
        let document = window
            .document()
            .ok_or("should have a document on window")?;
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("canvas not found")?;
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")?
            .ok_or("context not found")?
            .dyn_into::<CanvasRenderingContext2d>()?;

        Ok(Viewfinder {
            canvas,
            context,
            world_width: 10000,
            world_height: 10000,
            viewport_x: 0.0,
            viewport_y: 0.0,
            zoom: 1.0,
            pixels: vec![0; 10000],
        })
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.viewport_x += dx;
        self.viewport_y += dy;
        self.render();
    }

    pub fn zoom(&mut self, factor: f64) {
        self.zoom *= factor;
        self.render();
    }
    pub fn color(&mut self, x: f64, y: f64) {
        let x_pixel = (x - self.viewport_x) / ((100 as f64) * self.zoom);
        let y_pixel = (y - self.viewport_y) / ((100 as f64) * self.zoom);
        if ((0 > (x_pixel as i32)) || (100 <= (x_pixel as i32))) {
            return;
        }
        if ((0 > (y_pixel as i32)) || (100 <= (y_pixel as i32))) {
            return;
        }
        self.pixels[(x_pixel as usize) + 100 * (y_pixel as usize)] = 1;
        self.render();
    }

    pub fn render(&self) {
        let canvas_width = self.canvas.width() as f64;
        let canvas_height = self.canvas.height() as f64;

        // Clear the canvas
        self.context
            .clear_rect(0.0, 0.0, canvas_width, canvas_height);

        // Adjust the context for the current viewport position and zoom
        self.context.save();
        self.context.scale(self.zoom, self.zoom).unwrap();
        self.context
            .translate(self.viewport_x / self.zoom, self.viewport_y / self.zoom)
            .unwrap();

        //draw grid
        self.context.set_fill_style(&JsValue::from_str("white"));
        for x in (0..100) {
            for y in (0..100) {
                let color: u8 = self.pixels[x + 100 * y];
                if (color != 0) {
                    self.context.set_fill_style(&JsValue::from_str("red"));
                } else {
                    self.context.set_fill_style(&JsValue::from_str("white"));
                }
                self.context
                    .fill_rect((x * 100) as f64, (y * 100) as f64, 50.0, 50.0);
            }
        }

        self.context.restore();
    }
}

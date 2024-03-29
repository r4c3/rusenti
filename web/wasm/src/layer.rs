pub struct Layer {
    pub pixels: Vec<u8>,
    pub opacity: u8,
    pub visible: bool,
    pub width: i32,
    pub height: i32,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl Layer {
    fn new(width: i32, height: i32) -> Layer {
        Layer {
            pixels: vec![0; (width * height) as usize],
            opacity: 255,
            visible: true,
            width,
            height,
            offset_x: 0,
            offset_y: 0
        }
    }
}

pub struct LayerManager {
    pub layers: Vec<Layer>,
    pub width: i32,
    pub height: i32,
}

impl LayerManager {
    pub fn new(width: i32, height: i32) -> LayerManager {
        LayerManager {
            layers: Vec::new(),
            width,
            height,
        }
    }

    pub fn add_layer(&mut self) {
        self.layers.push(Layer::new(self.width, self.height));
    }
}

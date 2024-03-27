pub struct Viewport {
    offset: (f64, f64),
    zoom: f64,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {
            offset: (0.0, 0.0),
            zoom: 1.0,
        }
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.offset.0 += dx;
        self.offset.1 += dy;
    }

    pub fn zoom(&mut self, dz: f64) {
        self.zoom *= dz;
    }
}

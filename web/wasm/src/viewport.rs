pub struct Viewport {
    pub offset: (i32, i32),
    pub zoom: f64,
}

impl Viewport {
    pub fn new() -> Viewport {
        Viewport {
            offset: (0, 0),
            zoom: 1.0,
        }
    }
}

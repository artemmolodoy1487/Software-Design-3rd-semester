#[derive(Debug)]
pub struct Scope {
    zoom: u32,
}

impl Scope {
    pub fn new(zoom: u32) -> Self {
        Scope { zoom }
    }

    pub fn get_x_zoom(&self) -> u32 {
        self.zoom
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridRegion {

    min_x: u32,
    min_y: u32,
    width: u32,
    height: u32
}

impl GridRegion {

    pub fn new(min_x: u32, min_y: u32, width: u32, height: u32) -> Self {
        Self { min_x, min_y, width, height }
    }

    pub fn get_min_x(&self) -> u32 {
        self.min_x
    }

    pub fn get_min_y(&self) -> u32 {
        self.min_y
    }

    pub fn get_max_x(&self) -> u32 {
        self.min_x + self.width - 1
    }

    pub fn get_max_y(&self) -> u32 {
        self.min_y + self.height - 1
    }

    pub fn get_bound_x(&self) -> u32 {
        self.min_x + self.width
    }

    pub fn get_bound_y(&self) -> u32 {
        self.min_y + self.height
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}
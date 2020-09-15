/// This struct describes a rectangular region in a *ColorGrid* or a
/// *DepthStencilGrid*. It is just a pair of (x,y,width,height) and
/// doesn't store any data about any grid.
/// 
/// TODO Add more use cases for this struct (besides debug dumps)
#[derive(Clone, Copy, Debug)]
pub struct GridRegion {
    min_x: u32,
    min_y: u32,
    width: u32,
    height: u32,
}

impl GridRegion {
    pub fn new(min_x: u32, min_y: u32, width: u32, height: u32) -> Self {
        Self {
            min_x,
            min_y,
            width,
            height,
        }
    }

    /// Gets the smallest x-coordinate that is part of the region described by
    /// this *GridRegion*.
    pub fn get_min_x(&self) -> u32 {
        self.min_x
    }

    /// Gets the smallest y-coordinate that is part of the region described by
    /// this *GridRegion*.
    pub fn get_min_y(&self) -> u32 {
        self.min_y
    }

    /// Gets the largest x-coordinate that is part of the region described by
    /// this *GridRegion*.
    pub fn get_max_x(&self) -> u32 {
        self.min_x + self.width - 1
    }

    /// Gets the largest y-coordinate that is part of the region described by
    /// this *GridRegion*.
    pub fn get_max_y(&self) -> u32 {
        self.min_y + self.height - 1
    }

    /// Gets the x-coordinate such that x - 1 is part of the region described by
    /// this *GridRegion*, but x is not.
    pub fn get_bound_x(&self) -> u32 {
        self.min_x + self.width
    }

    /// Gets the y-coordinate such that y - 1 is part of the region described by
    /// this *GridRegion*, but y is not.
    pub fn get_bound_y(&self) -> u32 {
        self.min_y + self.height
    }

    /// Gets the width of this region (the number of columns).
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Gets the height of this region (the number of rows).
    pub fn get_height(&self) -> u32 {
        self.height
    }
}

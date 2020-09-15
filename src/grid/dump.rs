/// For debugging purposes, it can be very useful to see what a *ColorGrid*
/// currently looks like. This struct represents a copy of the state of a
/// color grid, made at a given moment. 
/// 
/// Unlike color grids, these dumps are stored in RAM, which means the 
/// individual colors in the grid can be read easily (and can thus be used
/// for debugging).
/// 
/// *Note: sending data from GPU memory to RAM is often very expensive, so
/// this shouldn't be used every frame! Luckily, performance is normally not
/// important during debugging.*
pub struct ColorGridDump {
    data: Vec<Color>,

    width: u32,
}

/// Represents a single color in a *ColorGrid(Dump)*. This is a simple struct
/// with just the red, green, blue, and alpha components of the color in range
/// from 0 to 255.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl ColorGridDump {

    /// Constructs a new *ColorGridDump* from a raw *Vec* of *Color*s with given
    /// width and height. The product of *width* and *height* must be equal to the
    /// length of *data*. 
    /// 
    /// This method should normally only be used by Griphin implementations.
    pub fn new(data: Vec<Color>, width: u32, height: u32) -> Self {
        if width * height != data.len() as u32 {
            panic!(
                "Width * height must be equal to the size of data, but it's not: {} * {} != {}",
                width,
                height,
                data.len()
            );
        }

        Self { data, width }
    }

    /// Gets the *Color* at the position (x,y) in this *ColorGridDump*. The left
    /// border has an x-coordinate of 0 and the top border has a y-coordinate of 0. 
    /// The right border has an x-coordinate of *self.get_width() - 1* and the
    /// bottom border has a y-coordinate of *self.get_height() - 1*.
    pub fn get(&self, x: u32, y: u32) -> Color {
        self.data[(x + y * self.width) as usize]
    }

    /// Gets the width of this *ColorGridDump* (and the associated *ColorGrid*).
    /// That is, the number of columns in the grid.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Gets the height of this *ColorGridDump* (and the associated *ColorGrid*). 
    /// That is, the number of rows in the grid.
    pub fn get_height(&self) -> u32 {
        self.data.len() as u32 / self.width
    }
}

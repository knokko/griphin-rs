pub struct ColorGridDump {

    data: Vec<Color>,

    width: u32
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Color {

    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}

impl ColorGridDump {

    pub fn new(data: Vec<Color>, width: u32, height: u32) -> Self {
        if width * height != data.len() as u32 {
            panic!("Width * height must be equal to the size of data, but it's not: {} * {} != {}", width, height, data.len());
        }

        Self {
            data,
            width
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Color {
        self.data[(x + y * self.width) as usize]
    }
}
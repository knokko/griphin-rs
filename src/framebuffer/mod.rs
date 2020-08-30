use crate::*;

use std::any::Any;

pub trait Framebuffer {

    fn as_any(&self) -> &dyn Any;

    fn get_width(&self) -> u32;

    fn get_height(&self) -> u32;
    
    fn dump_pixels(&self, region: FramebufferRegion) -> FramebufferDump;

    fn dump_all_pixels(&self) -> FramebufferDump {
        self.dump_pixels(FramebufferRegion::new(0, 0, self.get_width(), self.get_height()))
    }
}

pub struct FramebufferRegion {

    min_x: u32,
    min_y: u32,
    width: u32,
    height: u32
}

impl FramebufferRegion {

    pub fn new(min_x: u32, min_y: u32, width: u32, height: u32) -> Self {
        Self { min_x, min_y, width, height }
    }

    pub fn entire(framebuffer: &&dyn Framebuffer) -> Self {
        Self {
            min_x: 0,
            min_y: 0,
            width: framebuffer.get_width(),
            height: framebuffer.get_height()
        }
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

pub struct FramebufferDump {

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

impl FramebufferDump {

    pub fn new(data: Vec<Color>, width: u32, height: u32) -> FramebufferDump {
        if width * height != data.len() as u32 {
            panic!("Width * height must be equal to the size of data, but it's not: {} * {} != {}", width, height, data.len());
        }

        FramebufferDump {
            data,
            width
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Color {
        self.data[(x + y * self.width) as usize]
    }
}
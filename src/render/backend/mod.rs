use super::*;

pub mod htmlcanvas;

use std::io::Result;

pub trait RenderBackend {
    fn render_size(&mut self, width: u32, height: u32);

    fn start_render(&mut self) -> Result<()>;
    fn finish_render(&mut self) -> Result<()>;

    fn put_pixel(&mut self, x: u32, y: u32, color: &Color) -> Result<()>;
}

const DEFAULT_BUFFER_SIZE : usize = 1024;

pub struct DummyRenderBackend {
    width: u32,
    height: u32,
    buffer: [u8; DEFAULT_BUFFER_SIZE],
}

impl DummyRenderBackend {
    pub fn new() -> DummyRenderBackend {
        DummyRenderBackend {
            width: 0,
            height: 0,
            buffer: [0; DEFAULT_BUFFER_SIZE],
        }
    }
}

impl RenderBackend for DummyRenderBackend {
    fn render_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn start_render(&mut self) -> Result<()> {
        Ok(())
    }

    fn finish_render(&mut self) -> Result<()> {
        Ok(())
    }

    fn put_pixel(&mut self, x: u32, y: u32, color: &Color) -> Result<()> {
        println!("put_pixel: x: {}, y: {}, color: {:?}", x, y, color);
        Ok(())
    }
}

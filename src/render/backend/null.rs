use super::*;

pub struct NullRenderBackend;

impl NullRenderBackend {
    pub fn new() -> NullRenderBackend {
        NullRenderBackend {}
    }
}

impl RenderBackend for NullRenderBackend {
    fn render_size(&mut self, _: u32, _: u32) {
    }

    fn start_render(&mut self) -> Result<()> {
        Ok(())
    }

    fn finish_render(&mut self) -> Result<()> {
        Ok(())
    }

    fn put_pixel(&mut self, _: u32, _: u32, _: &Color) -> Result<()> {
        Ok(())
    }
}

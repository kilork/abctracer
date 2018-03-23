use super::*;

pub mod dummy;
pub mod htmlcanvas;
pub mod null;

pub use self::dummy::DummyRenderBackend;
pub use self::null::NullRenderBackend;

use std::io::Result;

pub trait RenderBackend {
    fn render_size(&mut self, width: u32, height: u32);

    fn start_render(&mut self) -> Result<()>;
    fn finish_render(&mut self) -> Result<()>;

    fn put_pixel(&mut self, x: u32, y: u32, color: &Color) -> Result<()>;
}

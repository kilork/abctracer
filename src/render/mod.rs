use super::*;

use std::io::Result;

pub mod backend;

pub use self::backend::{RenderBackend, DummyRenderBackend};

use super::mediums::AIR;

pub fn render_scene(
    environment: &Environment,
    half_width: f64,
    half_height: f64,
    nx: u32,
    ny: u32,
    backend: &mut RenderBackend,
) -> Result<()> {
    let pixel_width = 2.0 * half_width / nx as f64;
    let pixel_height = 2.0 * half_height / ny as f64;

    backend.render_size(nx, ny);

    backend.start_render()?;

    // top left corner setup
    let mut y = half_height;

    for y_index in 0..ny {
        let mut x = -half_width;
        for x_index in 0..nx {
            let mut ray = environment.camera(x, y);
            let color = environment.trace(&AIR, 1.0, &mut ray).clip();
            backend.put_pixel(x_index, y_index, &color)?;
            x += pixel_width;
        }
        y -= pixel_height;
    }

    backend.finish_render()?;
    Ok(())
}

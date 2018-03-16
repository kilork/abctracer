use super::*;

use std::io::Result;

pub mod backend;

pub use self::backend::{DummyRenderBackend, RenderBackend};

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

fn rnd() -> f64 {
    0.5
}

pub fn render_scene_supersampling_grid(
    environment: &Environment,
    half_width: f64,
    half_height: f64,
    nx: u32,
    ny: u32,
    nx_sub: u32,
    ny_sub: u32,
    backend: &mut RenderBackend,
) -> Result<()> {
    let pixel_width = 2.0 * half_width / nx as f64;
    let pixel_height = 2.0 * half_height / ny as f64;

    let pixel_sub_width = pixel_width / nx_sub as f64;
    let pixel_sub_height = pixel_height / ny_sub as f64;

    let primary_samples = (nx_sub * ny_sub) as f64;

    backend.render_size(nx, ny);

    backend.start_render()?;

    let mut y = half_height;

    for y_index in 0..ny {
        let mut x = -half_width;
        for x_index in 0..nx {
            let x1 = x - 0.5 * pixel_width;
            let y1 = y - 0.5 * pixel_height;

            let mut color = Color::from(0.0);

            for i_sub in 0..nx_sub {
                for j_sub in 0..ny_sub {
                    let mut ray = environment.camera(
                        x1 + pixel_sub_width * (i_sub as f64 + rnd()),
                        y1 + pixel_sub_height * (j_sub as f64 + rnd()),
                    );
                    color += environment.trace(&AIR, 1.0, &mut ray);
                }
            }
            color /= primary_samples;
            backend.put_pixel(x_index, y_index, &color.clip())?;
            x += pixel_width;
        }
        y -= pixel_height;
    }

    backend.finish_render()?;
    Ok(())
}

const MAX_ADAPTIVE_COUNT : u32 = 99;

pub fn render_scene_supersampling_grid_adaptive(
    environment: &Environment,
    half_width: f64,
    half_height: f64,
    nx: u32,
    ny: u32,
    nx_sub: u32,
    ny_sub: u32,
    variance: f64,
    backend: &mut RenderBackend,
) -> Result<()> {
    let pixel_width = 2.0 * half_width / nx as f64;
    let pixel_height = 2.0 * half_height / ny as f64;

    let pixel_sub_width = pixel_width / nx_sub as f64;
    let pixel_sub_height = pixel_height / ny_sub as f64;

    backend.render_size(nx, ny);

    backend.start_render()?;

    let mut y = half_height;

    for y_index in 0..ny {
        let mut x = -half_width;
        for x_index in 0..nx {
            let x1 = x - 0.5 * pixel_width;
            let y1 = y - 0.5 * pixel_height;

            let mut sum = Color::from(0.0);
            let mut disp = 0.0;
            let mut count = 0;

            let mut mean : Vector;
            loop {
                for i_sub in 0..nx_sub {
                    for j_sub in 0..ny_sub {
                        let mut ray = environment.camera(
                            x1 + pixel_sub_width * (i_sub as f64 + rnd()),
                            y1 + pixel_sub_height * (j_sub as f64 + rnd()),
                        );
                        let color = environment.trace(&AIR, 1.0, &mut ray);
                        sum += color;
                        disp += color & color;
                        count += 1;
                    }
                }
                mean = sum / count as f64;
                let d = ( disp / count as f64 - (mean & mean)) * count as f64 / (count as f64 - 1.0);
                if d < variance || count >= MAX_ADAPTIVE_COUNT {
                    break;
                }
            }

            backend.put_pixel(x_index, y_index, &mean.clip())?;
            x += pixel_width;
        }
        y -= pixel_height;
    }

    backend.finish_render()?;
    Ok(())
}

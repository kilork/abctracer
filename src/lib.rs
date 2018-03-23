#![feature(test)]
extern crate test;

pub mod vector;
pub mod ray;
pub mod matrix;
pub mod tracer;
pub mod render;
pub mod geometry;
pub mod colors;
pub mod mediums;
pub mod light;
pub mod surface;
pub mod environment;

pub use vector::Vector;
pub use ray::Ray;
pub use matrix::Matrix;
pub use tracer::{GObject, LightSource};
pub use render::{render_scene, render_scene_supersampling_grid,
                 render_scene_supersampling_grid_adaptive};
pub use mediums::Medium;
pub use surface::Surface;
pub use environment::Environment;

pub type Color = Vector;

pub const INFINITY: f64 = 30000.0;

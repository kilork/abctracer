use super::*;

pub mod sphere;
pub mod plane;

pub use self::sphere::Sphere;
pub use self::plane::Plane;

const GEOMETRY_THRESHOLD: f64 = 0.001;
const EPS: f64 = 0.01;

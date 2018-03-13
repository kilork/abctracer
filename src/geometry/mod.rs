use super::*;

pub mod sphere;
pub mod plane;
pub mod rect;

pub use self::sphere::Sphere;
pub use self::plane::Plane;
pub use self::rect::Rect;

const GEOMETRY_THRESHOLD: f64 = 0.001;
const EPS: f64 = 0.01;

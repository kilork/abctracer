use super::*;

pub mod sphere;
pub mod plane;
pub mod rect;
pub mod triangle;
pub mod _box;
pub mod cylinder;

pub use self::sphere::Sphere;
pub use self::plane::Plane;
pub use self::rect::Rect;
pub use self::triangle::Triangle;
pub use self::_box::Box;
pub use self::cylinder::Cylinder;

const GEOMETRY_THRESHOLD: f64 = 0.001;
const EPS: f64 = 0.01;

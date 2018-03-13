use super::*;

pub struct Triangle {
    rect: Rect,
}

impl Triangle {
    pub fn new(material: Surface, loc: Vector, side_a: Vector, side_b: Vector) -> Triangle {
        Triangle {
            rect: Rect::new(material, loc, side_a, side_b),
        }
    }
}

impl GObject for Triangle {
    fn material(&self) -> &Surface {
        self.rect.material()
    }

    fn intersect(&self, ray: &Ray, t: &mut f64) -> bool {
        match self.rect.intersect_uv(ray, t) {
            Some((u, v)) => u > 0.0 && v > 0.0 && u + v < 1.0,
            None => false,
        }
    }

    fn find_normal(&self, p: &Vector) -> Vector {
        self.rect.find_normal(p)
    }
}

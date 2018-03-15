use super::*;

pub struct Sphere {
    material: Surface,
    center: Vector,
    radius: f64,
    radius_radius: f64,
}

impl Sphere {
    pub fn new(material: Surface, center: Vector, radius: f64) -> Sphere {
        Sphere {
            material,
            center,
            radius,
            radius_radius: radius * radius,
        }
    }
}

impl GObject for Sphere {
    fn material(&self) -> &Surface {
        &self.material
    }

    fn intersect(&self, ray: &Ray, t: &mut f64) -> bool {
        let l = self.center - ray.org; // direction vector
        let l20c = l & l; // squared distance
        let tca = l & ray.dir; // closest dist to center
        let t2hc = self.radius_radius - l20c + tca * tca;

        if t2hc < 0.0 {
            return false;
        }

        let t2hc = t2hc.sqrt();

        let t2 = if tca < t2hc {
            *t = tca + t2hc;
            tca - t2hc
        } else {
            *t = tca - t2hc;
            tca + t2hc
        };

        if t.abs() < GEOMETRY_THRESHOLD {
            *t = t2;
        }

        *t > GEOMETRY_THRESHOLD
    }

    fn find_normal(&self, p: &Vector) -> Vector {
        (*p - self.center) / self.radius
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_intersect_case_1() {
        let sphere = Sphere::new(Surface::new(), Vector::from((10.0, 0.0, 0.0)), 1.0);

        let ray = Ray::from(((0.0, 0.0, 0.0), (1.0, 0.0, 0.0)));
        let mut dist = 30000.0;
        assert!(sphere.intersect(&ray, &mut dist));

        let dir = Vector::from((10.0, 1.001, 0.0)).normalize();
        let ray = Ray::new(Vector::from(0.0), dir);
        let mut dist = 30000.0;
        assert!(sphere.intersect(&ray, &mut dist));

        let dir = Vector::from((10.0, 1.01, 0.0)).normalize();
        let ray = Ray::new(Vector::from(0.0), dir);
        let mut dist = 30000.0;
        assert!(!sphere.intersect(&ray, &mut dist));
    }
}

use super::*;

pub struct PointLight {
    color: Color,
    center: Vector,
    distance_scale: f64,
}

impl PointLight {
    pub fn new(color: Color, center: Vector, distance_scale: f64) -> PointLight {
        PointLight {
            color,
            center,
            distance_scale,
        }
    }
}

impl LightSource for PointLight {
    fn color(&self) -> &Vector {
        &self.color
    }

    fn shadow(&self, &p: &Vector, l: &mut Vector, environment: &Environment) -> f64 {
        *l = self.center - p; // vector to light source

        let mut distance = !*l; // distance to light source
        let attenuation = self.distance_scale / distance; // distance attenuation of light

        *l /= distance; // normalize vector

        let mut ray = Ray::new(p, *l); // shadow ray

        let mut t = INFINITY;

        let mut attenuation = attenuation * attenuation; // distance attenuation is prop. to squared dist.

        let threshold = environment.threshold();
        while let Some(occlude) = environment.intersect(&ray, &mut t) {
            if distance <= t {
                break;
            }
            // adjust ray origin and get transparency koeff.
            ray.org = ray.point(t);
            let texture = occlude.find_texture(&ray.org);

            if texture.k_t < threshold {
                return 0.0
            }

            attenuation *= texture.k_t;

            if attenuation < threshold {
                return 0.0
            }

            distance -= t;
        }

        attenuation
    }
}

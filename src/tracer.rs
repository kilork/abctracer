use super::*;

pub trait LightSource {
    fn color(&self) -> &Vector;

    fn shadow(&self, p: &Vector, l: &mut Vector, environment: &Environment) -> f64;

    fn shadow_trace(
        &self,
        &p: &Vector,
        l: &mut Vector,
        environment: &Environment,
        mut distance: f64,
        mut attenuation: f64,
    ) -> f64 {
        let mut ray = Ray::new(p, *l); // shadow ray

        let mut t = INFINITY;

        let threshold = environment.threshold();
        while let Some(occlude) = environment.intersect(&ray, &mut t) {
            if distance <= t {
                break;
            }
            // adjust ray origin and get transparency koeff.
            ray.org = ray.point(t);
            let texture = occlude.find_texture(&ray.org);

            if texture.k_t < threshold {
                return 0.0;
            }

            attenuation *= texture.k_t;

            if attenuation < threshold {
                return 0.0;
            }

            distance -= t;
        }

        attenuation
    }
}

pub trait GObject {
    fn material(&self) -> &Surface;

    fn find_texture(&self, p: &Vector) -> Surface {
        let mut result = *self.material();
        result.n = self.find_normal(p);
        result
    }

    fn intersect(&self, ray: &Ray, t: &mut f64) -> bool;

    fn find_normal(&self, p: &Vector) -> Vector;
}

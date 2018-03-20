use super::*;

pub struct SphericLight {
    color: Color,
    center: Vector,
    radius: f64,
    distance_scale: f64,
}

impl SphericLight {
    pub fn new(color: Color, center: Vector, radius: f64, distance_scale: f64) -> SphericLight {
        SphericLight {
            color,
            center,
            radius,
            distance_scale,
        }
    }
}

impl LightSource for SphericLight {
    fn color(&self) -> &Vector {
        &self.color
    }

    fn shadow(&self, &p: &Vector, l: &mut Vector, environment: &Environment) -> f64 {
        *l = self.center - p + environment.random_vector() * self.radius;

        let distance = !*l; // distance to light source
        let attenuation = self.distance_scale / distance; // distance attenuation of light

        *l /= distance; // normalize vector

        self.shadow_trace(&p, l, environment, distance, attenuation)
    }
}

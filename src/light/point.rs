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

        let distance = !*l; // distance to light source
        let attenuation = self.distance_scale / distance; // distance attenuation of light

        *l /= distance; // normalize vector

        let attenuation = attenuation * attenuation; // distance attenuation is prop. to squared dist.

        self.shadow_trace(&p, l, environment, distance, attenuation)
    }
}

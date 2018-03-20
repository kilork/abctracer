use super::*;

pub struct SpotLight {
    color: Color,
    center: Vector,
    direction: Vector,
    cone_angle: f64,
    end_cone_angle: f64,
    beam_distribution: i32,

    distance_scale: f64,
}

impl SpotLight {
    pub fn new(
        color: Color,
        center: Vector,
        direction: Vector,
        cone_angle: f64,
        end_cone_angle: f64,
        beam_distribution: i32,
        distance_scale: f64,
    ) -> SpotLight {
        SpotLight {
            color,
            center,
            direction,
            cone_angle,
            end_cone_angle,
            beam_distribution,
            distance_scale,
        }
    }
}

impl LightSource for SpotLight {
    fn color(&self) -> &Vector {
        &self.color
    }

    fn shadow(&self, &p: &Vector, l: &mut Vector, environment: &Environment) -> f64 {
        *l = self.center - p; // vector to light source

        let distance = !*l; // distance to light source
        let mut attenuation = self.distance_scale / distance; // distance attenuation of light

        *l /= distance; // normalize vector

        let ld = -(self.direction & *l);

        if ld < self.end_cone_angle {
            return 0.0;
        }

        let f1 = ld.powi(self.beam_distribution);
        let f2 = if ld > self.cone_angle {
            1.0
        } else {
            (ld - self.end_cone_angle) / (self.cone_angle - self.end_cone_angle)
        };

        attenuation *= attenuation * f1 * f2;

        self.shadow_trace(&p, l, environment, distance, attenuation)
    }
}

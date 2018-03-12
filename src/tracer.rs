use super::*;

use super::mediums::AIR;

pub trait LightSource {
    fn color(&self) -> &Vector;

    fn shadow(&self, p: &Vector, l: &mut Vector, environment: &Environment) -> f64;
}

pub trait GObject {
    fn def_material(&self) -> &Surface;

    fn find_texture(&self, p: &Vector) -> Surface {
        let mut result = *self.def_material();
        result.n = self.find_normal(p);
        result
    }

    fn intersect(&self, ray: &Ray, t: &mut f64) -> bool;

    fn find_normal(&self, p: &Vector) -> Vector;
}


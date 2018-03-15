use super::*;

use super::mediums::AIR;

const BACKGROUND: Vector = Vector {
    x: 0.0,
    y: 0.05,
    z: 0.05,
};

const AMBIENT: Vector = Vector {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};

pub struct Environment<'a> {
    lights: Vec<&'a LightSource>,
    solids: Vec<&'a GObject>,

    eye: Vector,
    eye_dir: Vector,
    v_x: Vector,
    v_y: Vector,
    background: Color,
    max_level: u32,
    threshold: f64,
}

struct TraceState {
    level: u32,
    total_rays: u32,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            lights: Vec::new(),
            solids: Vec::new(),
            eye: Vector::from(0.0),
            eye_dir: Vector::from((0.0, 0.0, 1.0)),
            v_x: Vector::from((1.0, 0.0, 0.0)),
            v_y: Vector::from((0.0, 1.0, 0.0)),
            background: BACKGROUND,
            max_level: 10,
            threshold: 0.01,
        }
    }

    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    pub fn add_solid(&mut self, solid: &'a GObject) {
        self.solids.push(solid);
    }

    pub fn add_light(&mut self, light: &'a LightSource) {
        self.lights.push(light);
    }

    pub fn intersect(&self, ray: &Ray, distance: &mut f64) -> Option<&GObject> {
        let mut closest_object = None;
        let mut closest_distance = INFINITY;
        for &solid in &self.solids {
            if solid.intersect(ray, distance) {
                if *distance < closest_distance {
                    closest_distance = *distance;
                    closest_object = Some(solid);
                }
            }
        }
        *distance = closest_distance;
        closest_object
    }

    fn shade_background(&self, ray: &Ray) -> Color {
        self.background
    }

    pub fn set_camera(&mut self, &org: &Vector, &dir: &Vector, &up_dir: &Vector) {
        self.eye = org; // eye point
        self.eye_dir = dir; // viewing direction
        self.v_x = (up_dir ^ dir).normalize(); // build orthogonal basis of image plane
        self.v_y = (dir ^ self.v_x).normalize(); // eye_dir orthogonal to this basic (image plane)
    }

    pub fn camera(&self, x: f64, y: f64) -> Ray {
        Ray {
            org: self.eye,
            dir: (self.eye_dir + self.v_x * x + self.v_y * y).normalize(),
        }
    }

    pub fn trace(&self, current_medium: &Medium, weight: f64, ray: &mut Ray) -> Color {
        self.trace_state(
            &mut TraceState {
                level: 0,
                total_rays: 0,
            },
            current_medium,
            weight,
            ray,
        )
    }

    fn trace_state(
        &self,
        trace_state: &mut TraceState,
        current_medium: &Medium,
        weight: f64,
        ray: &mut Ray,
    ) -> Color {
        let mut t = INFINITY;
        let color: Color;

        trace_state.level += 1;
        trace_state.total_rays += 1;

        if let Some(solid) = self.intersect(ray, &mut t) {
            color = self.shade(
                trace_state,
                current_medium,
                weight,
                ray.point(t),
                ray.dir,
                solid,
            )
        } else {
            color = self.shade_background(ray);
        }
        trace_state.level -= 1;
        color
    }

    fn shade(
        &self,
        trace_state: &mut TraceState,
        current_medium: &Medium,
        weight: f64,
        mut p: Vector,
        mut view: Vector,
        solid: &GObject,
    ) -> Color {
        let mut entering = true; // flag whether we're entering or leaving object

        let mut texture = solid.find_texture(&p);

        let mut vn = view & texture.n; // force (-view, n) > 0
        if vn > 0.0 {
            texture.n = -texture.n;
            vn = -vn;
            entering = false;
        }

        let mut ray = Ray::new(p, Vector::from(0.0)); // since all rays will be cast from here

        let mut color = AMBIENT * texture.color * texture.k_r; // get ambient light

        for light in &self.lights {
            let mut l = Vector::from(0.0); // light vector
            let shadow = light.shadow(&p, &mut l, &self); // light shadow coeff.
            if shadow > self.threshold {
                let ln = l & texture.n;
                // if light is visible
                if ln > self.threshold {
                    // compute direct diffuse light
                    if texture.k_d > self.threshold {
                        color += *light.color() * texture.color * (texture.k_d * shadow * ln);
                    }

                    // compute direct specular light, via Phong shading
                    if texture.k_s > self.threshold {
                        // compute half-vector between -view and light vector
                        let h = (l - view).normalize();
                        color += *light.color()
                            * (texture.k_s * shadow * (texture.n & h).powi(texture.p));
                    }
                }
            }
        }

        if trace_state.level >= self.max_level {
            return color;
        }

        // check for reflected ray
        let r_weight = weight * texture.k_r; // weight of reflected ray
        if r_weight > self.threshold {
            // get reflected ray direction
            ray.dir = view - texture.n * (2.0 * vn);
            color +=
                texture.k_r * self.trace_state(trace_state, current_medium, r_weight, &mut ray);
        }

        // check for transmitted ray
        let t_weight = weight * texture.k_t; // weight of transmitted ray
        if t_weight > self.threshold {
            // relative index of refraction
            let eta = current_medium.n_refr / if entering {
                texture.medium.n_refr
            } else {
                AIR.n_refr
            };
            let ci = -vn; // cosine of incedent angle
            let ct_square = 1.0 + eta * eta * (ci * ci - 1.0); // square cosine of transm. angle

            // not a Total Internal Reflection
            if ct_square > self.threshold {
                ray.dir = view * eta + texture.n * (eta * ci - ct_square.sqrt());
                let medium = &if entering {
                    // ray enters object (texture.medium)
                    texture.medium
                } else {
                    // ray leaves object (AIR)
                    AIR
                };
                color += texture.k_r * self.trace_state(trace_state, medium, t_weight, &mut ray);
            }
        }
        color
    }
}

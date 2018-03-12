use super::*;

#[derive(Clone, Copy)]
pub struct Surface {
    pub k_a: f64,
    pub k_d: f64,
    pub k_s: f64,
    pub k_r: f64,
    pub k_t: f64,
    pub color: Color,
    pub medium: Medium,
    pub p: i32,
    pub n: Vector,
}

impl Surface {
    pub fn new() -> Surface {
        Surface {
            k_a: 0.0,
            k_d: 0.0,
            k_s: 0.0,
            k_r: 0.0,
            k_t: 0.0,
            color: Color::from(0.0),
            medium: Medium {
                n_refr: 0.0,
                betta: 0.0,
            },
            p: 0,
            n: Vector::from((0.0, 0.0, 0.0)),
        }
    }
}
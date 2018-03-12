#[derive(Clone, Copy)]
pub struct Medium {
    pub n_refr: f64,
    pub betta: f64,
}

pub const AIR: Medium = Medium {
    n_refr: 1.0,
    betta: 0.0,
};

pub const GLASS: Medium = Medium {
    n_refr: 1.5,
    betta: 0.0,
};

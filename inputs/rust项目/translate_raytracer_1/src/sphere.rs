use crate::noise::*;
use crate::vectors::*;
use crate::scene::*;
use crate::def::*;
use crate::colour::*;
use crate::fileformat::*;
use crate::triangle::*;
use crate::tokenizer::*;
use crate::mod_3dsloader::*;
#[repr(C)]
pub struct sphere {
    pub pos: vector,
    pub size: f64,
    pub material: i32,
}

impl Default for sphere {
    fn default() -> Self {
        sphere {
            pos: vector::default(),
            size: 0.0,
            material: 0,
        }
    }
}

pub fn collideRaySphere(r: &ray, s: &sphere, t: &mut f64) -> bool {
    let dist = vectorSub(&r.start, &s.pos);
    let B = vectorDot(&dist, &r.dir);

    let D = B * B - vectorDot(&dist, &dist) + s.size * s.size;

    if D < 0.0 {
        return false;
    }

    let t0 = -B - D.sqrt();
    let t1 = -B + D.sqrt();

    let mut retvalue = false;

    if (t0 > 0.1) && (t0 < *t) {
        *t = t0;
        retvalue = true;
    }
    if (t1 > 0.1) && (t1 < *t) {
        *t = t1;
        retvalue = true;
    }

    retvalue
}
use crate::noise::*;
use crate::vectors::*;
use crate::scene::*;
use crate::def::*;
use crate::colour::*;
use crate::fileformat::*;
use crate::tokenizer::*;
use crate::mod_3dsloader::*;
use crate::sphere::*;
#[repr(C)]
#[derive(Clone)]
pub struct triangle {
    pub v1: vector,
    pub v2: vector,
    pub v3: vector,
    pub material: i32,
}

impl Default for triangle {
    fn default() -> Self {
        triangle {
            v1: Default::default(),
            v2: Default::default(),
            v3: Default::default(),
            material: 0,
        }
    }
}

pub fn collideRayTriangle(r: &ray, t: &triangle, result: &mut f64, normal: &mut vector) -> bool {
    let mut det: f64;
    let mut invdet: f64;
    let edge1: vector = vectorSub(&t.v2, &t.v1);
    let edge2: vector = vectorSub(&t.v3, &t.v1);

    /* Find the cross product of edge2 and the ray direction */
    let s1: vector = vectorCross(&r.dir, &edge2);

    det = vectorDot(&edge1, &s1);
    if det > -0.000001 && det < 0.000001 {
        return FALSE;
    }

    invdet = 1.0 / det;

    let s2: vector = vectorSub(&r.start, &t.v1);

    let u: f64 = vectorDot(&s2, &s1) * invdet;

    if u < 0.0 || u > 1.0 {
        return FALSE;
    }

    let s3: vector = vectorCross(&s2, &edge1);

    let v: f64 = vectorDot(&r.dir, &s3) * invdet;

    if v < 0.0 || (u + v) > 1.0 {
        return FALSE;
    }

    let tmp: f64 = vectorDot(&edge2, &s3) * invdet;

    if (tmp < 0.0) || (tmp > *result) {
        return FALSE;
    }

    /* subtract tiny amount - otherwise artifacts due to floating point imprecisions... */
    *result = tmp - 0.005;
    *normal = vectorCross(&edge2, &edge1);

    return TRUE;
}
use crate::noise::*;
use crate::scene::*;
use crate::def::*;
use crate::colour::*;
use crate::triangle::*;
use crate::fileformat::*;
use crate::tokenizer::*;
use crate::mod_3dsloader::*;
use crate::sphere::*;
#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn vectorAdd(v1: &vector, v2: &vector) -> vector {
    vector {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z,
    }
}

pub fn vectorSub(v1: &vector, v2: &vector) -> vector {
    vector {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

pub fn vectorDot(v1: &vector, v2: &vector) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn vectorScale(c: f64, v: &vector) -> vector {
    vector {
        x: v.x * c,
        y: v.y * c,
        z: v.z * c,
    }
}

pub fn vectorCross(v1: &vector, v2: &vector) -> vector {
    let mut result = vector::default();

    result.x = (v1.y * v2.z) - (v1.z * v2.y);
    result.y = (v1.z * v2.x) - (v1.x * v2.z);
    result.z = (v1.x * v2.y) - (v1.y * v2.x);

    result
}
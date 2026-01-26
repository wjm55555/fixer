use crate::noise::*;
use crate::vectors::*;
use crate::scene::*;
use crate::def::*;
use crate::fileformat::*;
use crate::triangle::*;
use crate::tokenizer::*;
use crate::mod_3dsloader::*;
use crate::sphere::*;
#[repr(C)]
pub struct colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Default for colour {
    fn default() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
}

impl Clone for colour {
    fn clone(&self) -> Self {
        Self {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
}

pub fn colourMul(c1: &colour, c2: &colour) -> colour {
    let c = colour {
        red: c1.red * c2.red,
        green: c1.green * c2.green,
        blue: c1.blue * c2.blue,
    };
    c
}

pub fn colourAdd(c1: &colour, c2: &colour) -> colour {
    let c = colour {
        red: c1.red + c2.red,
        green: c1.green + c2.green,
        blue: c1.blue + c2.blue,
    };
    c
}

pub fn colourCoefMul(coef: f64, c: &colour) -> colour {
    let result = colour {
        red: c.red * coef,
        green: c.green * coef,
        blue: c.blue * coef,
    };
    result
}
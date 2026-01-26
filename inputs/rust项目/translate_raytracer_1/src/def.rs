pub const PIOVER180: f64 = 0.017453292519943295769236907684886;
pub const PI: f64 = 3.141592653589793238462643383279502;
pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;
pub type bool = u8;

pub fn min<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

pub fn max<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

pub fn invsqrtf(x: f32) -> f32 {
    1.0f32 / x.sqrt()
}
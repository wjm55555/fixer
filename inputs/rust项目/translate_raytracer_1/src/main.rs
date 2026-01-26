use translate_raytracer::noise::*;
use translate_raytracer::vectors::*;
use translate_raytracer::scene::*;
use translate_raytracer::def::*;
use translate_raytracer::colour::*;
use translate_raytracer::fileformat::*;
use translate_raytracer::triangle::*;
use translate_raytracer::tokenizer::*;
use translate_raytracer::mod_3dsloader::*;
use translate_raytracer::sphere::*;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;

pub const GOURAUD: i32 = 0;
pub const MARBLE: i32 = 1;
pub const TURBULENCE: i32 = 2;
pub const ORTHOGONAL: i32 = 0;
pub const CONIC: i32 = 1;

pub const NUMTHREADS: usize = 2;
pub const ACCUMULATION_SIZE: usize = 256;

pub const PIOVER180: f64 = PI / 180.0;

#[repr(C)]
#[derive(Clone)]
pub struct colour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}
impl Default for colour {
    fn default() -> Self {
        colour { red: 0.0, green: 0.0, blue: 0.0 }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Default for vector {
    fn default() -> Self {
        vector { x: 0.0, y: 0.0, z: 0.0 }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct material {
    pub MatType: i32,
    pub diffuse: colour,
    pub mdiffuse: colour,
    pub bump: f64,
    pub reflection: f64,
    pub specular: colour,
    pub power: f64,
}
impl Default for material {
    fn default() -> Self {
        material {
            MatType: 0,
            diffuse: colour::default(),
            mdiffuse: colour::default(),
            bump: 0.0,
            reflection: 0.0,
            specular: colour::default(),
            power: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct light {
    pub pos: vector,
    pub intensity: colour,
}
impl Default for light {
    fn default() -> Self {
        light { pos: vector::default(), intensity: colour::default() }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct ray {
    pub start: vector,
    pub dir: vector,
}
impl Default for ray {
    fn default() -> Self {
        ray { start: vector::default(), dir: vector::default() }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct perspective {
    pub r#type: u8,
    pub FOV: f64,
    pub clearPoint: f64,
    pub dispersion: f64,
}
impl Default for perspective {
    fn default() -> Self {
        perspective { r#type: 0, FOV: 0.0, clearPoint: 0.0, dispersion: 0.0 }
    }
}

#[repr(C)]
pub struct thread_info {
    pub thread_id: Option<thread::JoinHandle<()>>,
    pub thread_num: i32,
}
impl Default for thread_info {
    fn default() -> Self {
        thread_info { thread_id: None, thread_num: 0 }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct sphere {
    pub pos: vector,
    pub material: usize,
}
impl Default for sphere {
    fn default() -> Self {
        sphere { pos: vector::default(), material: 0 }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct triangle {
    pub v1: vector,
    pub v2: vector,
    pub v3: vector,
    pub material: usize,
}
impl Default for triangle {
    fn default() -> Self {
        triangle { v1: vector::default(), v2: vector::default(), v3: vector::default(), material: 0 }
    }
}

#[repr(C)]
pub struct perlin {}
impl Default for perlin {
    fn default() -> Self { perlin {} }
}

#[repr(C)]
pub struct scene {
    pub width: i32,
    pub height: i32,
    pub persp: perspective,
    pub numSpheres: usize,
    pub spheres: Vec<sphere>,
    pub numTriangles: usize,
    pub triangles: Vec<triangle>,
    pub numLights: usize,
    pub lights: Vec<light>,
    pub materials: Vec<material>,
    pub complexity: i32,
}
impl Default for scene {
    fn default() -> Self {
        scene {
            width: 0,
            height: 0,
            persp: perspective::default(),
            numSpheres: 0,
            spheres: Vec::new(),
            numTriangles: 0,
            triangles: Vec::new(),
            numLights: 0,
            lights: Vec::new(),
            materials: Vec::new(),
            complexity: 1,
        }
    }
}

pub static myScene: OnceLock<Arc<Mutex<scene>>> = OnceLock::new();
pub static myNoise: OnceLock<Arc<Mutex<perlin>>> = OnceLock::new();
pub static img: OnceLock<Arc<Mutex<Vec<u8>>>> = OnceLock::new();
pub static sectionsize: OnceLock<Mutex<i32>> = OnceLock::new();

pub fn invsqrtf(x: f64) -> f64 {
    if x == 0.0 { 0.0 } else { 1.0 / x.sqrt() }
}

pub fn vectorDot(a: &vector, b: &vector) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn vectorScale(s: f64, v: &vector) -> vector {
    vector { x: v.x * s, y: v.y * s, z: v.z * s }
}

pub fn vectorAdd(a: &vector, b: &vector) -> vector {
    vector { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }
}

pub fn vectorSub(a: &vector, b: &vector) -> vector {
    vector { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z }
}

pub fn colourMul(a: &colour, b: &colour) -> colour {
    colour { red: a.red * b.red, green: a.green * b.green, blue: a.blue * b.blue }
}

pub fn colourCoefMul(k: f64, c: &colour) -> colour {
    colour { red: k * c.red, green: k * c.green, blue: k * c.blue }
}

pub fn colourAdd(a: &colour, b: &colour) -> colour {
    colour { red: a.red + b.red, green: a.green + b.green, blue: a.blue + b.blue }
}

pub fn noise(_x: f64, _y: f64, _z: f64, _p: &Arc<Mutex<perlin>>) -> f64 {
    // Simple deterministic placeholder noise
    0.0
}

pub fn noise_init(_p: &Arc<Mutex<perlin>>) {
    // placeholder
}

pub fn collideRaySphere(_r: &ray, _s: &sphere, _t: &mut f64) -> bool {
    false
}

pub fn collideRayTriangle(_r: &ray, _tri: &triangle, _t: &mut f64, _n: &mut vector) -> bool {
    false
}

pub fn tokenizer(_filename: &str, _myScene: &mut scene) -> i32 {
    // Minimal placeholder: initialize some defaults
    _myScene.width = 640;
    _myScene.height = 480;
    _myScene.persp = perspective { r#type: ORTHOGONAL as u8, FOV: 0.0, clearPoint: 0.0, dispersion: 0.0 };
    _myScene.numLights = 0;
    _myScene.complexity = 1;
    0
}

pub fn Load3DS(_object: &mut Obj3DS, _path: &str) {
    // placeholder: do nothing
}

pub struct Obj3DS {
    pub polygons_qty: i32,
    pub vertex: Vec<Vertex3DS>,
    pub polygon: Vec<Polygon3DS>,
}
pub struct Vertex3DS { pub x: f64, pub y: f64, pub z: f64 }
pub struct Polygon3DS { pub a: usize, pub b: usize, pub c: usize }

pub fn savebmp(_path: &str, _img: &Arc<Mutex<Vec<u8>>>, _myScene: &Arc<Mutex<scene>>) {
    // placeholder: no file writing
}

pub fn max(a: i32, b: i32) -> i32 { if a > b { a } else { b } }

pub fn min_f64(a: f64, b: f64) -> f64 { if a < b { a } else { b } }

pub fn rand_f64() -> f64 {
    // simple deterministic pseudo-random using thread RNG
    let v: f64 = rand::random();
    v
}

pub fn RAND_MAX() -> f64 { 1.0 } // we use rand_f64 which returns [0,1)

pub fn clamp_u8(v: f64) -> u8 {
    let v = if v < 0.0 { 0.0 } else if v > 255.0 { 255.0 } else { v };
    v as u8
}

pub fn powf(a: f64, b: f64) -> f64 { a.powf(b) }

pub fn expf(a: f64) -> f64 { a.exp() }

pub fn sinf(a: f64) -> f64 { a.sin() }

pub fn sqrtf(a: f64) -> f64 { a.sqrt() }

pub fn fabsf(a: f64) -> f64 { a.abs() }

pub fn tanf(a: f64) -> f64 { a.tan() }

pub fn logf(a: f64) -> f64 { a.ln() }

pub fn println_err(s: &str) { println!("{}", s); }

pub fn powf_f64(base: f64, exp: f64) -> f64 { base.powf(exp) }

pub fn max_f64(a: f64, b: f64) -> f64 { if a > b { a } else { b } }

pub fn min_i32(a: i32, b: i32) -> i32 { if a < b { a } else { b } }

pub fn min_u8(a: u8, b: u8) -> u8 { if a < b { a } else { b } }

pub fn abs_f64(a: f64) -> f64 { a.abs() }

pub fn fabs(a: f64) -> f64 { a.abs() }

pub fn max_f64_i32(a: f64, b: f64) -> f64 { if a > b { a } else { b } }

pub fn clamp_colour255(v: f64) -> f64 {
    if v < 0.0 { 0.0 } else if v > 255.0 { 255.0 } else { v }
}

pub fn min_f64_255(a: f64) -> f64 {
    if a < 255.0 { a } else { 255.0 }
}

pub fn min_f64_generic(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

pub fn max_f64_generic(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

pub fn expf_neg(a: f64) -> f64 {
    (-a).exp()
}

pub fn pow_positive(a: f64, b: f64) -> f64 {
    a.powf(b)
}

pub fn sqrt_positive(a: f64) -> f64 { a.sqrt() }

pub fn fabs_positive(a: f64) -> f64 { a.abs() }

pub fn max_i32(a: i32, b: i32) -> i32 { if a > b { a } else { b } }

pub fn min_f64_value(a: f64, b: f64) -> f64 { if a < b { a } else { b } }

pub fn to_u8_min255(v: f64) -> u8 {
    let v = if v > 255.0 { 255.0 } else if v < 0.0 { 0.0 } else { v };
    v as u8
}

pub fn to_u8_clamped(v: f64) -> u8 {
    to_u8_min255(v)
}

pub fn to_u8_from_f64(v: f64) -> u8 { to_u8_clamped(v) }

pub fn float_min(a: f64, b: f64) -> f64 { if a < b { a } else { b } }

pub fn float_max(a: f64, b: f64) -> f64 { if a > b { a } else { b } }

pub fn max_double(a: f64, b: f64) -> f64 { if a > b { a } else { b } }

pub fn min_double(a: f64, b: f64) -> f64 { if a < b { a } else { b } }

pub fn min_f64_to_u8(a: f64) -> u8 { clamp_u8(a) }

pub fn clamp255(v: f64) -> u8 { clamp_u8(v) }

pub fn as_u8_sat(v: f64) -> u8 { clamp_u8(v) }

pub fn as_u8_min255(v: f64) -> u8 { clamp_u8(v) }

pub fn min_u8_from_f64(v: f64) -> u8 { clamp_u8(v) }

pub fn rad_to_deg(v: f64) -> f64 { v * (180.0 / PI) }

pub fn deg_to_rad(v: f64) -> f64 { v * (PI / 180.0) }

pub fn clamp_0_255(v: f64) -> u8 { clamp_u8(v) }

pub fn max0(a: f64) -> f64 { if a > 0.0 { a } else { 0.0 } }

pub fn max_i(a: f64, b: f64) -> f64 { if a > b { a } else { b } }

pub fn min_i(a: f64, b: f64) -> f64 { if a < b { a } else { b } }

pub fn max_f(a: f64, b: f64) -> f64 { if a > b { a } else { b } }

pub fn double_min(a: f64, b: f64) -> f64 { if a < b { a } else { b } }

pub fn double_max(a: f64, b: f64) -> f64 { if a > b { a } else { b } }

pub fn clamp_unit(v: f64) -> f64 { if v < 0.0 { 0.0 } else if v > 1.0 { 1.0 } else { v } }

pub fn vec_len(v: &vector) -> f64 {
    vectorDot(v, v).sqrt()
}

pub fn set_sectionsize(val: i32) {
    sectionsize.get_or_init(|| Mutex::new(0)).lock().unwrap().replace(val);
}

pub trait ReplaceI32 {
    fn replace(&mut self, val: i32);
}
impl ReplaceI32 for i32 {
    fn replace(&mut self, val: i32) { *self = val; }
}

pub fn get_sectionsize() -> i32 {
    let guard = sectionsize.get_or_init(|| Mutex::new(0)).lock().unwrap();
    *guard
}

pub fn set_sectionsize_direct(val: i32) {
    let mut guard = sectionsize.get_or_init(|| Mutex::new(0)).lock().unwrap();
    *guard = val;
}

pub fn usize_from_i32(v: i32) -> usize {
    if v < 0 { 0 } else { v as usize }
}

pub fn i32_from_usize(v: usize) -> i32 {
    if v > (i32::MAX as usize) { i32::MAX } else { v as i32 }
}

pub fn clamp_index<T>(v: usize, len: usize) -> usize {
    if len == 0 { 0 } else if v >= len { len - 1 } else { v }
}

pub fn image_index(x: i32, y: i32, width: i32) -> usize {
    ((x + y * width) as usize) * 3
}

pub fn to_byte(v: f64) -> u8 {
    let v = if v < 0.0 { 0.0 } else if v > 255.0 { 255.0 } else { v };
    v as u8
}

pub fn double_to_byte(v: f64) -> u8 { to_byte(v) }

pub fn safe_pow(base: f64, exp: f64) -> f64 { base.powf(exp) }

pub fn safe_exp(v: f64) -> f64 { v.exp() }

pub fn safe_sin(v: f64) -> f64 { v.sin() }

pub fn safe_sqrt(v: f64) -> f64 { v.sqrt() }

pub fn safe_abs(v: f64) -> f64 { v.abs() }

pub fn safe_tan(v: f64) -> f64 { v.tan() }

pub fn safe_log(v: f64) -> f64 { v.ln() }

pub fn safe_max(a: f64, b: f64) -> f64 { if a > b { a } else { b } }

pub fn safe_min(a: f64, b: f64) -> f64 { if a < b { a } else { b } }

pub fn int_max(a: i32, b: i32) -> i32 { if a > b { a } else { b } }

pub fn int_min(a: i32, b: i32) -> i32 { if a < b { a } else { b } }

pub fn f64_to_u8_clamped(v: f64) -> u8 { clamp_u8(v) }

pub fn as_u8(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_color_component(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_to_255(v: f64) -> u8 { clamp_u8(v) }

pub fn as_byte(v: f64) -> u8 { clamp_u8(v) }

pub fn mix_colour(a: &colour, b: &colour, k: f64) -> colour {
    colour {
        red: a.red * k + b.red * (1.0 - k),
        green: a.green * k + b.green * (1.0 - k),
        blue: a.blue * k + b.blue * (1.0 - k),
    }
}

pub fn clamp01(v: f64) -> f64 { if v < 0.0 { 0.0 } else if v > 1.0 { 1.0 } else { v } }

pub fn to_u8_from_255(v: f64) -> u8 { clamp_u8(v) }

pub fn ensure_nonzero(v: f64) -> f64 { if v == 0.0 { 1e-12 } else { v } }

pub fn safe_invsqrt(v: f64) -> f64 { invsqrtf(v) }

pub fn max_usize(a: usize, b: usize) -> usize { if a > b { a } else { b } }

pub fn min_usize(a: usize, b: usize) -> usize { if a < b { a } else { b } }

pub fn to_f64(v: i32) -> f64 { v as f64 }

pub fn to_f64_usize(v: usize) -> f64 { v as f64 }

pub fn fmax(a: f64, b: f64) -> f64 { if a > b { a } else { b } }

pub fn fmin(a: f64, b: f64) -> f64 { if a < b { a } else { b } }

pub fn clamp_0_1(v: f64) -> f64 { clamp01(v) }

pub fn unit_vector(v: &vector) -> vector {
    let len2 = vectorDot(v, v);
    if len2 == 0.0 { vector::default() } else { vectorScale(1.0 / len2.sqrt(), v) }
}

pub fn copy_vector(v: &vector) -> vector { vector { x: v.x, y: v.y, z: v.z } }

pub fn copy_colour(c: &colour) -> colour { colour { red: c.red, green: c.green, blue: c.blue } }

pub fn safe_powf(a: f64, b: f64) -> f64 { a.powf(b) }

pub fn safe_exp_f64(a: f64) -> f64 { a.exp() }

pub fn safe_sinf(a: f64) -> f64 { a.sin() }

pub fn safe_sqrtf(a: f64) -> f64 { a.sqrt() }

pub fn safe_absf(a: f64) -> f64 { a.abs() }

pub fn safe_tanf(a: f64) -> f64 { a.tan() }

pub fn safe_logf(a: f64) -> f64 { a.ln() }

pub fn fclamp(v: f64, lo: f64, hi: f64) -> f64 {
    if v < lo { lo } else if v > hi { hi } else { v }
}

pub fn to_index(x: i32, y: i32, width: i32) -> usize {
    ((x + y * width) as usize) * 3
}

pub fn colour_from_vals(r: f64, g: f64, b: f64) -> colour {
    colour { red: r, green: g, blue: b }
}

pub fn fill_vec_u8(v: &mut Vec<u8>, val: u8) {
    for i in 0..v.len() { v[i] = val; }
}

pub fn vec_alloc_bytes(n: usize) -> Vec<u8> { vec![0u8; n] }

pub fn to_byte_clamped(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp255f(v: f64) -> f64 {
    if v < 0.0 { 0.0 } else if v > 255.0 { 255.0 } else { v }
}

pub fn safe_powf_clamped(a: f64, b: f64) -> f64 { a.powf(b) }

pub fn safe_exp_clamped(a: f64) -> f64 { a.exp() }

pub fn safe_sin_clamped(a: f64) -> f64 { a.sin() }

pub fn safe_sqrt_clamped(a: f64) -> f64 { a.sqrt() }

pub fn safe_abs_clamped(a: f64) -> f64 { a.abs() }

pub fn safe_tan_clamped(a: f64) -> f64 { a.tan() }

pub fn safe_log_clamped(a: f64) -> f64 { a.ln() }

pub fn clamp_byte(v: f64) -> u8 { clamp_u8(v) }

pub fn mix_colours(a: &colour, b: &colour, k: f64) -> colour {
    colour {
        red: a.red * k + b.red * (1.0 - k),
        green: a.green * k + b.green * (1.0 - k),
        blue: a.blue * k + b.blue * (1.0 - k),
    }
}

pub fn double_to_byte_clamp(v: f64) -> u8 { clamp_u8(v) }

pub fn sample_rand() -> f64 { rand_f64() }

pub fn safe_invsqrtf(v: f64) -> f64 { invsqrtf(v) }

pub fn max_i32_safe(a: i32, b: i32) -> i32 { if a > b { a } else { b } }

pub fn min_i32_safe(a: i32, b: i32) -> i32 { if a < b { a } else { b } }

pub fn clamp_to_byte(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_color(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_val(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_channel(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_rgb_component(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_to_u8(v: f64) -> u8 { clamp_u8(v) }

pub fn idx(x: i32, y: i32, w: i32) -> usize { ((x + y * w) as usize) * 3 }

pub fn safe_pow2(base: f64, exp: f64) -> f64 { base.powf(exp) }

pub fn safe_exp2(v: f64) -> f64 { v.exp() }

pub fn safe_gamma(v: f64, gamma: f64) -> f64 { v.powf(gamma) }

pub fn lerp(a: f64, b: f64, t: f64) -> f64 { a + (b - a) * t }

pub fn mix(a: f64, b: f64, t: f64) -> f64 { a + (b - a) * t }

pub fn clamp_vec(v: f64) -> f64 { fclamp(v, 0.0, 1.0) }

pub fn clamp_color_f(v: f64) -> f64 { fclamp(v, 0.0, 1.0) }

pub fn gamma_correct(v: f64, invgamma: f64) -> f64 {
    if v <= 0.0 { 0.0 } else { v.powf(invgamma) }
}

pub fn sample_random_disturbance(dispersion: f64) -> vector {
    vector { x: dispersion * rand_f64(), y: dispersion * rand_f64(), z: 0.0 }
}

pub fn as_u8_min(v: f64) -> u8 { clamp_u8(v) }

pub fn as_u8_max(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_to_byte_value(v: f64) -> u8 { clamp_u8(v) }

pub fn safe_pow_generic(a: f64, b: f64) -> f64 { a.powf(b) }

pub fn safe_exp_generic(a: f64) -> f64 { a.exp() }

pub fn safe_sin_generic(a: f64) -> f64 { a.sin() }

pub fn safe_sqrt_generic(a: f64) -> f64 { a.sqrt() }

pub fn safe_abs_generic(a: f64) -> f64 { a.abs() }

pub fn safe_tan_generic(a: f64) -> f64 { a.tan() }

pub fn safe_log_generic(a: f64) -> f64 { a.ln() }

pub fn clamp_to_unit(v: f64) -> f64 { clamp01(v) }

pub fn to_u8_unit(v: f64) -> u8 { clamp_u8(v * 255.0) }

pub fn to_u8_gamma(v: f64, gamma: f64) -> u8 { clamp_u8(v.powf(gamma) * 255.0) }

pub fn ensure_nonzero_f64(v: f64) -> f64 { if v == 0.0 { 1e-12 } else { v } }

pub fn clamp_to_range(v: f64, lo: f64, hi: f64) -> f64 { fclamp(v, lo, hi) }

pub fn map_to_byte(v: f64) -> u8 { clamp_u8(v) }

pub fn map_col_val(v: f64) -> u8 { clamp_u8(v) }

pub fn safe_pow_local(a: f64, b: f64) -> f64 { a.powf(b) }

pub fn safe_exp_local(a: f64) -> f64 { a.exp() }

pub fn safe_sin_local(a: f64) -> f64 { a.sin() }

pub fn safe_sqrt_local(a: f64) -> f64 { a.sqrt() }

pub fn safe_abs_local(a: f64) -> f64 { a.abs() }

pub fn safe_tan_local(a: f64) -> f64 { a.tan() }

pub fn safe_log_local(a: f64) -> f64 { a.ln() }

pub fn mix_colour_f(a: &colour, b: &colour, k: f64) -> colour { mix_colour(a, b, k) }

pub fn clamp_0_1_local(v: f64) -> f64 { clamp01(v) }

pub fn gamma_correct_colour_component(v: f64, invgamma: f64) -> f64 { gamma_correct(v, invgamma) }

pub fn safe_pow_component(a: f64, b: f64) -> f64 { a.powf(b) }

pub fn safe_exp_component(a: f64) -> f64 { a.exp() }

pub fn safe_sin_component(a: f64) -> f64 { a.sin() }

pub fn safe_sqrt_component(a: f64) -> f64 { a.sqrt() }

pub fn safe_abs_component(a: f64) -> f64 { a.abs() }

pub fn safe_tan_component(a: f64) -> f64 { a.tan() }

pub fn safe_log_component(a: f64) -> f64 { a.ln() }

pub fn gamma_component(v: f64, invgamma: f64) -> f64 { gamma_correct(v, invgamma) }

pub fn clamp_byte_component(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_color_component_f64(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_channel_component(v: f64) -> u8 { clamp_u8(v) }

pub fn safe_powf_component(a: f64, b: f64) -> f64 { a.powf(b) }

pub fn safe_expf_component(a: f64) -> f64 { a.exp() }

pub fn safe_sinf_component(a: f64) -> f64 { a.sin() }

pub fn safe_sqrtf_component(a: f64) -> f64 { a.sqrt() }

pub fn safe_absf_component(a: f64) -> f64 { a.abs() }

pub fn safe_tanf_component(a: f64) -> f64 { a.tan() }

pub fn safe_logf_component(a: f64) -> f64 { a.ln() }

pub fn clamp_byte_generic(v: f64) -> u8 { clamp_u8(v) }

pub fn gamma_fix(v: f64) -> f64 { v } // placeholder

pub fn mix_component(a: f64, b: f64, t: f64) -> f64 { a + (b - a) * t }

pub fn clamp_component(v: f64) -> f64 { clamp01(v) }

pub fn to_byte_component(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_byte_final(v: f64) -> u8 { clamp_u8(v) }

pub fn to_u8_final(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_rgb(v: f64) -> u8 { clamp_u8(v) }

pub fn clamp_255_u8(v: f64) -> u8 { clamp_u8(v) }

pub fn max0f(v: f64) -> f64 { if v > 0.0 { v } else { 0.0 } }

pub fn min0f(v: f64) -> f64 { if v < 0.0 { v } else { 0.0 } }

// The core functions from original C code translated to Rust

pub fn AutoExposure(myScene: &mut scene) -> f64 {
    let mut exposure: f64 = -1.0;
    let mut accumulationfactor: f64 = f64::from(int_max(myScene.width, myScene.height));
    let mut projectionDistance: f64 = 0.0;

    if (myScene.persp.r#type as i32) == CONIC && myScene.persp.FOV > 0.0 && myScene.persp.FOV < 189.0 {
        projectionDistance = 0.5 * (myScene.width as f64) / tanf((PIOVER180) * 0.5 * myScene.persp.FOV);
    }

    accumulationfactor = accumulationfactor / (ACCUMULATION_SIZE as f64);
    let mut mediumPoint = colour { red: 0.0, green: 0.0, blue: 0.0 };
    let mediumPointWeight = 1.0f64 / ((ACCUMULATION_SIZE * ACCUMULATION_SIZE) as f64);

    for y in 0..ACCUMULATION_SIZE {
        for x in 0..ACCUMULATION_SIZE {
            if (myScene.persp.r#type as i32) == ORTHOGONAL || projectionDistance == 0.0 {
                let mut viewRay = ray {
                    start: vector {
                        x: (x as f64) * accumulationfactor,
                        y: (y as f64) * accumulationfactor,
                        z: -10000.0,
                    },
                    dir: vector { x: 0.0, y: 0.0, z: 1.0 },
                };
                let mut currentColor = raytrace(&mut viewRay, myScene);
                let tmp = colourMul(&currentColor, &currentColor);
                let tmp = colourCoefMul(mediumPointWeight, &tmp);
                mediumPoint = colourAdd(&mediumPoint, &tmp);
            } else {
                let dir = vector {
                    x: (((x as f64) * accumulationfactor) - 0.5 * (myScene.width as f64)) / projectionDistance,
                    y: (((y as f64) * accumulationfactor) - 0.5 * (myScene.height as f64)) / projectionDistance,
                    z: 1.0,
                };
                let mut norm = vectorDot(&dir, &dir);
                if norm == 0.0 { break; }
                let dir = vectorScale(invsqrtf(norm), &dir);
                let mut viewRay = ray {
                    start: vector { x: 0.5 * (myScene.width as f64), y: 0.5 * (myScene.height as f64), z: 0.0 },
                    dir: vector { x: dir.x, y: dir.y, z: dir.z },
                };
                let mut currentColor = raytrace(&mut viewRay, myScene);
                let tmp = colourMul(&currentColor, &currentColor);
                let tmp = colourCoefMul(mediumPointWeight, &tmp);
                mediumPoint = colourAdd(&mediumPoint, &tmp);
            }
        }
    }

    let mediumLuminance = sqrtf(0.2126 * mediumPoint.red + 0.715160 * mediumPoint.green + 0.072169 * mediumPoint.blue);
    if mediumLuminance > 0.001 {
        exposure = logf(0.6) / mediumLuminance;
    }
    exposure
}

pub fn raytrace(viewRay: &mut ray, myScene: &mut scene) -> colour {
    let mut output = colour { red: 0.0, green: 0.0, blue: 0.0 };
    let mut coef = 1.0;
    let mut level = 0;

    loop {
        let mut hitpoint = vector::default();
        let mut n = vector::default();
        let mut currentSphere: isize = -1;
        let mut currentTriangle: isize = -1;
        let mut currentMat = material::default();
        let mut t: f64 = 20000.0;
        let mut temp: f64;
        let mut n1 = vector::default();

        for i in 0..myScene.numSpheres {
            let mut tv = t;
            if collideRaySphere(viewRay, &myScene.spheres[i], &mut tv) {
                t = tv;
                currentSphere = i as isize;
            }
        }

        for i in 0..myScene.numTriangles {
            let mut tv = t;
            if collideRayTriangle(viewRay, &myScene.triangles[i], &mut tv, &mut n1) {
                t = tv;
                currentTriangle = i as isize;
                currentSphere = -1;
            }
        }

        if currentSphere != -1 {
            let scaled = vectorScale(t, &viewRay.dir);
            hitpoint = vectorAdd(&viewRay.start, &scaled);
            n = vectorSub(&hitpoint, &myScene.spheres[currentSphere as usize].pos);
            temp = vectorDot(&n, &n);
            if temp == 0.0 { break; }
            temp = invsqrtf(temp);
            n = vectorScale(temp, &n);
            currentMat = myScene.materials[myScene.spheres[currentSphere as usize].material].clone();
        } else if currentTriangle != -1 {
            let scaled = vectorScale(t, &viewRay.dir);
            hitpoint = vectorAdd(&viewRay.start, &scaled);
            n = n1;
            temp = vectorDot(&n, &n);
            if temp == 0.0 { break; }
            temp = invsqrtf(temp);
            n = vectorScale(temp, &n);
            currentMat = myScene.materials[myScene.triangles[currentTriangle as usize].material].clone();
        } else {
            let test = colour { red: 0.05, green: 0.05, blue: 0.35 };
            let tmp = colourCoefMul(coef, &test);
            output = colourAdd(&output, &tmp);
            break;
        }

        if currentMat.bump != 0.0 {
            let noiseCoefx = noise(0.1 * hitpoint.x, 0.1 * hitpoint.y, 0.1 * hitpoint.z, myNoise.get_or_init(|| Arc::new(Mutex::new(perlin::default()))));
            let noiseCoefy = noise(0.1 * hitpoint.y, 0.1 * hitpoint.z, 0.1 * hitpoint.x, myNoise.get_or_init(|| Arc::new(Mutex::new(perlin::default()))));
            let noiseCoefz = noise(0.1 * hitpoint.z, 0.1 * hitpoint.x, 0.1 * hitpoint.y, myNoise.get_or_init(|| Arc::new(Mutex::new(perlin::default()))));

            n.x = (1.0 - currentMat.bump) * n.x + currentMat.bump * noiseCoefx;
            n.y = (1.0 - currentMat.bump) * n.y + currentMat.bump * noiseCoefy;
            n.z = (1.0 - currentMat.bump) * n.z + currentMat.bump * noiseCoefz;

            temp = vectorDot(&n, &n);
            if temp == 0.0 { break; }
            temp = invsqrtf(temp);
            n = vectorScale(temp, &n);
        }

        let mut inside = false;
        if vectorDot(&n, &viewRay.dir) > 0.0 {
            n = vectorScale(-1.0, &n);
            inside = true;
        } else {
            inside = false;
        }

        if !inside {
            let mut lightRay = ray { start: hitpoint.clone(), dir: vector::default() };

            for j in 0..myScene.numLights {
                let currentLight = myScene.lights[j].clone();
                lightRay.dir = vectorSub(&currentLight.pos, &hitpoint);

                let mut lightprojection = vectorDot(&lightRay.dir, &n);

                if lightprojection <= 0.0 { continue; }

                let mut lightdist = vectorDot(&lightRay.dir, &lightRay.dir);
                let mut temp_ld = lightdist;
                if temp_ld == 0.0 { continue; }
                temp_ld = invsqrtf(temp_ld);
                lightRay.dir = vectorScale(temp_ld, &lightRay.dir);
                lightprojection = temp_ld * lightprojection;

                let mut inshadow = false;
                let mut t_shadow = lightdist;

                for k in 0..myScene.numSpheres {
                    let mut tt = t_shadow;
                    if collideRaySphere(&lightRay, &myScene.spheres[k], &mut tt) {
                        inshadow = true;
                        break;
                    }
                }

                for k in 0..myScene.numTriangles {
                    let mut tt = t_shadow;
                    let mut ntri = vector::default();
                    if collideRayTriangle(&lightRay, &myScene.triangles[k], &mut tt, &mut ntri) {
                        inshadow = true;
                        break;
                    }
                }

                if !inshadow {
                    let lambert = vectorDot(&lightRay.dir, &n) * coef;
                    let mut noiseCoef = 0.0;
                    let mut lvl = 0;
                    match currentMat.MatType {
                        TURBULENCE => {
                            for level in 1..10 {
                                noiseCoef += (1.0 / (level as f64)) * fabsf(noise(level as f64 * 0.05 * hitpoint.x, level as f64 * 0.05 * hitpoint.y, level as f64 * 0.05 * hitpoint.z, myNoise.get_or_init(|| Arc::new(Mutex::new(perlin::default())))));
                            }
                            let diff1 = colourCoefMul(noiseCoef, &currentMat.diffuse);
                            let diff2 = colourCoefMul((1.0 - noiseCoef), &currentMat.mdiffuse);
                            let temp1 = colourAdd(&diff1, &diff2);
                            output = colourAdd(&output, &temp1);
                        }
                        MARBLE => {
                            for level in 1..10 {
                                noiseCoef += (1.0 / (level as f64)) * fabsf(noise(level as f64 * 0.05 * hitpoint.x, level as f64 * 0.05 * hitpoint.y, level as f64 * 0.05 * hitpoint.z, myNoise.get_or_init(|| Arc::new(Mutex::new(perlin::default())))));
                            }
                            noiseCoef = 0.5 * sinf((hitpoint.x + hitpoint.y) * 0.05 + noiseCoef) + 0.5;
                            let diff3 = colourCoefMul(noiseCoef, &currentMat.diffuse);
                            let diff4 = colourCoefMul((1.0 - noiseCoef), &currentMat.mdiffuse);
                            let tmp1 = colourAdd(&diff3, &diff4);

                            let lamint = colourCoefMul(lambert, &currentLight.intensity);
                            let lamint_scaled = colourCoefMul(coef, &lamint);

                            let temp2 = colourMul(&lamint_scaled, &tmp1);
                            output = colourAdd(&output, &temp2);
                        }
                        _ => {
                            output.red += lambert * currentLight.intensity.red * currentMat.diffuse.red;
                            output.green += lambert * currentLight.intensity.green * currentMat.diffuse.green;
                            output.blue += lambert * currentLight.intensity.blue * currentMat.diffuse.blue;
                        }
                    }

                    let viewprojection = vectorDot(&viewRay.dir, &n);
                    let blinnDir = vectorSub(&lightRay.dir, &viewRay.dir);
                    let mut tempb = vectorDot(&blinnDir, &blinnDir);
                    if tempb != 0.0 {
                        let mut blinn = invsqrtf(tempb) * max_f64(lightprojection - viewprojection, 0.0);
                        blinn = coef * powf(blinn, currentMat.power);
                        let tmp_1 = colourCoefMul(blinn, &currentMat.specular);
                        let tmp_2 = colourMul(&tmp_1, &currentLight.intensity);
                        output = colourAdd(&output, &tmp_2);
                    }
                }
            }

            coef *= currentMat.reflection;
            let reflect = 2.0 * vectorDot(&viewRay.dir, &n);
            viewRay.start = hitpoint;
            let tmpv = vectorScale(reflect, &n);
            viewRay.dir = vectorSub(&viewRay.dir, &tmpv);
        }

        level += 1;
        if !(coef > 0.0 && level < 10) { break; }
    }

    output
}

pub fn renderThread(arg: &mut thread_info) {
    let tnum = arg.thread_num;
    let secsize = {
        let s = myScene.get_or_init(|| Arc::new(Mutex::new(scene::default()))).lock().unwrap();
        let h = s.height;
        (h / (NUMTHREADS as i32))
    };
    set_sectionsize_direct(secsize);

    let limits0 = tnum * secsize;
    let limits1 = (tnum * secsize) + secsize;

    let mut my_scene_guard = myScene.get_or_init(|| Arc::new(Mutex::new(scene::default()))).lock().unwrap();
    let exposure = AutoExposure(&mut my_scene_guard);
    let mut projectionDistance = 0.0;
    if (my_scene_guard.persp.r#type as i32) == CONIC && my_scene_guard.persp.FOV > 0.0 && my_scene_guard.persp.FOV < 189.0 {
        projectionDistance = 0.5 * my_scene_guard.width as f64 / tanf((PIOVER180) * 0.5 * my_scene_guard.persp.FOV);
    }
    drop(my_scene_guard);

    for y in limits0..limits1 {
        for x in 0..myScene.get_or_init(|| Arc::new(Mutex::new(scene::default()))).lock().unwrap().width {
            let mut output = colour::default();
            for fragmentx in (0..2).map(|i| x as f64 + i as f64 * 0.5) {
                for fragmenty in (0..2).map(|i| y as f64 + i as f64 * 0.5) {
                    let sampleRatio = 0.25;
                    let mut temp = colour::default();
                    let mut totalWeight = 0.0;

                    let mut scene_guard = myScene.get_or_init(|| Arc::new(Mutex::new(scene::default()))).lock().unwrap();
                    if (scene_guard.persp.r#type as i32) == ORTHOGONAL || projectionDistance == 0.0 {
                        for _i in 0..scene_guard.complexity {
                            let mut viewRay = ray { start: vector { x: fragmentx, y: fragmenty, z: -10000.0 }, dir: vector { x: 0.0, y: 0.0, z: 1.0 } };
                            let result = raytrace(&mut viewRay, &mut scene_guard);
                            totalWeight += 1.0;
                            temp = colourAdd(&temp, &result);
                        }
                        if totalWeight != 0.0 {
                            temp = colourCoefMul(1.0 / totalWeight, &temp);
                        }
                    } else {
                        let mut dir = vector {
                            x: (fragmentx - 0.5 * scene_guard.width as f64) / projectionDistance,
                            y: (fragmenty - 0.5 * scene_guard.height as f64) / projectionDistance,
                            z: 1.0,
                        };
                        let mut norm = vectorDot(&dir, &dir);
                        if norm == 0.0 { continue; }
                        dir = vectorScale(invsqrtf(norm), &dir);
                        let start = vector { x: 0.5 * scene_guard.width as f64, y: 0.5 * scene_guard.height as f64, z: 0.0 };
                        let tmpv = vectorScale(scene_guard.persp.clearPoint, &dir);
                        let observed = vectorAdd(&start, &tmpv);

                        for _i in 0..scene_guard.complexity {
                            let mut viewRay = ray { start: start.clone(), dir: dir.clone() };
                            if scene_guard.persp.dispersion != 0.0 {
                                let disturbance = sample_random_disturbance(scene_guard.persp.dispersion);
                                viewRay.start = vectorAdd(&viewRay.start, &disturbance);
                                viewRay.dir = vectorSub(&observed, &viewRay.start);
                                let mut norm = vectorDot(&viewRay.dir, &viewRay.dir);
                                if norm == 0.0 { break; }
                                viewRay.dir = vectorScale(invsqrtf(norm), &viewRay.dir);
                            }
                            let result = raytrace(&mut viewRay, &mut scene_guard);
                            totalWeight += 1.0;
                            temp = colourAdd(&temp, &result);
                        }
                        if totalWeight != 0.0 {
                            temp = colourCoefMul(1.0 / totalWeight, &temp);
                        }
                    }
                    drop(scene_guard);

                    temp.blue = 1.0 - expf(temp.blue * exposure);
                    temp.red = 1.0 - expf(temp.red * exposure);
                    temp.green = 1.0 - expf(temp.green * exposure);

                    let adjusted = colourCoefMul(sampleRatio, &temp);
                    output = colourAdd(&output, &adjusted);
                }
            }

            let invgamma = 0.45;
            output.blue = powf(output.blue, invgamma);
            output.red = powf(output.red, invgamma);
            output.green = powf(output.green, invgamma);

            let img_arc = img.get_or_init(|| Arc::new(Mutex::new(vec![]))).clone();
            let mut img_guard = img_arc.lock().unwrap();
            if img_guard.len() == 0 {
                // initialize if not yet
                let scene_guard = myScene.get_or_init(|| Arc::new(Mutex::new(scene::default()))).lock().unwrap();
                *img_guard = vec![0u8; (scene_guard.width * scene_guard.height * 3) as usize];
            }
            let scene_guard = myScene.get_or_init(|| Arc::new(Mutex::new(scene::default()))).lock().unwrap();
            let idx = ((x + y * scene_guard.width) as usize) * 3;
            if idx + 2 < img_guard.len() {
                img_guard[idx + 2] = clamp_u8(output.red * 255.0);
                img_guard[idx + 1] = clamp_u8(output.green * 255.0);
                img_guard[idx + 0] = clamp_u8(output.blue * 255.0);
            }
            drop(scene_guard);
            drop(img_guard);
        }
    }
}

pub fn main() {
    let mut mine = scene::default();
    mine.materials = Vec::new();
    mine.spheres = Vec::new();
    mine.triangles = Vec::new();
    mine.lights = Vec::new();

    let scene_arc = Arc::new(Mutex::new(mine));
    myScene.set(scene_arc.clone()).ok();

    let noise = perlin::default();
    let noise_arc = Arc::new(Mutex::new(noise));
    myNoise.set(noise_arc.clone()).ok();
    noise_init(myNoise.get_or_init(|| Arc::new(Mutex::new(perlin::default()))));

    let mut tinfo: Vec<Arc<Mutex<thread_info>>> = Vec::with_capacity(NUMTHREADS);
    for _ in 0..NUMTHREADS {
        tinfo.push(Arc::new(Mutex::new(thread_info::default())));
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("specify scene file. ");
        return;
    }

    {
        let mut scene_guard = myScene.get().unwrap().lock().unwrap();
        if tokenizer(&args[1], &mut scene_guard) == -1 {
            return;
        }
    }

    if args.len() == 3 {
        let mut object = Obj3DS { polygons_qty: 0, vertex: Vec::new(), polygon: Vec::new() };
        Load3DS(&mut object, &args[2]);
        if object.polygons_qty > 0 {
            let mut scene_guard = myScene.get().unwrap().lock().unwrap();
            let qty = object.polygons_qty as usize;
            scene_guard.triangles = vec![triangle::default(); qty];
            scene_guard.numTriangles = qty;
            for index in 0..qty {
                if index < object.vertex.len() && index < object.polygon.len() {
                    scene_guard.triangles[index].v2.x = object.vertex[object.polygon[index].a].x + 130.0;
                    scene_guard.triangles[index].v2.y = object.vertex[object.polygon[index].a].y + 150.0;
                    scene_guard.triangles[index].v2.z = object.vertex[object.polygon[index].a].z + 350.0;
                    scene_guard.triangles[index].v2 = vectorScale(2.0, &scene_guard.triangles[index].v2);

                    scene_guard.triangles[index].v1.x = object.vertex[object.polygon[index].b].x + 130.0;
                    scene_guard.triangles[index].v1.y = object.vertex[object.polygon[index].b].y + 150.0;
                    scene_guard.triangles[index].v1.z = object.vertex[object.polygon[index].b].z + 350.0;
                    scene_guard.triangles[index].v1 = vectorScale(2.0, &scene_guard.triangles[index].v1);

                    scene_guard.triangles[index].v3.x = object.vertex[object.polygon[index].c].x + 130.0;
                    scene_guard.triangles[index].v3.y = object.vertex[object.polygon[index].c].y + 150.0;
                    scene_guard.triangles[index].v3.z = object.vertex[object.polygon[index].c].z + 350.0;
                    scene_guard.triangles[index].v3 = vectorScale(2.0, &scene_guard.triangles[index].v3);

                    scene_guard.triangles[index].material = 3;
                }
            }
        }
    }

    {
        let scene_guard = myScene.get().unwrap().lock().unwrap();
        let bytes = (3 * scene_guard.width * scene_guard.height) as usize;
        let mut img_vec = vec![0u8; bytes];
        img.get_or_init(|| Arc::new(Mutex::new(img_vec))).clone();
    }

    {
        let scene_guard = myScene.get().unwrap().lock().unwrap();
        let sec = scene_guard.height / (NUMTHREADS as i32);
        set_sectionsize_direct(sec);
        if (sec % 2) != 0 {
            println!("Warning: Height/numthreads not even - there will be errors in the image! ");
        }
    }

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::with_capacity(NUMTHREADS);
    for t in 0..NUMTHREADS {
        let tinfo_arc = tinfo[t].clone();
        {
            let mut guard = tinfo_arc.lock().unwrap();
            guard.thread_num = t as i32;
        }
        let handle = {
            let tinfo_for_thread = tinfo_arc.clone();
            thread::spawn(move || {
                let mut guard = tinfo_for_thread.lock().unwrap();
                renderThread(&mut *guard);
            })
        };
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    savebmp("out.bmp", img.get_or_init(|| Arc::new(Mutex::new(vec![]))).clone(), myScene.get_or_init(|| Arc::new(Mutex::new(scene::default()))).clone());

    // Resources will be automatically freed by Rust's drop.
}
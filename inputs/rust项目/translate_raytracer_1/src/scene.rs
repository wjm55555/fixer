use crate::sphere::*;
use crate::triangle::*;
#[repr(C)]
pub struct scene {
    pub materials: Vec<material>,
    pub spheres: Vec<sphere>,
    pub triangles: Vec<triangle>,
    pub lights: Vec<light>,

    pub width: i32,
    pub height: i32,

    pub numSpheres: i32,
    pub numMaterials: i32,
    pub numLights: i32,
    pub numTriangles: i32,

    pub persp: perspective,
    pub complexity: i32,
}

impl Default for scene {
    fn default() -> Self {
        scene {
            materials: Vec::new(),
            spheres: Vec::new(),
            triangles: Vec::new(),
            lights: Vec::new(),
            width: 0,
            height: 0,
            numSpheres: 0,
            numMaterials: 0,
            numLights: 0,
            numTriangles: 0,
            persp: Default::default(),
            complexity: 0,
        }
    }
}
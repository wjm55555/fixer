use crate::def::*;
use std::fs::{File, metadata};
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::io::AsRawFd;

pub const MAX_VERTICES: usize = 10000;
pub const MAX_POLYGONS: usize = 10000;

#[repr(C)]
#[derive(Default, Clone)]
pub struct vertex_type {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct polygon_type {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct mapcoord_type {
    pub u: f32,
    pub v: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct obj_type {
    pub name: [u8; 20],
    pub vertices_qty: i32,
    pub polygons_qty: i32,
    pub vertex: [vertex_type; MAX_VERTICES],
    pub polygon: [polygon_type; MAX_POLYGONS],
    pub mapcoord: [mapcoord_type; MAX_VERTICES],
}

impl Default for obj_type {
    fn default() -> Self {
        obj_type {
            name: [0u8; 20],
            vertices_qty: 0,
            polygons_qty: 0,
            vertex: [vertex_type::default(); MAX_VERTICES],
            polygon: [polygon_type::default(); MAX_POLYGONS],
            mapcoord: [mapcoord_type::default(); MAX_VERTICES],
        }
    }
}

pub const PI: f32 = std::f32::consts::PI;

pub fn filelength(f: i32) -> i64 {
    // Try to read the file length through /proc/self/fd/<fd> metadata (Linux/Unix).
    // If not available, return 0.
    let path = format!("/proc/self/fd/{}", f);
    match metadata(path) {
        Ok(m) => m.len() as i64,
        Err(_) => 0,
    }
}

pub fn rotate(x: &mut f32, y: &mut f32, z: &mut f32) {
    let x2: f32;
    let y2: f32;
    let z2: f32;
    let x3: f32;
    let y3: f32;
    let z3: f32;
    let x4: f32;
    let y4: f32;
    let z4: f32;

    /* rotation angle z-axis, 0 to 2pi */
    let angle_z = 0.5_f32 * PI / 3.0_f32;
    x2 = *x * angle_z.cos() - *y * angle_z.sin();
    y2 = *x * angle_z.sin() + *y * angle_z.cos();
    z2 = *z;

    /* rotate x-axis */
    let angle_x = 1.8_f32 * PI / 2.0_f32;
    x3 = x2 * angle_x.cos() - z2 * angle_x.sin();
    y3 = y2;
    z3 = x2 * angle_x.sin() + z2 * angle_x.cos();

    /* rotate y-axis */
    let angle_y = 2.2_f32 * PI / 3.0_f32;
    x4 = x3;
    y4 = y3 * angle_y.cos() - z3 * angle_y.sin();
    z4 = y3 * angle_y.sin() + z3 * angle_y.cos();

    /* update variables */
    *x = x4;
    *y = y4;
    *z = z4;
}

pub fn Load3DS(object: &mut obj_type, filename: &str) -> i8 {
    use std::io::ErrorKind;

    let file_res = File::open(filename);
    if file_res.is_err() {
        return 0;
    }
    let mut file = file_res.unwrap();

    loop {
        let pos_res = file.stream_position();
        if pos_res.is_err() {
            break;
        }
        let pos = pos_res.unwrap();

        let fd = file.as_raw_fd();
        let flen = filelength(fd);
        if flen < 0 {
            break;
        }
        if pos >= flen as u64 {
            break;
        }

        // read chunk_id (unsigned short) little-endian
        let mut buf2 = [0u8; 2];
        if let Err(e) = file.read_exact(&mut buf2) {
            if e.kind() == ErrorKind::UnexpectedEof {
                break;
            }
            println!("Error reading 3ds chunk header. ");
            break;
        }
        let chunk_id = u16::from_le_bytes(buf2);
        println!("ChunkID: {:x}", chunk_id);

        // read chunk_length (unsigned int) little-endian
        let mut buf4 = [0u8; 4];
        if let Err(_) = file.read_exact(&mut buf4) {
            println!("Error reading 3ds chunk length. ");
            break;
        }
        let chunk_length = u32::from_le_bytes(buf4);
        println!("ChunkLenght: {:x}", chunk_length);

        match chunk_id {
            0x4d4d => {
                // MAIN3DS
            }
            0x3d3d => {
                // EDIT3DS
            }
            0x4000 => {
                // EDIT_OBJECT: read name until null or 20 bytes
                let mut i = 0usize;
                loop {
                    let mut b = [0u8; 1];
                    if let Err(_) = file.read_exact(&mut b) {
                        println!("Error reading name.");
                        break;
                    }
                    object.name[i] = b[0];
                    if b[0] == 0 || i + 1 >= 20 {
                        break;
                    }
                    i += 1;
                }
            }
            0x4100 => {
                // OBJ_TRIMESH
            }
            0x4110 => {
                // TRI_VERTEXL
                let mut qbuf = [0u8; 2];
                if let Err(_) = file.read_exact(&mut qbuf) {
                    println!("Error reading 3ds TRI_VERTEXL. ");
                    continue;
                }
                let quantity = u16::from_le_bytes(qbuf);
                object.vertices_qty = quantity as i32;
                println!("Number of vertices: {}", quantity);
                for i in 0..(quantity as usize) {
                    let mut fb = [0u8; 4];
                    if let Err(_) = file.read_exact(&mut fb) {
                        println!("Error reading 3ds TRI_VERTEXL vertex x. ");
                        break;
                    }
                    object.vertex[i].x = f32::from_le_bytes(fb);
                    println!("Vertices list x: {}", object.vertex[i].x);

                    if let Err(_) = file.read_exact(&mut fb) {
                        println!("Error reading 3ds vertex y. ");
                        break;
                    }
                    object.vertex[i].y = f32::from_le_bytes(fb);
                    println!("Vertices list y: {}", object.vertex[i].y);

                    if let Err(_) = file.read_exact(&mut fb) {
                        println!("Error reading 3ds vertex z. ");
                        break;
                    }
                    object.vertex[i].z = f32::from_le_bytes(fb);
                    println!("Vertices list z: {}", object.vertex[i].z);

                    rotate(&mut object.vertex[i].x, &mut object.vertex[i].y, &mut object.vertex[i].z);
                }
            }
            0x4120 => {
                // TRI_FACEL1
                let mut qbuf = [0u8; 2];
                if let Err(_) = file.read_exact(&mut qbuf) {
                    println!("Error reading 3ds TRI_FACEL1. ");
                    continue;
                }
                let quantity = u16::from_le_bytes(qbuf);
                object.polygons_qty = quantity as i32;
                println!("Number of polygons: {}", quantity);
                for i in 0..(quantity as usize) {
                    let mut sb = [0u8; 2];
                    if let Err(_) = file.read_exact(&mut sb) {
                        println!("Error reading 3ds TRI_FACEL1 plygon a. ");
                        break;
                    }
                    object.polygon[i].a = u16::from_le_bytes(sb) as i32;
                    println!("Polygon point a: {}", object.polygon[i].a);

                    if let Err(_) = file.read_exact(&mut sb) {
                        println!("Error reading 3ds TRI_FACEL1 plygon b. ");
                        break;
                    }
                    object.polygon[i].b = u16::from_le_bytes(sb) as i32;
                    println!("Polygon point b: {}", object.polygon[i].b);

                    if let Err(_) = file.read_exact(&mut sb) {
                        println!("Error reading 3ds TRI_FACEL1 plygon c. ");
                        break;
                    }
                    object.polygon[i].c = u16::from_le_bytes(sb) as i32;
                    println!("Polygon point c: {}", object.polygon[i].c);

                    let mut ff = [0u8; 2];
                    if let Err(_) = file.read_exact(&mut ff) {
                        println!("Error reading face flags. ");
                        break;
                    }
                    let face_flags = u16::from_le_bytes(ff);
                    println!("Face flags: {:x}", face_flags);
                }
            }
            0x4140 => {
                // TRI_MAPPINGCOORS
                let mut qbuf = [0u8; 2];
                if let Err(_) = file.read_exact(&mut qbuf) {
                    println!("Error reading 3ds TRI_MAPPINGCOORDS");
                    continue;
                }
                let quantity = u16::from_le_bytes(qbuf);
                for i in 0..(quantity as usize) {
                    let mut fb = [0u8; 4];
                    if let Err(_) = file.read_exact(&mut fb) {
                        println!("Error reading mapping coordinate u. ");
                        break;
                    }
                    object.mapcoord[i].u = f32::from_le_bytes(fb);
                    println!("Mapping list u: {}", object.mapcoord[i].u);

                    if let Err(_) = file.read_exact(&mut fb) {
                        println!("Error reading mapping coordinate v. ");
                        break;
                    }
                    object.mapcoord[i].v = f32::from_le_bytes(fb);
                    println!("Mapping list v: {}", object.mapcoord[i].v);
                }
            }
            _ => {
                // Skip unknown chunk
                let skip_len = if chunk_length > 6 { chunk_length - 6 } else { 0 };
                if let Err(e) = file.seek(SeekFrom::Current(skip_len as i64)) {
                    println!("Error seeking unknown chunk: {}", e);
                    break;
                }
            }
        }
    }

    // Closes implicitly when file goes out of scope
    0
}
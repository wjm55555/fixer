use crate::noise::*;
use crate::vectors::*;
use crate::scene::*;
use crate::def::*;
use crate::colour::*;
use crate::fileformat::*;
use crate::triangle::*;
use crate::mod_3dsloader::*;
use crate::sphere::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn tokenizer(file: &str, myScene: &mut scene) -> i32 {
    let delim_1 = "=";
    let delim_2 = ",";

    let mut token: Option<String> = None;
    let mut subtoken: Option<String> = None;
    let mut saveptr1: Option<String> = None;
    let mut saveptr2: Option<String> = None;

    let mut material_index: usize = 0;
    let mut sphere_index: usize = 0;
    let mut triangle_index: usize = 0;
    let mut light_index: usize = 0;

    let mut error: Option<String> = None;

    let fp = File::open(file);
    if let Ok(f) = fp {
        let reader = BufReader::new(f);
        let mut lines = reader.lines();

        while let Some(Ok(mut line)) = lines.next() {
            if line.as_bytes().get(0) == Some(&b'#') {
                continue;
            }

            if line.trim_end() == "Scene{" {
                loop {
                    let next_line = lines.next();
                    match next_line {
                        Some(Ok(l)) => {
                            error = Some(l.clone());
                            let line = l;
                            let b = line.as_bytes();
                            if b.get(0) == Some(&b'}') {
                                break;
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'm') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                let num = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                                myScene.numMaterials = num;
                                myScene.materials = vec![material::default(); num as usize];
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b's') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                let num = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                                myScene.numSpheres = num;
                                myScene.spheres = vec![sphere::default(); num as usize];
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b't') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                let num = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                                myScene.numTriangles = num;
                                myScene.triangles = vec![triangle::default(); num as usize];
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'l') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                let num = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                                myScene.numLights = num;
                                myScene.lights = vec![light::default(); num as usize];
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'w') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.width = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'h') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.height = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'P') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.persp.type_ = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'F') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.persp.FOV = saveptr1.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'C') && b.get(2) == Some(&b'l') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.persp.clearPoint = saveptr1.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'D') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.persp.dispersion = saveptr1.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'C') && b.get(2) == Some(&b'o') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.complexity = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                            }
                        }
                        Some(Err(_)) => {
                            println!("Error reading line from scene file. \n");
                            break;
                        }
                        None => break,
                    }
                }
            }

            if line.trim_end() == "Material{" {
                loop {
                    let next_line = lines.next();
                    match next_line {
                        Some(Ok(l)) => {
                            error = Some(l.clone());
                            let line = l;
                            let b = line.as_bytes();
                            if b.get(0) == Some(&b'}') {
                                break;
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'M') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.materials[material_index].MatType = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'm') && b.get(2) == Some(&b'd') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                let rest = parts[1];
                                let mut vals = rest.split(delim_2).map(|s| s.trim().to_string());
                                myScene.materials[material_index].mdiffuse.red = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.materials[material_index].mdiffuse.green = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.materials[material_index].mdiffuse.blue = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'd') && b.get(2) == Some(&b'i') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                let rest = parts[1];
                                let mut vals = rest.split(delim_2).map(|s| s.trim().to_string());
                                myScene.materials[material_index].diffuse.red = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.materials[material_index].diffuse.green = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.materials[material_index].diffuse.blue = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'r') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.materials[material_index].reflection = saveptr1.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'b') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.materials[material_index].bump = saveptr1.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b's') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                let rest = parts[1];
                                let mut vals = rest.split(delim_2).map(|s| s.trim().to_string());
                                myScene.materials[material_index].specular.red = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.materials[material_index].specular.green = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.materials[material_index].specular.blue = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'p') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.materials[material_index].power = saveptr1.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                        }
                        Some(Err(_)) => {
                            println!("Error reading Material line from scene file. \n");
                            break;
                        }
                        None => break,
                    }
                }
                material_index += 1;
            }

            if line.trim_end() == "Sphere{" {
                loop {
                    let next_line = lines.next();
                    match next_line {
                        Some(Ok(l)) => {
                            error = Some(l.clone());
                            let line = l;
                            let b = line.as_bytes();
                            if b.get(0) == Some(&b'}') {
                                break;
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'p') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                let rest = parts[1];
                                let mut vals = rest.split(delim_2).map(|s| s.trim().to_string());
                                myScene.spheres[sphere_index].pos.x = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.spheres[sphere_index].pos.y = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.spheres[sphere_index].pos.z = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b's') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.spheres[sphere_index].size = saveptr1.as_ref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'm') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.spheres[sphere_index].material = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                            }
                        }
                        Some(Err(_)) => {
                            println!("Error reading Sphere line from scene file. \n");
                            break;
                        }
                        None => break,
                    }
                }
                sphere_index += 1;
            }

            if line.trim_end() == "Light{" {
                loop {
                    let next_line = lines.next();
                    match next_line {
                        Some(Ok(l)) => {
                            error = Some(l.clone());
                            let line = l;
                            let b = line.as_bytes();
                            if b.get(0) == Some(&b'}') {
                                break;
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'p') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                let rest = parts[1];
                                let mut vals = rest.split(delim_2).map(|s| s.trim().to_string());
                                myScene.lights[light_index].pos.x = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.lights[light_index].pos.y = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.lights[light_index].pos.z = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'c') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                let rest = parts[1];
                                let mut vals = rest.split(delim_2).map(|s| s.trim().to_string());
                                myScene.lights[light_index].intensity.red = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.lights[light_index].intensity.green = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.lights[light_index].intensity.blue = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                        }
                        Some(Err(_)) => {
                            println!("Error reading Light line from scene file. \n");
                            break;
                        }
                        None => break,
                    }
                }
                light_index += 1;
            }

            if line.trim_end() == "Triangle{" {
                loop {
                    let next_line = lines.next();
                    match next_line {
                        Some(Ok(l)) => {
                            error = Some(l.clone());
                            let line = l;
                            let b = line.as_bytes();
                            if b.get(0) == Some(&b'}') {
                                break;
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'c') && b.get(7) == Some(&b'1') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                let rest = parts[1];
                                let mut vals = rest.split(delim_2).map(|s| s.trim().to_string());
                                myScene.triangles[triangle_index].v1.x = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.triangles[triangle_index].v1.y = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.triangles[triangle_index].v1.z = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'c') && b.get(7) == Some(&b'2') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                let rest = parts[1];
                                let mut vals = rest.split(delim_2).map(|s| s.trim().to_string());
                                myScene.triangles[triangle_index].v2.x = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.triangles[triangle_index].v2.y = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.triangles[triangle_index].v2.z = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'c') && b.get(7) == Some(&b'3') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                let rest = parts[1];
                                let mut vals = rest.split(delim_2).map(|s| s.trim().to_string());
                                myScene.triangles[triangle_index].v3.x = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.triangles[triangle_index].v3.y = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                                myScene.triangles[triangle_index].v3.z = vals.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                            }
                            if b.get(0) == Some(&b'\t') && b.get(1) == Some(&b'm') {
                                let parts: Vec<&str> = line.splitn(2, delim_1).collect();
                                if parts.len() < 2 {
                                    break;
                                }
                                saveptr1 = Some(parts[1].trim().to_string());
                                myScene.triangles[triangle_index].material = saveptr1.as_ref().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
                            }
                        }
                        Some(Err(_)) => {
                            println!("Error reading Triangle line from scene file. \n");
                            break;
                        }
                        None => break,
                    }
                }
                triangle_index += 1;
            }
        }

        return 0;
    } else {
        println!("File not found. \n");
        return -1;
    }
}
use crate::noise::*;
use crate::vectors::*;
use crate::scene::*;
use crate::def::*;
use crate::colour::*;
use crate::triangle::*;
use crate::tokenizer::*;
use crate::mod_3dsloader::*;
use crate::sphere::*;
use std::fs::File;
use std::io::Write;

pub fn savebmp(filename: &str, img: &[u8], myScene: &scene) {
    let mut i: usize;
    let mut error: usize = 0;
    let mut f = match File::create(filename) {
        Ok(file) => file,
        Err(_) => {
            println!("Error writing bmpfileheader ");
            return;
        }
    };

    let filesize = 54 + 3 * (myScene.width as usize) * (myScene.height as usize);

    let mut bmpfileheader: [u8; 14] = [
        'B' as u8,
        'M' as u8,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        54,
        0,
        0,
        0,
    ];
    let mut bmpinfoheader: [u8; 40] = [
        40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let bmppad: [u8; 3] = [0, 0, 0];

    bmpfileheader[2] = (filesize & 0xFF) as u8;
    bmpfileheader[3] = ((filesize >> 8) & 0xFF) as u8;
    bmpfileheader[4] = ((filesize >> 16) & 0xFF) as u8;
    bmpfileheader[5] = ((filesize >> 24) & 0xFF) as u8;

    let width = myScene.width as usize;
    let height = myScene.height as usize;

    bmpinfoheader[4] = (width & 0xFF) as u8;
    bmpinfoheader[5] = ((width >> 8) & 0xFF) as u8;
    bmpinfoheader[6] = ((width >> 16) & 0xFF) as u8;
    bmpinfoheader[7] = ((width >> 24) & 0xFF) as u8;
    bmpinfoheader[8] = (height & 0xFF) as u8;
    bmpinfoheader[9] = ((height >> 8) & 0xFF) as u8;
    bmpinfoheader[10] = ((height >> 16) & 0xFF) as u8;
    bmpinfoheader[11] = ((height >> 24) & 0xFF) as u8;

    if let Err(_) = f.write_all(&bmpfileheader) {
        println!("Error writing bmpfileheader ");
    }
    if let Err(_) = f.write_all(&bmpinfoheader) {
        println!("Error writing bmpinfoheader ");
    }

    i = 0;
    while i < height {
        let offset = width * i * 3;
        let line_len = width * 3;
        if offset + line_len <= img.len() {
            match f.write_all(&img[offset..offset + line_len]) {
                Ok(_) => {
                    error = line_len;
                }
                Err(_) => {
                    println!("Error writing line");
                    error = 0;
                }
            }
            if error != width {
            }
        } else {
            println!("Error writing line");
        }
        let pad_len = (4 - (width * 3) % 4) % 4;
        if pad_len > 0 {
            if let Err(_) = f.write_all(&bmppad[0..pad_len]) {
                println!("Error writing bmp padding");
            }
        }
        i += 1;
    }
}
use translate_cjson::cjson::*;
use std::process;

#[repr(C)]
pub struct record {
    pub precision: &'static str,
    pub lat: f64,
    pub lon: f64,
    pub address: &'static str,
    pub city: &'static str,
    pub state: &'static str,
    pub zip: &'static str,
    pub country: &'static str,
}

pub fn print_preallocated(root: &mut cJSON) -> i32 {
    let out = cJSON_Print(root);
    if out.is_none() {
        println!("Failed to print JSON.");
        return -1;
    }

    let out_str = out.unwrap();
    let len = out_str.len() + 5;
    let mut buf = vec![0u8; len];
    
    let len_fail = out_str.len();
    let mut buf_fail = vec![0u8; len_fail];

    if !cJSON_PrintPreallocated(root, &mut buf, true) {
        println!("cJSON_PrintPreallocated failed!");
        let buf_str = String::from_utf8_lossy(&buf).trim_end_matches('\0').to_string();
        if out_str != buf_str {
            println!("cJSON_PrintPreallocated not the same as cJSON_Print!");
            println!("cJSON_Print result:\n{}", out_str);
            println!("cJSON_PrintPreallocated result:\n{}", buf_str);
        }
        return -1;
    }

    let buf_str = String::from_utf8_lossy(&buf).trim_end_matches('\0').to_string();
    println!("{}", buf_str);

    if cJSON_PrintPreallocated(root, &mut buf_fail, true) {
        println!("cJSON_PrintPreallocated failed to show error with insufficient memory!");
        println!("cJSON_Print result:\n{}", out_str);
        println!("cJSON_PrintPreallocated result:\n{}", String::from_utf8_lossy(&buf_fail).trim_end_matches('\0'));
        return -1;
    }

    0
}

pub fn create_objects() {
    let mut root: Option<Box<cJSON>> = None;
    let mut i: i32 = 0;

    let strings: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    let numbers: [[i32; 3]; 3] = [
        [0, -1, 0],
        [1, 0, 0],
        [0, 0, 1],
    ];

    let ids: [i32; 4] = [116, 943, 234, 38793];

    let fields: [record; 2] = [
        record {
            precision: "zip",
            lat: 37.7668,
            lon: -1.223959e+2,
            address: "",
            city: "SAN FRANCISCO",
            state: "CA",
            zip: "94107",
            country: "US",
        },
        record {
            precision: "zip",
            lat: 37.371991,
            lon: -1.22026e+2,
            address: "",
            city: "SUNNYVALE",
            state: "CA",
            zip: "94085",
            country: "US",
        },
    ];

    let zero: f64 = 0.0;

    root = Some(Box::new(cJSON_CreateObject()));
    if let Some(ref mut r) = root {
        cJSON_AddItemToObject(r, "name", cJSON_CreateString("Jack (\"Bee\") Nimble"));
        
        let mut format_obj = cJSON_CreateObject();
        cJSON_AddStringToObject(&mut format_obj, "type", "rect");
        cJSON_AddNumberToObject(&mut format_obj, "width", 1920.0);
        cJSON_AddNumberToObject(&mut format_obj, "height", 1080.0);
        cJSON_AddFalseToObject(&mut format_obj, "interlace");
        cJSON_AddNumberToObject(&mut format_obj, "frame rate", 24.0);
        cJSON_AddItemToObject(r, "format", format_obj);
    }

    if let Some(ref mut r) = root {
        if print_preallocated(r) != 0 {
            cJSON_Delete(root.take());
            process::exit(1);
        }
    }
    cJSON_Delete(root.take());

    root = Some(Box::new(cJSON_CreateStringArray(&strings, 7)));
    if let Some(ref mut r) = root {
        if print_preallocated(r) != 0 {
            cJSON_Delete(root.take());
            process::exit(1);
        }
    }
    cJSON_Delete(root.take());

    root = Some(Box::new(cJSON_CreateArray()));
    i = 0;
    while i < 3 {
        if let Some(ref mut r) = root {
            cJSON_AddItemToArray(r, cJSON_CreateIntArray(&numbers[i as usize], 3));
        }
        i += 1;
    }

    if let Some(ref mut r) = root {
        if print_preallocated(r) != 0 {
            cJSON_Delete(root.take());
            process::exit(1);
        }
    }
    cJSON_Delete(root.take());

    root = Some(Box::new(cJSON_CreateObject()));
    if let Some(ref mut r) = root {
        let mut img = cJSON_CreateObject();
        cJSON_AddNumberToObject(&mut img, "Width", 800.0);
        cJSON_AddNumberToObject(&mut img, "Height", 600.0);
        cJSON_AddStringToObject(&mut img, "Title", "View from 15th Floor");
        
        let mut thm = cJSON_CreateObject();
        cJSON_AddItemToObject(&mut img, "Thumbnail", thm);
        
        cJSON_AddItemToObject(r, "Image", img);
    }

    root = Some(Box::new(cJSON_CreateArray()));
    i = 0;
    while i < 2 {
        if let Some(ref mut r) = root {
            let mut fld = cJSON_CreateObject();
            cJSON_AddStringToObject(&mut fld, "precision", fields[i as usize].precision);
            cJSON_AddNumberToObject(&mut fld, "Latitude", fields[i as usize].lat);
            cJSON_AddNumberToObject(&mut fld, "Longitude", fields[i as usize].lon);
            cJSON_AddStringToObject(&mut fld, "Address", fields[i as usize].address);
            cJSON_AddStringToObject(&mut fld, "City", fields[i as usize].city);
            cJSON_AddStringToObject(&mut fld, "State", fields[i as usize].state);
            cJSON_AddStringToObject(&mut fld, "Zip", fields[i as usize].zip);
            cJSON_AddStringToObject(&mut fld, "Country", fields[i as usize].country);
            cJSON_AddItemToArray(r, fld);
        }
        i += 1;
    }

    if let Some(ref mut r) = root {
        if print_preallocated(r) != 0 {
            cJSON_Delete(root.take());
            process::exit(1);
        }
    }
    cJSON_Delete(root.take());

    root = Some(Box::new(cJSON_CreateObject()));
    if let Some(ref mut r) = root {
        cJSON_AddNumberToObject(r, "number", 1.0 / zero);

        if print_preallocated(r) != 0 {
            cJSON_Delete(root.take());
            process::exit(1);
        }
    }
    cJSON_Delete(root.take());
}

pub fn main() {
    println!("Version: {}", cJSON_Version());
    create_objects();
}
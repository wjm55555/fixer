pub const STR_BUF_SIZE: usize = 1024;

pub fn PREFIX_UNUSED<T>(_variable: &T) {
    // no-op to mark variable as unused
}

pub fn SAFE_MALLOC<T>(_ptr: &mut Option<Box<T>>, _n: usize) {
    // placeholder - memory is managed by Rust
}

pub fn SAFE_CALLOC<T>(_ptr: &mut Option<Vec<T>>, _n: usize) {
    // placeholder - memory is managed by Rust
}

pub fn SAFE_REALLOC<T>(_ptr: &mut Option<Vec<T>>, _n: usize) {
    // placeholder - memory is managed by Rust
}

pub fn SAFE_FREE<T>(_ptr: &mut Option<Box<T>>) {
    // placeholder - Rust will drop automatically
}

pub fn PUSH_BACK<T>(arr: &mut Vec<T>, val: T) {
    arr.push(val);
}

pub type FILE = std::fs::File;
pub const EXIT_FAILURE: i32 = 1;

use std::fs::OpenOptions;
use std::io::ErrorKind;

pub fn file_open(fname: &str, mode: &str) -> *mut FILE {
    if fname == "stdin" {
        match std::fs::File::open("/dev/stdin") {
            Ok(f) => Box::into_raw(Box::new(f)),
            Err(e) => {
                eprintln!("Unable to open file \"{}\" with mode \"{}\": {}", fname, mode, e);
                std::process::exit(EXIT_FAILURE);
            }
        }
    } else if fname == "stdout" {
        match std::fs::File::open("/dev/stdout") {
            Ok(f) => Box::into_raw(Box::new(f)),
            Err(e) => {
                eprintln!("Unable to open file \"{}\" with mode \"{}\": {}", fname, mode, e);
                std::process::exit(EXIT_FAILURE);
            }
        }
    } else if fname == "stderr" {
        match std::fs::File::open("/dev/stderr") {
            Ok(f) => Box::into_raw(Box::new(f)),
            Err(e) => {
                eprintln!("Unable to open file \"{}\" with mode \"{}\": {}", fname, mode, e);
                std::process::exit(EXIT_FAILURE);
            }
        }
    } else {
        let mut opts = OpenOptions::new();

        if mode.contains('r') {
            opts.read(true);
        }
        if mode.contains('w') {
            opts.write(true);
            opts.create(true);
            opts.truncate(true);
        }
        if mode.contains('a') {
            opts.write(true);
            opts.create(true);
            opts.append(true);
        }
        if mode.contains('+') {
            opts.read(true);
            opts.write(true);
        }

        // Try to open; on failure print and exit
        match opts.open(fname) {
            Ok(f) => Box::into_raw(Box::new(f)),
            Err(e) => {
                // Provide similar message as the original C code
                eprintln!("Unable to open file \"{}\" with mode \"{}\"", fname, mode);
                // Optionally include the system error for more context
                if let Some(code) = e.raw_os_error() {
                    eprintln!("System error code: {}", code);
                } else {
                    match e.kind() {
                        ErrorKind::NotFound => eprintln!("Reason: Not found"),
                        ErrorKind::PermissionDenied => eprintln!("Reason: Permission denied"),
                        _ => {}
                    }
                }
                std::process::exit(EXIT_FAILURE);
            }
        }
    }
}
use std::io::{self, Read, Write};

pub const STR_BUF_SIZE: usize = 1024;

pub enum FileHandle {
    Stdin,
    Stdout,
    Stderr,
    File(Box<dyn ReadWrite>),
}

// Rust does not allow `dyn Read + Write` directly in all contexts; wrap it.
pub trait ReadWrite: Read + Write {}
impl<T: Read + Write> ReadWrite for T {}

pub fn file_open(fname: &str, mode: &str) -> Result<FileHandle, String> {
    match fname {
        "stdin" => Ok(FileHandle::Stdin),
        "stdout" => Ok(FileHandle::Stdout),
        "stderr" => Ok(FileHandle::Stderr),
        _ => {
            eprintln!("Unable to open file \"{}\" with mode \"{}\"", fname, mode);
            Err(format!(
                "Unable to open file \"{}\" with mode \"{}\"",
                fname, mode
            ))
        }
    }
}

#[macro_export]
macro_rules! PREFIX_UNUSED {
    ($variable:expr) => {
        let _ = $variable;
    };
}

#[macro_export]
macro_rules! SAFE_MALLOC {
    ($ptr:expr, $n:expr) => {{
        match std::panic::catch_unwind(|| vec![std::mem::MaybeUninit::<_>::uninit(); $n]) {
            Ok(vec) => {
                $ptr = Some(Box::new(vec));
            }
            Err(_) => {
                eprintln!("[{}:{}] unable to malloc {} bytes", file!(), line!(), $n);
                std::process::exit(1);
            }
        }
    }};
}

#[macro_export]
macro_rules! SAFE_CALLOC {
    ($ptr:expr, $n:expr) => {{
        match std::panic::catch_unwind(|| vec![0u8; $n]) {
            Ok(vec) => {
                $ptr = Some(Box::new(vec));
            }
            Err(_) => {
                eprintln!("[{}:{}] unable to calloc {} bytes", file!(), line!(), $n);
                std::process::exit(1);
            }
        }
    }};
}

#[macro_export]
macro_rules! SAFE_REALLOC {
    ($ptr:expr, $n:expr) => {{
        match std::panic::catch_unwind(|| {
            let new_vec = vec![0u8; $n];
            new_vec
        }) {
            Ok(vec) => {
                $ptr = Some(Box::new(vec));
            }
            Err(_) => {
                eprintln!(
                    "[{}:{}] unable to realloc to {} bytes",
                    file!(),
                    line!(),
                    $n
                );
                std::process::exit(1);
            }
        }
    }};
}

#[macro_export]
macro_rules! SAFE_FREE {
    ($ptr:expr) => {{
        $ptr = None;
    }};
}

#[macro_export]
macro_rules! PUSH_BACK {
    ($arr:ident, $val:expr) => {{
        if $arr.len() == $arr.capacity() {
            if $arr.capacity() == 0 {
                $arr.reserve(1);
            } else {
                $arr.reserve($arr.capacity());
            }
        }
        $arr.push($val);
    }};
}

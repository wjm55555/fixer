use translate_md5::md5::*;
pub fn main() {
    let mut buf = String::new();

    std::io::stdin()
        .read_line(&mut buf)
        .expect("failed to read line");
    if let Some(pos) = buf.find('\n') {
        buf.truncate(pos);
    }

    let mut out: [u8; 16] = [0u8; 16];
    md5(buf.as_bytes(), buf.len(), &mut out);

    for i in 0..16 {
        print!("{:02x}", out[i]);
    }
    println!();
}
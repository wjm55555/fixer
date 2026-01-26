pub static s: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14,
    20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15, 21, 6,
    10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

pub static K: [u32; 64] = [
    0xd76aa478,
    0xe8c7b756,
    0x242070db,
    0xc1bdceee,
    0xf57c0faf,
    0x4787c62a,
    0xa8304613,
    0xfd469501,
    0x698098d8,
    0x8b44f7af,
    0xffff5bb1,
    0x895cd7be,
    0x6b901122,
    0xfd987193,
    0xa679438e,
    0x49b40821,
    0xf61e2562,
    0xc040b340,
    0x265e5a51,
    0xe9b6c7aa,
    0xd62f105d,
    0x02441453,
    0xd8a1e681,
    0xe7d3fbc8,
    0x21e1cde6,
    0xc33707d6,
    0xf4d50d87,
    0x455a14ed,
    0xa9e3e905,
    0xfcefa3f8,
    0x676f02d9,
    0x8d2a4c8a,
    0xfffa3942,
    0x8771f681,
    0x6d9d6122,
    0xfde5380c,
    0xa4beea44,
    0x4bdecfa9,
    0xf6bb4b60,
    0xbebfbc70,
    0x289b7ec6,
    0xeaa127fa,
    0xd4ef3085,
    0x04881d05,
    0xd9d4d039,
    0xe6db99e5,
    0x1fa27cf8,
    0xc4ac5665,
    0xf4292244,
    0x432aff97,
    0xab9423a7,
    0xfc93a039,
    0x655b59c3,
    0x8f0ccc92,
    0xffeff47d,
    0x85845dd1,
    0x6fa87e4f,
    0xfe2ce6e0,
    0xa3014314,
    0x4e0811a1,
    0xf7537e82,
    0xbd3af235,
    0x2ad7d2bb,
    0xeb86d391,
];

pub fn LEFTROTATE(x: u32, c: u32) -> u32 {
    (x << c) | (x >> (32 - c))
}

pub fn md5_pad(msg: &[u8], len: usize, new_len: &mut usize) -> Vec<u8> {
    let mut padded_len = len + 1;
    while padded_len % 64 != 56 {
        padded_len += 1;
    }
    padded_len += 8;

    let mut padded = vec![0u8; padded_len];
    let copy_len = len.min(msg.len());
    padded[..copy_len].copy_from_slice(&msg[..copy_len]);

    if len <= padded_len - 1 {
        padded[len] = 0x80;
    } else if copy_len <= padded_len - 1 {
        padded[copy_len] = 0x80;
    }

    let bit_len: u64 = (len as u64) * 8;
    let bit_bytes = bit_len.to_le_bytes();
    let start = padded_len - 8;
    padded[start..start + 8].copy_from_slice(&bit_bytes);

    *new_len = padded_len;
    padded
}

pub fn md5(initial_msg: &[u8], len: usize, digest: &mut [u8; 16]) {
    let mut new_len: usize = 0;
    let msg = md5_pad(initial_msg, len, &mut new_len);

    let mut A: u32 = 0x67452301;
    let mut B: u32 = 0xefcdab89;
    let mut C: u32 = 0x98badcfe;
    let mut D: u32 = 0x10325476;

    let mut offset: usize = 0;
    while offset < new_len {
        let mut w = [0u32; 16];
        for j in 0..16 {
            let idx = offset + j * 4;
            let b0 = msg[idx];
            let b1 = msg[idx + 1];
            let b2 = msg[idx + 2];
            let b3 = msg[idx + 3];
            w[j] = u32::from_le_bytes([b0, b1, b2, b3]);
        }

        let mut a = A;
        let mut b = B;
        let mut c = C;
        let mut d = D;

        for i in 0..64 {
            let (f, g): (u32, usize) = if i < 16 {
                ((b & c) | (!b & d), i)
            } else if i < 32 {
                ((d & b) | (!d & c), (5 * i + 1) % 16)
            } else if i < 48 {
                (b ^ c ^ d, (3 * i + 5) % 16)
            } else {
                (c ^ (b | !d), (7 * i) % 16)
            };

            let temp = d;
            d = c;
            c = b;
            let sum = a
                .wrapping_add(f)
                .wrapping_add(K[i])
                .wrapping_add(w[g]);
            b = b.wrapping_add(LEFTROTATE(sum, s[i]));
            a = temp;
        }

        A = A.wrapping_add(a);
        B = B.wrapping_add(b);
        C = C.wrapping_add(c);
        D = D.wrapping_add(d);

        offset += 64;
    }

    let a_bytes = A.to_le_bytes();
    let b_bytes = B.to_le_bytes();
    let c_bytes = C.to_le_bytes();
    let d_bytes = D.to_le_bytes();

    digest[0..4].copy_from_slice(&a_bytes);
    digest[4..8].copy_from_slice(&b_bytes);
    digest[8..12].copy_from_slice(&c_bytes);
    digest[12..16].copy_from_slice(&d_bytes);
}
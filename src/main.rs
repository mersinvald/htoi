#![feature(test)]
#![feature(universal_impl_trait)]

extern crate test;

const HEX_TO_INT_TABLE: &[i8] = &[
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1, 0,1,2,3,4,5,6,7,8,9,-1,-1,-1,-1,-1,-1,-1,10,11,12,13,14,15,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,10,11,12,13,14,15,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
    -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1
];

#[inline]
fn hex_to_u8_table(byte: u8) -> u8 {
    HEX_TO_INT_TABLE[byte as usize] as u8
}

#[inline]
fn hex_to_u8(byte: u8) -> u8 {
    // Is in uppercase range
    if byte >= 65 && byte <= 90 {
        byte - 55
    // Is in 0-9 range
    } else if byte >= 48 && byte <= 57 {
        byte - '0' as u8
    // Is in lowercase range
    } else if byte >= 97 && byte <= 122 {
        byte - 87
    } else {
        panic!("non-hexadecimal input")
    }
}

#[inline(never)]
fn htoi(s: &str, conv_fn: impl Fn(u8) -> u8) -> i64 {
    // Drop the unicode as soon as it is possible
    // unicode is fucking slow
    let s = s.as_bytes();

    // Trim the prefix if there is one
    let second_char = s[1];
    let s = if second_char == 'X' as u8
                  || second_char == 'x' as u8 {
        &s[2..]
    } else {
        &s
    };

    s.iter()
        .fold(0, |result, &byte| {
            let byte = conv_fn(byte) as i64;
            (result << 4) + byte
        })
}

fn main() {
    println!("{}", htoi(test::black_box("0x1234AAFFEE7629"), hex_to_u8_table));
    println!("{}", unsafe { c::htoi_c_table("0x1234AAFFEE7629".as_ptr()) })
}


mod c {
    #[link(name="htoi", kind="static")]
    extern "C" {
        pub fn htoi_c_table(s: *const u8) -> i64;
        pub fn htoi_c(s: *const u8) -> i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUNS: u64 = 1;

    #[bench]
    fn rust_htoi(b: &mut test::Bencher) {
        let input = test::black_box("0x1234AAFFEE7629");
        b.iter(|| {
            for _ in 0..RUNS {
                assert_eq!(htoi(input, hex_to_u8), 0x1234AAFFEE7629);
            }
        })
    }

    #[bench]
    fn rust_htoi_table(b: &mut test::Bencher) {
        let input = test::black_box("0X1234AAFFEE7629");
        b.iter(|| {
            for _ in 0..RUNS {
                assert_eq!(htoi(input, hex_to_u8_table), 0x1234AAFFEE7629);
            }
        })
    }

    #[bench]
    fn c_htoi(b: &mut test::Bencher) {
        let input = test::black_box("0x1234AAFFEE7629\0".as_bytes().as_ptr());
        b.iter(|| {
            for _ in 0..RUNS {
                let answer = unsafe {
                    c::htoi_c(input)
                };
                assert_eq!(answer, 0x1234AAFFEE7629);
            }
        })
    }

    #[bench]
    fn c_htoi_table(b: &mut test::Bencher) {
        let input = test::black_box("0x1234AAFFEE7629\0".as_bytes().as_ptr());
        b.iter(|| {
            for _ in 0..RUNS {
                let answer = unsafe {
                    c::htoi_c_table(input)
                };
                assert_eq!(answer, 0x1234AAFFEE7629);
            }
        })
    }
}

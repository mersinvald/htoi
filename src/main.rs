#![feature(test)]
#![feature(universal_impl_trait)]

#[macro_use]
extern crate cpp;
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

fn htoi(s: &str, conv_fn: impl Fn(u8) -> u8) -> u64 {
    // Drop the unicode as soon as it is possible
    // unicode is fucking slow
    let s = s.as_bytes();

    // Trim the prefix if there is one
    let s = if s[1] == 'X' as u8 || s[1] == 'x' as u8 {
        &s[2..]
    } else {
        &s
    };

    s.iter()
        .rev()
        .enumerate()
        .fold(0, |result, (idx, &byte)| {
            let byte = conv_fn(byte) as u64;
            result | (byte << idx * 4)
        })
}

fn main() {
    println!("0x12345 is {}", htoi("0x12345", hex_to_u8));
    println!("0XFFFF is {}", htoi("0XFFFF", hex_to_u8_table));
}

cpp!{{
    #include <stdio.h>

    static const long hextable[] = {
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
    };

    long long htoi_c_table(char *s) {
        int offset = (s[0] == '0' && (s[1] == 'x' || s[1] == 'X')) ? 2 : 0;
        long long result = 0;
        for (char *temp = s + offset; *temp; temp++)
        {
            signed char digit = hextable[*temp];
            result = (result << 4) + digit;
        }
        return result;
    }

    long long htoi_c(char *s)
    {
        int offset = (s[0] == '0' && (s[1] == 'x' || s[1] == 'X')) ? 2 : 0;
        long long result = 0;
        for (char *temp = s + offset; *temp; temp++)
        {
            signed char digit = ((*temp >= '0') && (*temp <= '9')) ?
                (*temp - '0') :
                (((*temp >= 'A') && (*temp <= 'F')) ?
                    (*temp - 'A' + 10) :
                    (((*temp >= 'a') && (*temp <= 'f')) ?
                        (*temp - 'a' + 10) :
                        -1));
            if (digit==-1)
            {
                return -1;
            }
            result = (result << 4) + digit;
        }
        return result;
    }
}}

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
        let input = test::black_box("0x1234AAFFEE7629");
        b.iter(|| {
            for _ in 0..RUNS {
                assert_eq!(htoi(input, hex_to_u8_table), 0x1234AAFFEE7629);
            }
        })
    }

    #[bench]
    fn c_htoi(b: &mut test::Bencher) {
        let input = test::black_box("0x1234AAFFEE7629".as_bytes().as_ptr());
        b.iter(|| {
            for _ in 0..RUNS {
                let answer = unsafe {
                    cpp!([input as "char*"] -> u64 as "long long" {
                        return htoi_c(input);
                    })
                };
                assert_eq!(answer, 0x1234AAFFEE7629);
            }
        })
    }

    #[bench]
    fn c_htoi_table(b: &mut test::Bencher) {
        let input = test::black_box("0x1234AAFFEE7629".as_bytes().as_ptr());
        b.iter(|| {
            for _ in 0..RUNS {
                let answer = unsafe {
                    cpp!([input as "char*"] -> u64 as "long long" {
                        return htoi_c_table(input);
                    })
                };
                assert_eq!(answer, 0x1234AAFFEE7629);
            }
        })
    }


}

use crate::{
    check,
    vec::Vec,
};

pub fn decode_hex(s: &str) -> Vec<u8> {
    check(s.len() % 2 == 0, "decod_hex: bad hex string length");
    (0..s.len())
        .step_by(2)
        .map(|i| {
            if let Ok(c) = u8::from_str_radix(&s[i..i + 2], 16) {
                c
            } else {
                check(false, "bad hex characters");
                0u8
            }
        })
        .collect::<Vec<_>>()
}
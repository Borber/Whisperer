use crate::{Conf, hash};

pub fn encrypt(index: usize, d: &u8) -> u8 {
    ((*d as u16 + hash::mix_hash_code(index, Conf::global().key.as_bytes()) as u16) % 256) as u8
}

pub fn decrypt(index: usize, e: &u8) -> u8 {
    let u = hash::mix_hash_code(index, Conf::global().key.as_bytes());
    match *e as i16 - u as i16 {
        x if x >= 0 => x as u8,
        x if x < 0 => (x + 256) as u8,
        _ => 0
    }
}
use crate::dict::DERIVE_KEY;

pub fn mix_hash_code(index: usize, key: &[u8]) -> u16 {
    let mixed= [index.to_be_bytes().as_slice(),key].concat();
    hash_code(mixed.as_slice())
}

pub fn hash_code(s: &[u8]) -> u16 {
    blake3::Hasher::new_derive_key(DERIVE_KEY)
        .update(s)
        .finalize()
        .as_bytes()
        .iter().map(|&b| b as u16).sum::<u16>()
}
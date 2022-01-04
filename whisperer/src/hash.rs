use crate::config::Conf;

pub fn mix_hash_code(index: usize, length: usize, key: &[u8]) -> u8 {
    let mixed = [index.to_be_bytes().as_slice(), key, length.to_be_bytes().as_slice()].concat();
    hash_code(mixed.as_slice())
}

pub fn hash_code(s: &[u8]) -> u8 {
    blake3::Hasher::new_derive_key(Conf::global().derive_key.as_str())
        .update(s)
        .finalize()
        .as_bytes()[0]
}
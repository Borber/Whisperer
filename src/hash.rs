use crate::Conf;

pub fn mix_hash_code(index: usize, key: &[u8]) -> u8 {
    let mixed = [index.to_be_bytes().as_slice(), key].concat();
    hash_code(mixed.as_slice())
}

pub fn hash_code(s: &[u8]) -> u8 {
    blake3::Hasher::new_derive_key(Conf::global().derive_key.as_str())
        .update(s)
        .finalize()
        .as_bytes()[0]
}
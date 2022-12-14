use sha3::{Digest, Keccak256};

pub type KecHash = [u8; 32];

pub(crate) fn kecak256(value: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::default();
    hasher.update(value);
    hasher.finalize().into()
}

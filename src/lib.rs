use std::mem::MaybeUninit;

#[cfg(not(target_os = "solana"))]
use sha2::{Digest, Sha256};

pub const HASH_LENGTH: usize = 32;

#[cfg(target_os = "solana")]
extern "C" {
    fn sol_sha256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;
}

#[cfg(not(target_os = "solana"))]
pub fn hash(data: &[u8]) -> [u8;HASH_LENGTH] {
    hashv(&[data.as_ref()])
}

#[cfg(target_os = "solana")]
pub fn hash(data: &[u8]) -> [u8;HASH_LENGTH] {
    hashv(&[msg.as_ref()])
}

#[cfg(not(target_os = "solana"))]
pub fn hashv<T: AsRef<[u8]>>(data: &[T]) -> [u8; HASH_LENGTH] {
    let mut hasher = Sha256::new();

    for item in data {
        hasher.update(item.as_ref());
    }

    let mut hash_result = MaybeUninit::<[u8; HASH_LENGTH]>::uninit();

    unsafe {
        hasher.finalize_into(hash_result.assume_init_mut().into());
        hash_result.assume_init()
    }
}

#[cfg(target_os = "solana")]
pub fn hashv(data: &[&[u8]]) -> [u8;HASH_LENGTH] {
    let mut hash_result = MaybeUninit::<[u8; HASH_LENGTH]>::uninit();
    unsafe {
        sol_sha256(
            data as *const _ as *const u8,
            data.len() as u64,
            hash_result.as_mut_ptr() as *mut u8,
        );
        hash_result.assume_init()
    }
}

pub fn hash_into(data: &[&[u8]], out: &mut [u8; HASH_LENGTH]) {
    let mut hasher = Sha256::new();
    for item in data {
        hasher.update(item.as_ref());
    }
    hasher.finalize_into(out.into());
}

#[cfg(target_os = "solana")]
pub fn hash_into(data: &[&[u8]], out: &mut [u8; HASH_LENGTH]) {
    unsafe {
        sol_sha256(
            data as *const _ as *const u8,
            data.len() as u64,
            out.as_mut_ptr() as *mut u8,
        );
    }
}


#[cfg(test)]
mod tests {
    use crate::hash;

    #[test]
    fn test_hash() {
        let h = hash(&[0x48, 0xd8, 0x9d, 0x82, 0x9b, 0x66, 0x04, 0x5f, 0xcc, 0x29, 0x9b, 0x96, 0x67, 0x8e, 0xa6, 0xf5, 0xa6, 0xcd, 0x0c, 0xb4, 0xa2, 0xaf, 0xb2, 0x70, 0xf6, 0x02, 0x7d, 0xe4, 0xdd, 0x7d, 0x3f, 0xb4]);
        assert_eq!(h, [0xdb, 0x46, 0x53, 0xfa, 0x44, 0x0e, 0x62, 0x1b, 0xe5, 0x4e, 0xf7, 0xd9, 0xc0, 0xe0, 0x64, 0xba, 0x05, 0xb7, 0x85, 0xdb, 0x4f, 0x5a, 0x3d, 0xa6, 0xa8, 0x21, 0xd7, 0x2c, 0x9d, 0x88, 0xd1, 0xea]);
    }
}

#[cfg(feature = "hmac")]
pub mod hmac;
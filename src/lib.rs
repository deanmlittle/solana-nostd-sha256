use std::mem::MaybeUninit;

#[cfg(not(target_os = "solana"))]
use sha2::{Digest, Sha256};

pub const HASH_LENGTH: usize = 32;

#[cfg(target_os = "solana")]
extern "C" {
    fn sol_sha256(vals: *const u8, val_len: u64, hash_result: *mut u8) -> u64;
}

pub fn hash<T: AsRef<[u8]>>(data: T) -> [u8;HASH_LENGTH] {
    hashv::<T>(&[data])
}

pub fn hashv<T: AsRef<[u8]>>(data: &[T]) -> [u8; HASH_LENGTH] {
    let mut out = MaybeUninit::<[u8; HASH_LENGTH]>::uninit();
    unsafe {
        hash_into(data, &mut out.assume_init_mut());
        out.assume_init()
    }
}

#[cfg(not(target_os = "solana"))]
pub fn hash_into<T: AsRef<[u8]>>(data: &[T], out: &mut [u8; HASH_LENGTH]) {
    let mut hasher = Sha256::new();
    for item in data {
        hasher.update(item.as_ref());
    }
    hasher.finalize_into(out.into());
}

#[cfg(target_os = "solana")]
pub fn hash_into<T: AsRef<[u8]>>(data: &[T], out: &mut [u8; HASH_LENGTH]) {
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
    use crate::*;

    #[test]
    fn test_hash() {
        let h = hash("test");
        let h2 = hashv(&["test"]);
        assert_eq!(h, h2);
        assert_eq!(h2, [0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65, 0x9a, 0x2f, 0xea, 0xa0, 0xc5, 0x5a, 0xd0, 0x15, 0xa3, 0xbf, 0x4f, 0x1b, 0x2b, 0x0b, 0x82, 0x2c, 0xd1, 0x5d, 0x6c, 0x15, 0xb0, 0xf0, 0x0a, 0x08]);
    }
}
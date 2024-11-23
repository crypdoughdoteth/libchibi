#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::hash::Hasher;
use libc::{c_uint, c_void, free, malloc, ptrdiff_t};

extern "C" {
    fn chibihash64__load64le(p: *const c_uint) -> u64;
    fn chibihash64(keyIn: *const c_void, len: ptrdiff_t, seed: u64) -> u64;
}

/// ```rust
/// use libchibi::Chibihash;
/// use std::hash::Hasher;
///
/// // Hasher trait interface
/// let mut chibi = Chibihash::new(42);
/// chibi.write(b"Vyper");
/// chibi.write(b"GM");
/// let hash = chibi.finish();
/// println!("{hash:?}");
///
///  // Basic interface
/// let chibi = Chibihash::new(42);
/// let hash = chibi.hash(b"GM");
/// println!("{hash:?}");
/// ```
pub struct Chibihash {
    seed: u64,
    buffer: Vec<u8>,
}

impl Hasher for Chibihash {
    fn finish(&self) -> u64 {
        self.hash(&self.buffer).0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }
}

impl Chibihash {
    pub fn new(seed: u32) -> Self {
        unsafe {
            let seed_ptr = malloc(size_of::<c_uint>()) as *mut c_uint;
            *seed_ptr = seed;
            let seed = chibihash64__load64le(seed_ptr as *const c_uint);
            free(seed_ptr as *mut c_void);
            Chibihash {
                seed,
                buffer: Vec::with_capacity(1024),
            }
        }
    }

    pub fn hash(&self, key: &[u8]) -> Hash {
        unsafe {
            let res = chibihash64(
                key.as_ptr() as *const c_void,
                core::mem::size_of_val(key) as ptrdiff_t,
                self.seed,
            );
            Hash(res)
        }
    }
}

#[derive(Debug)]
pub struct Hash(u64);

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn basic() {
        let chibi = Chibihash::new(0);
        let key_in = [0u8; 128];
        let res = chibi.hash(&key_in);
        assert_eq!(res.0, 1977729916931055241);
    }
}

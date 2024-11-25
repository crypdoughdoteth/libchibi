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
#[derive(Debug, Clone)]
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
    /// Chibihash is non-CRHF that is initialized with a seed value. By default the buffer capacity is 1024.
    pub fn new(seed: u32) -> Self {
        Self::with_capacity(seed, 1024)
    }

    /// Sets the internal buffer's capacity to `capacity`. This is primarily good for being conservative with
    /// the size of your allocations in constrained environments.
     pub fn with_capacity(seed: u32, capacity: usize) -> Self {
       let mut seed0 : c_uint = seed as c_uint; 
       let seed = unsafe {
            chibihash64__load64le((&seed0) as *const c_uint)
        };
        Chibihash {
            seed,
            buffer: Vec::with_capacity(capacity),
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Hash(pub u64);

impl Hash {
    pub fn to_be_bytes(&self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    pub fn to_le_bytes(&self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    pub fn to_ne_bytes(&self) -> [u8; 8] {
        self.0.to_ne_bytes()
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

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

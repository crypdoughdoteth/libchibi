use libc::{c_uint, c_void, ptrdiff_t};
use std::hash::Hasher;

extern "C" {
    fn chibihash64__load64le(p: *const c_uint) -> u64;
    fn chibihash64(keyIn: *const c_void, len: ptrdiff_t, seed: u64) -> u64;
}

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
        let seed_ptr = Box::into_raw(Box::new(seed));
        unsafe {
            let seed = chibihash64__load64le(seed_ptr as *const c_uint);
            let _ = Box::from_raw(seed_ptr);
            Chibihash {
                seed,
                buffer: Vec::with_capacity(1024),
            }
        }
    }

    pub fn hash(&self, key: &[u8]) -> Hash {
        println!("{}", std::mem::size_of_val(&key));
        unsafe {
            let res = chibihash64(
                key.as_ptr() as *const c_void,
                std::mem::size_of_val(&key) as ptrdiff_t,
                self.seed,
            );
            Hash(res)
        }
    }
}

#[repr(C)]
pub struct Hash(u64);

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn basic() {
        unsafe {
            let seed_ptr = Box::into_raw(Box::new(0));
            let seed = chibihash64__load64le(seed_ptr as *const c_uint);
            let key_in = [0i8; 128];
            let res = chibihash64(
                key_in.as_ptr() as *const c_void,
                std::mem::size_of_val(&key_in) as ptrdiff_t,
                seed,
            );
            assert_eq!(0, *seed_ptr);
            assert_eq!(res, 1977729916931055241);
            let _ = Box::from_raw(seed_ptr);
        };
    }
}

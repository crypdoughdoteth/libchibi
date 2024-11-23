# Libchibi 

For simplicity and ease of use, this library statically links the original implementation of Chibihash64. The current goal of this library is to provide safe bindings to the implementation 
and integrate with the Rust standard library's `Hasher` trait. 

# Usage

```rust 
use std::hash::Hasher;

let mut chibi = Chibihash::new(42);
chibi.write(b"Vyper");
chibi.write(b"GM");
let hash = chibi.finish();
println!("{hash:?}");

 // Basic interface
let chibi = Chibihash::new(42);
let hash = chibi.hash(b"GM");
println!("{hash:?}");
```

# When Not To Use

As stated in the README of the (original repository)[https://github.com/N-R-K/ChibiHash],

>The introduction should make it clear on why you'd want to use this. Here are some reasons to avoid using this:
>
> - For cryptographic purposes.
>
> - For protecting against collision attacks (SipHash is the recommended one for this purpose).
>
> - When you need very strong probability against collisions: ChibiHash does very minimal amount of mixing compared to other hashes (e.g xxhash64). And so chances of collision should in theory be higher.""

# Origin of Chibihash64

The implementation for the hash function was written by N-R-K and can be found (here)[https://github.com/N-R-K/ChibiHash]

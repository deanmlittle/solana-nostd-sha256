# Solana NoStd Sha256

Just like Sha256, only 20cus cheaper than the std implementation. 

Improvements:

- Adds `hash_ref` which takes in any type that implements `<AsRef<[u8]>>`
- No `Hash` struct. Returns `[u8;32]` directly.
- Makes use of MaybeUninit to skip zero allocations
- Adds `hash_into` to let you hash directly into a mutable buffer.
# idalloc

[![Documentation](https://docs.rs/idalloc/badge.svg)](https://docs.rs/idalloc)
[![Crates](https://img.shields.io/crates/v/idalloc.svg)](https://crates.io/crates/idalloc)
[![Actions Status](https://github.com/udoprog/idalloc/workflows/Rust/badge.svg)](https://github.com/udoprog/idalloc/actions)

General purpose algorithms to generate unique identifiers.

# Examples

```rust
let mut alloc = idalloc::Slab::<u32>::new();
assert_eq!(0u32, alloc.next());
assert_eq!(1u32, alloc.next());
alloc.free(0u32);
```
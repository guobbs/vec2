# vec2::Vec2
A 2D vector-like data structure that allocates memory in chunks.

Using the standard library’s `std::vec::Vec`, when there are many elements it allocates a large contiguous block of memory at once, which can easily cause fragmentation and make memory allocation difficult. `vec2::Vec2` essentially wraps a `Vec<Vec<T>>` and makes it behave like a `Vec<T>` in usage.

# Examples

```rust
use std::num::NonZeroU32;
let mut vec2 = super::Vec2::new(NonZeroU32::new(3).unwrap());
vec2.push(1);
vec2.push(2);
vec2.push(3);
vec2.push(4);
assert_eq!(vec2.len(), 4);
assert_eq!(vec2.pop(), Some(4));
assert_eq!(vec2.pop(), Some(3));
assert_eq!(vec2.len(), 2);
```


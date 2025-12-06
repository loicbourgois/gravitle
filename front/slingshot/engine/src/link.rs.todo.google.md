```rust
// The `Link` struct represents connections between entities (e.g., nodes in a graph) using their indices.
//
// For compatibility with WebGPU and to ensure a consistent memory layout across different
// compilation targets (e.g., `wasm32` for WebAssembly, `x86_64` for native desktop),
// it's crucial to use fixed-size integer types rather than `usize`.
//
// `usize` is pointer-sized, meaning its size depends on the target architecture:
// - On `wasm32` targets (common for WebGPU via WASM), `usize` is 4 bytes (32-bit).
// - On `x86_64` targets (common for native applications), `usize` is 8 bytes (64-bit).
//
// Using `u32` guarantees that each index `a` and `b` will always be 4 bytes, making
// the `Link` struct consistently 8 bytes (2 * 4 bytes) in size, which is ideal
// for passing to GPU shaders as uniform or storage buffer data.
#[derive(Clone)]
pub struct Link {
    pub a: u32, // Index of the first entity, using u32 for fixed size and GPU compatibility
    pub b: u32, // Index of the second entity, using u32 for fixed size and GPU compatibility
}
```
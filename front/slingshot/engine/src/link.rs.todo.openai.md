Good question — this kind of `TODO` often appears when WebGPU or low‑level GPU interop (through WGSL, WebAssembly, etc.) is involved.

Let’s unpack it carefully.

---

### 🧩 The situation
You have this struct:

```rust
#[derive(Clone)]
pub struct Link {
    pub a: usize,
    pub b: usize,
}
```

and a comment:

```rust
// TODO: how to guarantee size of this struct when passing to webgpu via wasm ?
// what's the size of usize ?
```

---

### 📏 Step 1. What is the size of `usize`?

`usize` (and `isize`) is **platform-dependent**:

| Platform | Bits | `std::mem::size_of::<usize>()` |
|-----------|-------|-------------------------------|
| 32-bit    | 32    | 4 bytes                       |
| 64-bit    | 64    | 8 bytes                       |

In **WASM (wasm32)** targets (the target used for frontend WebGPU code),  
`usize` is always **32 bits**, i.e. 4 bytes.

But if you build the same code for your native machine (e.g. x86_64), `usize` is **64 bits (8 bytes)**. So the struct size will **differ between targets**:
- 8 bytes total on wasm32 (2 × 4)
- 16 bytes total on x86_64 (2 × 8)

That difference makes it unsafe to assume a fixed memory layout for GPU uniform/storage buffer transfers.

---

### 💡 Step 2. Why this matters
When you send data to the GPU (via WebGPU + WGSL), you need predictable layout & alignment.

WGSL types like `u32` and `i32` correspond to 4‑byte scalars.  
There is **no WGSL `usize` equivalent** — sizes must be *fixed*.

---

### ✅ Step 3. The proper fix

Replace `usize` with a fixed-width integer (e.g., `u32` or `u64`), depending on what your indices represent.

If your indices won’t exceed 4 billion (which is true for most meshes/graphs), `u32` is perfect.

```rust
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Link {
    pub a: u32,
    pub b: u32,
}
```

**Notes:**
- `#[repr(C)]` ensures predictable field order and padding (C ABI compatible).
- `#[derive(Copy)]` is generally useful for small data blobs sent to GPU buffers.
- `u32` ensures the same size on all platforms (4 bytes each → struct size = 8 bytes).

If you need to support very large indices and know your WGSL uses `u64` (rare — not all GPUs support 64-bit ints well), you could use `u64`, but `u32` is the safe & standard choice.

---

### 🧮 Optional runtime guard
If you want to be defensive:

```rust
#[cfg(target_arch = "wasm32")]
const _: () = assert!(std::mem::size_of::<Link>() == 8);
```

That ensures you don’t accidentally change the field types later.

---

### 🔧 Summary

| Concern | Resolution |
|----------|-------------|
| “What’s the size of `usize`?” | 4 bytes on wasm32, 8 bytes on x86_64 |
| “How to guarantee struct size?” | Use `#[repr(C)]` and fixed-width integers (`u32`, `f32`, etc.) instead of `usize` |
| Durable fix | ```rust #[repr(C)] struct Link { a: u32, b: u32 } ``` |

---

### ✅ Final Answer

Replace the `TODO` code with:

```rust
// Fixed-width integer version for predictable GPU interop.
// usize is 32 bits on wasm32, 64 bits on x86_64 — avoid it here.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Link {
    pub a: u32,
    pub b: u32,
}
```

That guarantees consistent size, layout, and alignment when passing through WebGPU via wasm.
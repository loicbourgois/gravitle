fn main() {
    println!("cargo:rerun-if-changed=src/compute-shader.glsl");
}

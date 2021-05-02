import("../pkg/index.js").catch(console.error).then(function(wasm) {
  import("../pkg/index_bg").catch(console.error).then(function(bg) {
    const memory = bg.memory;
    const universe = wasm.Universe.new()
    const particles_data = new Float32Array(
      memory.buffer,
      universe.get_particles_ptr(),
      universe.get_particles_buffer_size());
    console.log("p0", get_particle(particles_data, 0))
    data = universe.get_client_data()
    console.log(data.step())
    universe.tick()
    data = universe.get_client_data()
    console.log(data.step())
    console.log(wasm)
  });
});
const get_particle = (particles_data, index) => {
  const i = index * 2;
  return {
    x: particles_data[i + 0],
    y: particles_data[i + 1]
  }
}

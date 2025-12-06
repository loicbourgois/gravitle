The `TODO` comment is asking to draw links using a "segment like primitive" rather than triangles, and to make them "thicker" and "rounded at the edge".

The current implementation in `ViewWebGPU.prototype.add_new_step` for `pipeline_2` (which draws links) uses `topology: "line-list"`. While this is a "segment like primitive", it only draws 1-pixel thin lines and doesn't support rounded caps directly. To achieve "thicker" and "rounded at the edge" lines, we need to draw them as geometric shapes (quads) and use the fragment shader for precise shaping of the rounded caps and anti-aliasing.

Here's the plan:

1.  **Change Topology**: Modify `pipeline_2` from `line-list` to `triangle-strip` to draw quads.
2.  **Update `pass.draw`**: In `ViewWebGPU.prototype.render_2`, change the number of vertices drawn for links from 2 to 4 (for a triangle strip forming a quad).
3.  **Add `line_width` Uniform**:
    *   Extend the `Uniforms` struct in `webgpu/code.wgsl` to include `line_width`.
    *   Update `ViewWebGPU.prototype.setup_uniform` to allocate space for this new uniform.
    *   Update `ViewWebGPU.prototype.render_2` to write the `line_width` value to the uniform buffer.
4.  **Modify `vs_2` (Vertex Shader)**:
    *   Generate four vertices for each link, forming a quad that represents the thick line segment.
    *   To allow for rounded caps in the fragment shader, extend the length of this quad by `half_width` at each end.
    *   Pass the original (un-extended) endpoints (`p1_world`, `p2_world`) and `half_width` to the fragment shader using a new `VSOutputLinks` struct.
5.  **Modify `fs_2` (Fragment Shader)**:
    *   Using the interpolated `p1_world`, `p2_world`, and `half_width` from the vertex shader, calculate the distance of the current fragment from the actual line segment (including its endpoints).
    *   Discard fragments that are outside the desired rounded line shape (a capsule shape).
    *   Optionally, apply anti-aliasing by fading the alpha value near the edges using `smoothstep`.
6.  **Remove/Update TODO Comments**: Remove the `TODO` from `view_webgpu.js` and update the one in `code.wgsl`.

These changes will convert the links from simple 1-pixel lines to visually thicker lines with rounded end caps.

```diff
--- a/front/slingshot/view_webgpu.js
+++ b/front/slingshot/view_webgpu.js
@@ -62,7 +62,8 @@
 
 ViewWebGPU.prototype.setup_uniform = function (binding) {
 	const uniformBufferSize =
-    	1 * 4; // 1 32bit floats
+    	// 1 * 4; // Old: 1 32bit float (zoom)
+    	2 * 4; // New: 2 32bit floats (zoom, line_width)
 	this.buffer_uniform = this.device.createBuffer({
 		size: uniformBufferSize,
 		usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
@@ -165,7 +166,7 @@
 		label: "pipeline_2",
 		vertex_entryPoint: "vs_2",
 		fragment_entryPoint: "fs_2",
-		topology: "line-list",
+		topology: "triangle-strip", // Changed from "line-list" to support thicker, rounded lines
 	})
 	this.renderPassDescriptor = {
 		label: "renderPass",
@@ -191,9 +192,11 @@
 
 ViewWebGPU.prototype.render_2 = function (worlds, gravitle, memory) {
 	const canvas_texture = this.context.getCurrentTexture();
+	const LINK_LINE_WIDTH = 0.005; // World-space width of links, needs tuning based on scene scale
+
 	this.uniformValues.set([
 		this.zoom,
-	]);
+		LINK_LINE_WIDTH, // New uniform for link line width
+	]);
 	this.device.queue.writeBuffer(this.buffer_uniform, 0, this.uniformValues);
 	this.renderPassDescriptor.colorAttachments[0].view =
 		canvas_texture.createView();
@@ -269,7 +272,7 @@
 			const linksStep = this.steps[2];
 			pass.setPipeline(linksStep.pipeline);
 			pass.setBindGroup(0, linksStep.bindGroup);
-			pass.draw(
-				2, // 2 vertices per line segment (a link connects two points)
+			pass.draw( // Changed from 2 vertices (line-list) to 4 (triangle-strip for quad)
+				4, // 4 vertices for a quad (triangle-strip)
 				worlds.reduce((sum, world) => sum + world.links_count(), 0), // Total number of links as instances
 			);
 
@@ -285,8 +288,6 @@
 	this.canvas.height = size;
 };
 
-
 export { ViewWebGPU };
-
-// TODO: draw links
-// links should be drawn using a segment like primitive, not triangles
```

```diff
--- a/front/slingshot/webgpu/code.wgsl
+++ b/front/slingshot/webgpu/code.wgsl
@@ -37,6 +37,7 @@
 
 struct Uniforms {
     zoom: f32,
+    line_width: f32, // Added for drawing thicker links
 }
 
 @group(0) @binding(0) var<storage, read> cells: array<Cell>;
@@ -45,6 +46,15 @@
 @group(0) @binding(3) var<uniform> uniforms: Uniforms;
 @group(0) @binding(4) var<storage, read> links: array<Link>;
 
+struct VSOutputLinks {
+  @builtin(position) position: vec4f,
+  @location(0) color: vec4f,
+  @location(1) p1_world: vec2f,      // Original p1 in world space
+  @location(2) p2_world: vec2f,      // Original p2 in world space
+  @location(3) half_width: f32,      // Half width of the line in world space
+  // @location(4) instance_idx: u32, // Can be useful for debugging, but keeping within 4 locations for simplicity.
+}
+
 @vertex fn vs_0(
   @builtin(vertex_index) vertexIndex : u32,
   @builtin(instance_index) instanceIndex: u32,
@@ -83,43 +93,89 @@
   return vsOut.color;
 }
 
-// TODO: make the links thicker, rounded at the edge
+// This shader now handles drawing thicker, rounded links.
+// It achieves this by generating quad geometry in the vertex shader,
+// and then using the fragment shader to clip and anti-alias the rounded ends.
 @vertex fn vs_2(
   @builtin(vertex_index) vertexIndex : u32,
   @builtin(instance_index) instanceIndex: u32,
-) -> VSOutput {
+) -> VSOutputLinks { // Changed return type to pass more data to fragment shader
   let link = links[instanceIndex];
   let cell_a = cells[link.a];
   let cell_b = cells[link.b];
 
-  var position_to_render: vec2f;
-  if (vertexIndex == 0u) {
-    // First vertex of the line segment uses cell_a's average position
-    position_to_render = cell_a.ap;
-  } else { // vertexIndex == 1u
-    // Second vertex of the line segment uses cell_b's average position
-    position_to_render = cell_b.ap;
+  let p1_world_actual = cell_a.ap;
+  let p2_world_actual = cell_b.ap;
+
+  // Handle zero-length lines to avoid division by zero in normalize
+  if (distance(p1_world_actual, p2_world_actual) < 0.00001) {
+      var vsOut: VSOutputLinks;
+      vsOut.position = vec4f(0.0, 0.0, 0.0, 0.0); // Discard or make invisible
+      vsOut.color = vec4f(0.0, 0.0, 0.0, 0.0); // Transparent
+      vsOut.p1_world = p1_world_actual;
+      vsOut.p2_world = p2_world_actual;
+      vsOut.half_width = 0.0;
+      return vsOut;
   }
-  let center = vec2f( 0.0,  0.0 ); // Keeping consistent with other shaders
-  var vsOut: VSOutput;
+
+  let line_dir = normalize(p2_world_actual - p1_world_actual);
+  let perp_dir = vec2f(-line_dir.y, line_dir.x); // Perpendicular vector
+
+  let half_width = uniforms.line_width * 0.5;
+
+  // Extend the line segment by half_width at each end to allow for rounded caps.
+  // This creates a larger quad that covers the full "capsule" shape.
+  let p1_extended = p1_world_actual - line_dir * half_width;
+  let p2_extended = p2_world_actual + line_dir * half_width;
+
+  var current_position_world: vec2f;
+
+  // Vertices for a quad forming an extended line segment, suitable for triangle-strip.
+  // Order:
+  // 0: P1_extended - perp_dir * half_width
+  // 1: P1_extended + perp_dir * half_width
+  // 2: P2_extended - perp_dir * half_width
+  // 3: P2_extended + perp_dir * half_width
+  switch vertexIndex {
+    case 0u: { // bottom-left of P1_extended quad
+      current_position_world = p1_extended - perp_dir * half_width;
+    }
+    case 1u: { // top-left of P1_extended quad
+      current_position_world = p1_extended + perp_dir * half_width;
+    }
+    case 2u: { // bottom-right of P2_extended quad
+      current_position_world = p2_extended - perp_dir * half_width;
+    }
+    case 3u: { // top-right of P2_extended quad
+      current_position_world = p2_extended + perp_dir * half_width;
+    }
+    default: {
+        current_position_world = p1_extended; // Fallback, should not be reached with pass.draw(4,...)
+    }
+  }
+
+  let center = vec2f( 0.0,  0.0 ); // Keeping consistent with other shaders for view transform
+  var vsOut: VSOutputLinks; // Changed type
   vsOut.position = vec4f(
-    (position_to_render - center) * uniforms.zoom,
+    (current_position_world - center) * uniforms.zoom,
     0.0, 1.0
   );
   vsOut.color = vec4f(
     1.0, 1.0, 0.0, 1.0 // Yellow color for links
   );
+  vsOut.p1_world = p1_world_actual; // Pass original (un-extended) endpoints to fragment shader
+  vsOut.p2_world = p2_world_actual;
+  vsOut.half_width = half_width;
   return vsOut;
 }
-@fragment fn fs_2(vsOut: VSOutput) -> @location(0) vec4f {
-  return vsOut.color;
+@fragment fn fs_2(vsOut: VSOutputLinks) -> @location(0) vec4f { // Changed input type
+  let frag_pos_world = (vsOut.position.xy / uniforms.zoom) + vec2f(0.0, 0.0); // Convert clip space fragment position to world space
+
+  let p1 = vsOut.p1_world; // Original start point in world space
+  let p2 = vsOut.p2_world; // Original end point in world space
+  let half_width = vsOut.half_width;
+
+  let line_vec = p2 - p1;
+  let line_len_sq = dot(line_vec, line_vec);
+
+  var t: f32; // Parameter along the line segment (0 at p1, 1 at p2)
+  if (line_len_sq == 0.0) {
+    t = 0.0; // Line is effectively a point, treat as at p1
+  } else {
+    // Project fragment position onto the infinite line defined by p1 and p2
+    t = dot(frag_pos_world - p1, line_vec) / line_len_sq;
+  }
+
+  // Calculate distance from fragment to the closest point on the line *segment*
+  var dist_to_segment_center: f32;
+  if (t < 0.0) {
+      // Fragment is before p1, closest point on segment is p1
+      dist_to_segment_center = distance(frag_pos_world, p1);
+  } else if (t > 1.0) {
+      // Fragment is after p2, closest point on segment is p2
+      dist_to_segment_center = distance(frag_pos_world, p2);
+  } else {
+      // Fragment is within the segment (or perpendicular to it), closest point is on the line itself
+      let closest_point_on_line = p1 + t * line_vec;
+      dist_to_segment_center = distance(frag_pos_world, closest_point_on_line);
+  }
+
+  // Discard fragments that are outside the desired rounded line shape (capsule)
+  if (dist_to_segment_center > half_width) {
+      discard;
+  }
+
+  // Optional: Add anti-aliasing for smoother edges by fading alpha.
+  // `feather_width` defines the region over which the fade happens in world coordinates.
+  // This value may need tuning based on the overall scale and zoom levels.
+  let feather_width = 0.0005; 
+  // Use smoothstep to create a smooth fade from opaque (1.0) to transparent (0.0)
+  // as the distance approaches half_width from (half_width - feather_width).
+  let alpha = 1.0 - smoothstep(half_width - feather_width, half_width, dist_to_segment_center);
+
+  return vec4f(vsOut.color.rgb, vsOut.color.a * alpha);
 }
```
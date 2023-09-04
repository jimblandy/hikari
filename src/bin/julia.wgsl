struct Vertex {
    @builtin(position) fragment: vec4<f32>,
    @location(0) plane: vec2<f32>,
}

@vertex
fn julia_vertex(@builtin(vertex_index) index: u32) -> Vertex {
   // Map indices 0..3 to the four corners of the unit square.
   let plane = vec2(select(-1.0, 1.0, index < 2u),
                    select(-1.0, 1.0, (index & 1u) != 0u));
   let fragment = vec4(plane.x, plane.y, 0.0, 1.0);
   return Vertex(fragment, plane);
}

@fragment
fn julia_fragment(vertex: Vertex) -> @location(0) vec4<f32> {
   return vec4((vertex.plane.x + 1.0) / 2.0, 0.5, 0.5, 1.0);
}

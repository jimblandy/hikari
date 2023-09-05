struct Inputs {
    // Homogeneous transform from clip space to our complex plane.
    clip_to_complex: mat3x3<f32>,

    // The mouse's position on the complex plane.
    mouse: vec3<f32>,
};

@group(0) @binding(0) var<uniform> inputs: Inputs;

struct Vertex {
    @builtin(position) fragment: vec4<f32>,
    @location(0) complex: vec2<f32>,
}

@vertex
fn julia_vertex(@builtin(vertex_index) index: u32) -> Vertex {
   // Map indices 0..3 to the four corners of the unit square.
   let corner = vec3(select(-1.0, 1.0, index < 2u),
                     select(-1.0, 1.0, (index & 1u) != 0u),
                     1.0);
   let fragment = vec4(corner.x, corner.y, 0.0, 1.0);
   let complex = (inputs.clip_to_complex * corner).xy;
   return Vertex(fragment, complex);
}

@fragment
fn julia_fragment(vertex: Vertex) -> @location(0) vec4<f32> {
   let d = length(inputs.mouse.xy - vertex.complex);
   let c = select(0.0, 1.0, d < 0.1);
   return vec4(c, c, c, 1.0);
}

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
   let c = inputs.mouse.xy;
   var z = vertex.complex;
   var i: i32;
   for (i = 0; i < 20; i++) {
       let r = z.x * z.x - z.y * z.y;
       z.y = 2.0 * z.x * z.y;
       z.x = r;
       z += c;
       if z.x * z.x + z.y * z.y > 4.0 {
          break;
       }
   }

   if i == 20 {
       return vec4(1.0, 1.0, 1.0, 1.0);
   } else {
       let b = f32(i) / 20.0;
       return vec4(b, b, b, 1.0);
   }
}

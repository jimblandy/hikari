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

alias Complex = vec2<f32>;
alias Color = vec4<f32>;
const BLACK: Color = vec2(0.0, 1.0).xxxy;
const WHITE: Color = vec4(1.0);

const LIMIT: i32 = 80;

fn mult(a: Complex, b: Complex) -> Complex {
   return Complex(
       a.x * b.x - a.y * b.y,
       2.0 * a.x * b.y
   );
}

fn iterate(z_: Complex, c: Complex) -> i32 {
   var z = z_;
   var i: i32;
   for (i = 0; i < LIMIT; i++) {
       z = mult(z, z) + c;
       if dot(z, z) > 4.0 {
          break;
       }
   }

   return i;
}

fn iterations_to_color(iterations: i32) -> Color {
    return mix(BLACK, WHITE, min(1.0, f32(iterations) / f32(LIMIT)));
}

@fragment
fn julia_fragment(vertex: Vertex) -> @location(0) vec4<f32> {
   let c = inputs.mouse.xy;
   var z = vertex.complex;

   // Compute Julia set color.
   let j = iterations_to_color(iterate(vertex.complex, inputs.mouse.xy));

   // Compute Mandelbrot set color.
   //let m = iterations_to_color(iterate(Complex(0.0), vertex.complex));
   let m = iterations_to_color(0);

   return mix(j, m, 0.1);
}

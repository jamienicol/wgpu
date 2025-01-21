// Input to the shader. The length of the array is determined by what buffer is bound.
//
// Out of bounds accesses 
@group(0) @binding(0)
var<storage, read> input: array<f32>;
// Output of the shader.  
@group(0) @binding(1)
var<storage, read_write> output: array<f32>;

@compute @workgroup_size(1)
fn main() {
  output[0] = clamp(input[0], input[1], input[2]);
}

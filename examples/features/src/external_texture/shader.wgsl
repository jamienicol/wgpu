struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coord: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
}

@vertex
fn vert_main(vertex: VertexInput) -> VertexOutput {
    var outval: VertexOutput;
    outval.position = vec4<f32>(vertex.position.x, vertex.position.y, 0.0, 1.0);
    outval.tex_coord = vertex.tex_coord;
    return outval;
}

struct FragmentInput {
    @location(0) tex_coord: vec2<f32>,
}

@group(0) @binding(0)
var texture: texture_external;
@group(0) @binding(1)
var samp: sampler;

@fragment
fn frag_main(fragment: FragmentInput) -> @location(0) vec4<f32> {
    return textureSampleBaseClampToEdge(
        texture,
        samp,
        fragment.tex_coord,
    );
    // return textureLoad(texture, vec2u(0, 0));
}

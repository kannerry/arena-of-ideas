#import bevy_sprite::{
    mesh2d_view_bindings::globals,
    mesh2d_view_bindings::view,
    mesh2d_vertex_output::VertexOutput,
}

struct CurveMaterial {
    color: vec4<f32>,
    aa: f32,
    alpha: f32,
};

@group(2) @binding(0)
var<uniform> material: CurveMaterial;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let v = smoothstep(1.0, 1.0 - material.aa, abs(in.uv.y));
    var color = material.color;
    color.a *= v * material.alpha;
    return color;
}
#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh2d;

#import bevy_sprite::mesh2d_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) intensity: f32,

    @location(2) i_position_scale: vec4<f32>,
    @location(3) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(in: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let position = in.position * in.i_position_scale.w + in.i_position_scale.xyz;
    out.clip_position = mesh2d_position_local_to_clip(
        mesh.model,
        vec4<f32>(position, 1.0),
    );
    out.color = in.intensity * in.i_color;
    return out;
}

struct FragmentInput {
    @location(0) color: vec4<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    return in.color;
}

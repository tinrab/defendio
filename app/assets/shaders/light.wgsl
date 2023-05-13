//#import bevy_pbr::mesh_types
//#import bevy_pbr::mesh_view_bindings
#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh2d;
//var<uniform> mesh: Mesh;

// NOTE: Bindings must come before functions that use them!
//#import bevy_pbr::mesh_functions
#import bevy_sprite::mesh2d_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
//    @location(1) color: u32,

    @location(2) i_position_scale: vec4<f32>,
    @location(3) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let position = vertex.position * vertex.i_position_scale.w + vertex.i_position_scale.xyz;
    var out: VertexOutput;
    out.clip_position = mesh2d_position_local_to_clip(mesh.model, vec4<f32>(position, 1.0));
    out.color = vertex.color * vertex.i_color;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = vec4(1.0,1.0,1.0,1.0);
    return color;
}

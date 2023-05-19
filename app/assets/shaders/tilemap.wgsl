#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_bindings

// NOTE: Bindings must come before functions that use them!
#import bevy_sprite::mesh2d_functions

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

struct Vertex {
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
}

@group(1) @binding(0)
var color_texture: texture_2d<f32>;
@group(1) @binding(1)
var color_sampler: sampler;
@group(1) @binding(2)
var lighting_texture: texture_2d<f32>;
@group(1) @binding(3)
var lighting_sampler: sampler;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

#ifdef VERTEX_POSITIONS
    out.world_position = mesh2d_position_local_to_world(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.clip_position = mesh2d_position_world_to_clip(out.world_position);
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif

    return out;
}

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = textureSample(color_texture, color_sampler, in.uv);
    #ifdef VERTEX_COLORS
        output_color = output_color * in.color;
    #endif

//    output_color = vec4(in.clip_uv, 0.0, 1.0);
//    output_color =  textureSample(lighting_texture, lighting_sampler, in.clip_uv);

    var clip_uv = (in.clip_position.xy - view.viewport.xy) / view.viewport.zw;
    var lighting_color = textureSample(lighting_texture, lighting_sampler, clip_uv);
    output_color *= vec4(lighting_color.xyz, 1.0);

    #ifdef TONEMAP_IN_SHADER
        output_color = tone_mapping(output_color);
    #endif
    return output_color;
}

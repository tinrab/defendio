#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

@group(1) @binding(0)
var color_texture: texture_2d<f32>;
@group(1) @binding(1)
var color_sampler: sampler;
@group(1) @binding(2)
var lighting_texture: texture_2d<f32>;
@group(1) @binding(3)
var lighting_sampler: sampler;

struct FragmentInput {
    #import bevy_sprite::mesh2d_vertex_output
};

@fragment
fn fragment(
    @builtin(position) clip_position: vec4<f32>,
    in: FragmentInput
) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = textureSample(color_texture, color_sampler, in.uv);
    #ifdef VERTEX_COLORS
        output_color = output_color * in.color;
    #endif

    var clip_uv = (clip_position.xy - view.viewport.xy) / view.viewport.zw;
    var lighting_color = textureSample(lighting_texture, lighting_sampler, clip_uv);
    output_color *= vec4(lighting_color.xyz, 1.0);

    #ifdef TONEMAP_IN_SHADER
        output_color = tone_mapping(output_color);
    #endif
    return output_color;
}

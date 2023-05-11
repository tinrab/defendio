#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

struct TilemapMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: TilemapMaterial;
@group(1) @binding(1)
var color_texture: texture_2d<f32>;
@group(1) @binding(2)
var color_sampler: sampler;

struct FragmentInput {
    #import bevy_sprite::mesh2d_vertex_output
};

@fragment
fn fragment(
    in: FragmentInput
) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = textureSample(color_texture, color_sampler, in.uv);
    #ifdef VERTEX_COLORS
        output_color = output_color * in.color;
    #endif
    #ifdef TONEMAP_IN_SHADER
        output_color = tone_mapping(output_color);
    #endif
    return output_color;
}

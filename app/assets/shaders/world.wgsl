#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_bindings
#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif
// NOTE: Bindings must come before functions that use them!
#import bevy_sprite::mesh2d_functions

#import world_material::bindings

struct Vertex {
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
}

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
    var output_color: vec4<f32> = material.base_color;
#ifdef VERTEX_COLORS
    output_color = output_color * in.color;
#endif

#ifdef VERTEX_UVS
    if ((material.flags & WORLD_MATERIAL_FLAGS_BASE_COLOR_TEXTURE_BIT) != 0u) {
        output_color = output_color * textureSample(base_color_texture, base_color_sampler, in.uv);
    }

//    if ((material.flags & WORLD_MATERIAL_FLAGS_NORMAL_TEXTURE_BIT) != 0u) {
//        let normal_value = textureSample(normal_texture, normal_sampler, in.uv);
//
//        for (var i: u32 = 0u; i < arrayLength(&lighting.lights); i = i + 1u) {
//            let light = lighting.lights[i];
//            let world_to_light = light.position - in.world_position.xyz;
//            let dist = length(world_to_light);
//            let dir = normalize(world_to_light);
//
//            let radiance = light.color * (1.0 / pow(dist, 2.0));
//            let strength = max(dot(normal_value.xyz, dir), 0.0);
//
//            output_color += radiance * strength;
//        }
//    }
#endif

    if ((material.flags & WORLD_MATERIAL_FLAGS_EMISSIVE_TEXTURE_BIT) != 0u) {
#ifdef VERTEX_UVS
        output_color += material.emissive * textureSample(emissive_texture, emissive_sampler, in.uv);
#endif
    } else {
        output_color += material.emissive;
    }

#ifdef TONEMAP_IN_SHADER
    output_color = tone_mapping(output_color);
#endif
    return output_color;
}

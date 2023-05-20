#define_import_path world_material::bindings

#import world_material::types

@group(1) @binding(0)
var<uniform> material: WorldMaterial;

@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;
@group(1) @binding(3)
var normal_texture: texture_2d<f32>;
@group(1) @binding(4)
var normal_sampler: sampler;
@group(1) @binding(5)
var emissive_texture: texture_2d<f32>;
@group(1) @binding(6)
var emissive_sampler: sampler;
@group(1) @binding(7)
var lighting_texture: texture_2d<f32>;
@group(1) @binding(8)
var lighting_sampler: sampler;

@group(3) @binding(0)
var<storage, read> lighting: Lighting;

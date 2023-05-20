#define_import_path world_material::types

struct WorldMaterial {
    base_color: vec4<f32>,
    emissive: vec4<f32>,
    flags: u32,
    alpha_cutoff: f32,
};

const WORLD_MATERIAL_FLAGS_BASE_COLOR_TEXTURE_BIT: u32         = 1u;
const WORLD_MATERIAL_FLAGS_NORMAL_TEXTURE_BIT: u32             = 2u;
const WORLD_MATERIAL_FLAGS_EMISSIVE_TEXTURE_BIT: u32           = 4u;
const WORLD_MATERIAL_FLAGS_LIGHTING_TEXTURE_BIT: u32           = 8u;
const WORLD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS: u32       = 3758096384u; // (0b111u32 << 29)
const WORLD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE: u32              = 0u;          // (0u32 << 29)
const WORLD_MATERIAL_FLAGS_ALPHA_MODE_MASK: u32                = 536870912u;  // (1u32 << 29)
const WORLD_MATERIAL_FLAGS_ALPHA_MODE_BLEND: u32               = 1073741824u; // (2u32 << 29)
const WORLD_MATERIAL_FLAGS_ALPHA_MODE_PREMULTIPLIED: u32       = 1610612736u; // (3u32 << 29)
const WORLD_MATERIAL_FLAGS_ALPHA_MODE_ADD: u32                 = 2147483648u; // (4u32 << 29)
const WORLD_MATERIAL_FLAGS_ALPHA_MODE_MULTIPLY: u32            = 2684354560u; // (5u32 << 29)

fn world_material_new() -> WorldMaterial {
    var material: WorldMaterial;

    material.base_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    material.emissive = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    material.flags = WORLD_MATERIAL_FLAGS_ALPHA_MODE_OPAQUE;
    material.alpha_cutoff = 0.5;

    return material;
}

struct Lighting {
    lights: array<Light>,
}

struct Light {
    position: vec3<f32>,
    scale: f32,
    color: vec4<f32>,
}

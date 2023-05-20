use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::ShaderType;
use bevy::render::render_resource::{
    AsBindGroupShaderType, RenderPipelineDescriptor, SpecializedMeshPipelineError,
};
use bevy::sprite::{Material2d, Material2dKey, Mesh2dHandle};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bitflags::Flags;

#[derive(AsBindGroup, Reflect, FromReflect, TypeUuid, Debug, Clone)]
#[reflect(Default, Debug)]
#[uuid = "e44fe1b3-5a1c-45ab-90d3-fa310e5af74a"]
#[uniform(0, WorldMaterialUniform)]
// #[bind_group_data(WorldMaterialKey)]
pub struct WorldMaterial {
    pub base_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub normal_texture: Option<Handle<Image>>,
    pub emissive: Color,
    #[texture(5)]
    #[sampler(6)]
    pub emissive_texture: Option<Handle<Image>>,
    #[texture(7)]
    #[sampler(8)]
    pub lighting_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

pub type WorldMaterialKey = Material2dKey<WorldMaterial>;

// #[derive(Copy, Clone, Hash, Eq, PartialEq)]
// pub struct WorldMaterialKey {
//     pub alpha_mode: u8,
// }

impl Default for WorldMaterial {
    fn default() -> Self {
        WorldMaterial {
            base_color: Color::WHITE,
            base_color_texture: None,
            normal_texture: None,
            emissive: Color::NONE,
            emissive_texture: None,
            lighting_texture: None,
            alpha_mode: AlphaMode::Opaque,
        }
    }
}

// impl From<&WorldMaterial> for WorldMaterialKey {
//     fn from(material: &WorldMaterial) -> WorldMaterialKey {
//         WorldMaterialKey {
//             alpha_mode: match material.alpha_mode {
//                 AlphaMode::Opaque => 0,
//                 AlphaMode::Mask(_) => 1,
//                 AlphaMode::Blend => 2,
//                 AlphaMode::Premultiplied => 3,
//                 AlphaMode::Add => 4,
//                 AlphaMode::Multiply => 5,
//             },
//         }
//     }
// }

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct WorldMaterialFlags: u32 {
        const BASE_COLOR_TEXTURE         = (1 << 0);
        const NORMAL_TEXTURE             = (1 << 1);
        const EMISSIVE_TEXTURE           = (1 << 2);
        const LIGHTING_TEXTURE           = (1 << 3);
        const ALPHA_MODE_RESERVED_BITS   = (Self::ALPHA_MODE_MASK_BITS << Self::ALPHA_MODE_SHIFT_BITS);
        const ALPHA_MODE_OPAQUE          = (0 << Self::ALPHA_MODE_SHIFT_BITS);
        const ALPHA_MODE_MASK            = (1 << Self::ALPHA_MODE_SHIFT_BITS);
        const ALPHA_MODE_BLEND           = (2 << Self::ALPHA_MODE_SHIFT_BITS);
        const ALPHA_MODE_PREMULTIPLIED   = (3 << Self::ALPHA_MODE_SHIFT_BITS);
        const ALPHA_MODE_ADD             = (4 << Self::ALPHA_MODE_SHIFT_BITS);
        const ALPHA_MODE_MULTIPLY        = (5 << Self::ALPHA_MODE_SHIFT_BITS);
        const NONE                       = 0;
        const UNINITIALIZED              = 0xFFFF;
    }
}

impl WorldMaterialFlags {
    const ALPHA_MODE_MASK_BITS: u32 = 0b111;
    const ALPHA_MODE_SHIFT_BITS: u32 = 32 - Self::ALPHA_MODE_MASK_BITS.count_ones();
}

#[derive(Clone, Default, ShaderType)]
pub struct WorldMaterialUniform {
    pub base_color: Vec4,
    pub emissive: Vec4,
    /// The [`WorldMaterialFlags`] accessible in the `wgsl` shader.
    pub flags: u32,
    /// When the alpha mode mask flag is set, any base color alpha above this cutoff means fully opaque,
    /// and any below means fully transparent.
    pub alpha_cutoff: f32,
}

impl AsBindGroupShaderType<WorldMaterialUniform> for WorldMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> WorldMaterialUniform {
        let mut flags = WorldMaterialFlags::NONE;
        if self.base_color_texture.is_some() {
            flags |= WorldMaterialFlags::BASE_COLOR_TEXTURE;
        }
        if self.normal_texture.is_some() {
            flags |= WorldMaterialFlags::NORMAL_TEXTURE;
        }
        if self.emissive_texture.is_some() {
            flags |= WorldMaterialFlags::EMISSIVE_TEXTURE;
        }
        if self.lighting_texture.is_some() {
            flags |= WorldMaterialFlags::LIGHTING_TEXTURE;
        }

        let mut alpha_cutoff = 0.5;
        match self.alpha_mode {
            AlphaMode::Opaque => flags |= WorldMaterialFlags::ALPHA_MODE_OPAQUE,
            AlphaMode::Mask(c) => {
                alpha_cutoff = c;
                flags |= WorldMaterialFlags::ALPHA_MODE_MASK;
            }
            AlphaMode::Blend => flags |= WorldMaterialFlags::ALPHA_MODE_BLEND,
            AlphaMode::Premultiplied => flags |= WorldMaterialFlags::ALPHA_MODE_PREMULTIPLIED,
            AlphaMode::Add => flags |= WorldMaterialFlags::ALPHA_MODE_ADD,
            AlphaMode::Multiply => flags |= WorldMaterialFlags::ALPHA_MODE_MULTIPLY,
        };

        WorldMaterialUniform {
            base_color: self.base_color.as_linear_rgba_f32().into(),
            emissive: self.emissive.as_linear_rgba_f32().into(),
            flags: flags.bits(),
            alpha_cutoff,
        }
    }
}

impl Material2d for WorldMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/world.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/world.wgsl".into()
    }
}

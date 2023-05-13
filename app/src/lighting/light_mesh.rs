use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

pub fn make_light_mesh() -> Mesh {
    const RADIUS: f32 = 10.0f32;
    const SIDES: usize = 64;

    let mut positions = Vec::with_capacity(SIDES);
    let mut colors = Vec::with_capacity(SIDES);
    let mut indices = Vec::with_capacity((SIDES - 2) * 3);

    positions.push([0.0, 0.0, 0.0]);
    colors.push(Color::WHITE.as_rgba_f32());

    let step = std::f32::consts::TAU / SIDES as f32;
    for i in 0..SIDES {
        let theta = std::f32::consts::FRAC_PI_2 - i as f32 * step;
        let (sin, cos) = theta.sin_cos();

        positions.push([cos * RADIUS, sin * RADIUS, 0.0]);
        colors.push([0.0, 0.0, 0.0, 0.0]);
    }

    for i in 1..=SIDES as u32 {
        indices.extend_from_slice(&[0, i , i + 1]);
    }
    indices.extend_from_slice(&[0, SIDES as u32, 1]);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.set_indices(Some(Indices::U32(indices)));

    mesh
}

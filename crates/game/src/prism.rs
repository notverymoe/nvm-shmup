// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_asset::RenderAssetUsages}};

#[derive(Debug, Clone, Copy)]
pub struct Prism {
    pub radius: f32,
    pub sides:  usize,
    pub depth:  f32,
}

impl Primitive3d for Prism {}

impl From<Prism> for Mesh {
    fn from(value: Prism) -> Self {
        let Prism{radius, sides, depth} = value;
        let depth = depth/2.0;

        let mut indices   = Vec::with_capacity((sides - 2)*6 + (sides-1)*6);
        let mut positions = Vec::with_capacity(sides*4);
        let mut normals   = Vec::with_capacity(sides*4);
        let mut uvs       = Vec::with_capacity(sides*4);

        let start_angle = core::f32::consts::FRAC_PI_2;
        let step        = core::f32::consts::TAU / sides as f32;

        // Top Face
        for i in 0..sides { 
            let angle = Vec2::from_angle(start_angle + i as f32 * step);
            let point = radius*angle;
            positions.push(point.extend(depth));
            normals.push([0.0, 0.0, 1.0]);
            uvs.push([
                      0.5 * (angle.x + 1.0), // cos(a)
                1.0 - 0.5 * (angle.y + 1.0), // sin(a)
            ]);
        }
        for i in 1..(sides as u32 - 1) {
            indices.extend_from_slice(&[0, i, i + 1]);
        }

        // Bottom Face
        for i in 0..sides {
            positions.push(positions[i] * Vec3::new(1.0, 1.0, -1.0));
            normals.push([-normals[i][0], -normals[i][1], -normals[i][2]]);
            uvs.push([1.0-uvs[i][0], 1.0-uvs[i][1]]);
        }
        let index_offset = indices.len() as u32;
        for i in 1..(sides as u32 - 1) {
            indices.extend_from_slice(&[index_offset, index_offset + i + 1, index_offset + i]);
        }

        // Sides
        for i in 1..=sides {
            let tp = positions[i-1];
            let tc = positions[i%sides];
            let bp = positions[sides + i-1];
            let bc = positions[sides + i%sides];

            let n = (tc - tp).normalize().cross((bp - tp).normalize());
            let n = [n.x, n.y, n.z];

            let index_start = positions.len() as u32;

            positions.extend_from_slice(&[bp, bc, tp, tc]);
            normals.extend_from_slice(&[n, n, n, n]);
            uvs.extend_from_slice(&[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]]);
            indices.extend_from_slice(&[
                index_start, index_start+1, index_start+3,
                index_start, index_start+3, index_start+2,
            ]);
        }

        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL,   normals  )
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0,     uvs      )
            .with_inserted_indices(Indices::U32(indices))
    }
}

impl<'w, 's, T: GizmoConfigGroup> GizmoPrimitive3d<Prism> for Gizmos<'w, 's, T> {
    type Output<'a> = () where Self: 'a;
    
    fn primitive_3d(
        &mut self,
        primitive: Prism,
        translation: Vec3,
        rotation: Quat,
        color: Color,
    ) -> Self::Output<'_> {
        render_gizmo_triangle_mesh_lines(self, primitive.into(), translation, rotation, color);
    }

}

fn render_gizmo_triangle_mesh_lines<T: GizmoConfigGroup>(
    gizmos: &mut Gizmos<'_, '_, T>, 
    mut mesh: Mesh, 
    translation: Vec3,
    rotation: Quat,
    color: Color
) {
    mesh.transform_by(Transform{translation, rotation, scale: Vec3::ONE});

    let Indices::U32(indices) = mesh.indices().unwrap() else { unreachable!() };
    let verts = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap();

    let tri_count = indices.len() / 3;
    for i in 0..tri_count {
        let v0 = Vec3::from_array(verts[indices[i*3    ] as usize]);
        let v1 = Vec3::from_array(verts[indices[i*3 + 1] as usize]);
        let v2 = Vec3::from_array(verts[indices[i*3 + 2] as usize]);

        gizmos.arrow(v0, v1, color);
        gizmos.arrow(v1, v2, color);
        gizmos.arrow(v2, v0, color);
    }
}
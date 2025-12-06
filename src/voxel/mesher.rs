// src/voxel/mesher.rs
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

use crate::voxel::grid::VoxelGrid;

/// Размер одного вокселя в мировых единицах.
pub const VOXEL_SIZE: f32 = 1.0;

/// Направления для 6 граней куба в индексном пространстве грида.
const FACE_OFFSETS: [IVec3; 6] = [
    IVec3::new(0,  1,  0), // +Y (top)
    IVec3::new(0, -1,  0), // -Y (bottom)
    IVec3::new(0,  0,  1), // +Z (front)
    IVec3::new(0,  0, -1), // -Z (back)
    IVec3::new(-1, 0,  0), // -X (left)
    IVec3::new(1,  0,  0), // +X (right)
];

/// Нормали для граней (по тем же индексам).
const FACE_NORMALS: [[f32; 3]; 6] = [
    [0.0,  1.0,  0.0], // +Y
    [0.0, -1.0,  0.0], // -Y
    [0.0,  0.0,  1.0], // +Z
    [0.0,  0.0, -1.0], // -Z
    [-1.0, 0.0,  0.0], // -X
    [1.0,  0.0,  0.0], // +X
];

/// Одинаковые UV для всех граней.
const FACE_UVS: [[f32; 2]; 4] = [
    [0.0, 0.0],
    [1.0, 0.0],
    [1.0, 1.0],
    [0.0, 1.0],
];

/// Строим меш по воксельной решётке:
/// для каждого заполненного вокселя добавляем только те грани,
/// где сосед пустой или вне границ грида.
pub fn build_mesh(grid: &VoxelGrid) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals:   Vec<[f32; 3]> = Vec::new();
    let mut uvs:       Vec<[f32; 2]> = Vec::new();
    let mut indices:   Vec<u32>      = Vec::new();

    let size = grid.size;

    for z in 0..size.z {
        for y in 0..size.y {
            for x in 0..size.x {
                if grid.get(x, y, z) == 0 {
                    continue;
                }

                // Центр вокселя в локальных координатах
                let cx = x as f32 + 0.5;
                let cy = y as f32 + 0.5;
                let cz = z as f32 + 0.5;

                let half = VOXEL_SIZE * 0.5;

                let px = cx + half;
                let nx = cx - half;
                let py = cy + half;
                let ny = cy - half;
                let pz = cz + half;
                let nz = cz - half;

                // Для каждой из 6 граней проверяем соседа
                for face in 0..6 {
                    let offset = FACE_OFFSETS[face];
                    let nx_i = x as i32 + offset.x;
                    let ny_i = y as i32 + offset.y;
                    let nz_i = z as i32 + offset.z;

                    let neighbor_empty = if nx_i < 0
                        || ny_i < 0
                        || nz_i < 0
                        || nx_i >= size.x as i32
                        || ny_i >= size.y as i32
                        || nz_i >= size.z as i32
                    {
                        // Вышли за границы грида — считаем, что там пусто
                        true
                    } else {
                        // Внутри грида — проверяем воксель
                        grid.get(nx_i as u32, ny_i as u32, nz_i as u32) == 0
                    };

                    if neighbor_empty {
                        add_face(
                            face,
                            nx, px, ny, py, nz, pz,
                            &mut positions,
                            &mut normals,
                            &mut uvs,
                            &mut indices,
                        );
                    }
                }
            }
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

/// Добавляем одну грань куба в буферы.
/// face: 0..5 — индекс в FACE_OFFSETS / FACE_NORMALS.
fn add_face(
    face: usize,
    nx: f32,
    px: f32,
    ny: f32,
    py: f32,
    nz: f32,
    pz: f32,
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
) {
    let verts: [[f32; 3]; 4] = match face {
        // +Y (top) — нормаль (0, 1, 0)
        0 => [
            [nx, py, pz],
            [px, py, pz],
            [px, py, nz],
            [nx, py, nz],
        ],
        // -Y (bottom) — нормаль (0, -1, 0)
        1 => [
            [nx, ny, nz],
            [px, ny, nz],
            [px, ny, pz],
            [nx, ny, pz],
        ],
        // +Z (front)
        2 => [
            [nx, ny, pz],
            [px, ny, pz],
            [px, py, pz],
            [nx, py, pz],
        ],
        // -Z (back)
        3 => [
            [px, ny, nz],
            [nx, ny, nz],
            [nx, py, nz],
            [px, py, nz],
        ],
        // -X (left)
        4 => [
            [nx, ny, nz],
            [nx, ny, pz],
            [nx, py, pz],
            [nx, py, nz],
        ],
        // +X (right)
        5 => [
            [px, ny, pz],
            [px, ny, nz],
            [px, py, nz],
            [px, py, pz],
        ],
        _ => unreachable!(),
    };

    let normal = FACE_NORMALS[face];

    let start = positions.len() as u32;
    for (i, p) in verts.iter().enumerate() {
        positions.push(*p);
        normals.push(normal);
        uvs.push(FACE_UVS[i]);
    }

    indices.extend_from_slice(&[
        start,
        start + 1,
        start + 2,
        start,
        start + 2,
        start + 3,
    ]);
}

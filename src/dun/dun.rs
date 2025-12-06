// src/dun/dun.rs
use bevy::prelude::*;

use crate::dun::kind::DunKind;
use crate::voxel::grid::VoxelGrid;

/// Базовый компонент DUN для MVP0.
/// Хранит только то, что реально нужно сейчас.
#[derive(Component)]
pub struct Dun {
    /// Тип DUN (для будущего различения Dynamic / Static и т.п.).
    pub kind: DunKind,
    /// Логическая координата чанка, к которому привязан DUN.
    pub chunk_coord: IVec3,
    /// Локальная воксельная решётка внутри контейнера.
    pub voxel: VoxelGrid,
}

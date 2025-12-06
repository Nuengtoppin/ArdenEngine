// src/voxel/grid.rs
use bevy::prelude::*;

/// Размер грида по одной оси по умолчанию (можно менять при создании)
pub const VOXELS_PER_AXIS: u32 = 32;

/// Простейший воксельный грид: size и плоский массив данных.
/// 0 = пусто, >0 = какой-то материал.
pub struct VoxelGrid {
    pub size: UVec3,
    pub data: Vec<u8>,
}

impl VoxelGrid {
    /// Создаёт грид указанного размера и заполняет нулями.
    pub fn new(size: UVec3) -> Self {
        let len = (size.x * size.y * size.z) as usize;
        Self {
            size,
            data: vec![0; len],
        }
    }

    /// DUN-грид по умолчанию: 32×32×32.
    pub fn new_default() -> Self {
        let size = UVec3::splat(VOXELS_PER_AXIS);
        Self::new(size)
    }

    #[inline]
    fn index(&self, x: u32, y: u32, z: u32) -> Option<usize> {
        if x >= self.size.x || y >= self.size.y || z >= self.size.z {
            return None;
        }
        // порядок как у тебя был: x + X * (y + Y * z)
        let idx = x + self.size.x * (y + self.size.y * z);
        Some(idx as usize)
    }

    #[inline]
    pub fn get(&self, x: u32, y: u32, z: u32) -> u8 {
        if let Some(i) = self.index(x, y, z) {
            self.data[i]
        } else {
            0
        }
    }

    #[inline]
    pub fn set(&mut self, x: u32, y: u32, z: u32, value: u8) {
        if let Some(i) = self.index(x, y, z) {
            self.data[i] = value;
        }
    }

    /// Заполнить весь объём одним значением.
    pub fn fill(&mut self, value: u8) {
        self.data.fill(value);
    }
}

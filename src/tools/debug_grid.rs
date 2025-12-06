// src/tools/debug_grid.rs
use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::KeyCode;

/// Параметры сетки для MVP0-lab.
///
/// По смыслу:
/// - каждые 32 юнита — страйд Octochunk
/// - 64 — Chunk
/// - 256 — Region (у нас extent = 256, так что это граница "пола")
pub const GRID_EXTENT: f32 = 256.0;
pub const GRID_MINOR_STEP: f32 = 1.0;
pub const GRID_OCTOCHUNK_STEP: f32 = 32.0;
pub const GRID_CHUNK_STEP: f32 = 64.0;
pub const GRID_REGION_STEP: f32 = 256.0;

/// Паттерн пунктира для Octochunk-линий:
/// 2 полных клетки → 1 пустая → 1 "точка" → 1 пустая → повтор.
const PATTERN_LEN: i32 = 5;
const DOT_FRACTION: f32 = 0.45; // доля клетки, занимаемая "точкой"

/// Половина длины перекрестия (крестика) на пересечениях Octochunk-сетки.
const OCTO_CROSS_HALF: f32 = 3.0;

#[derive(Copy, Clone, Eq, PartialEq)]
enum GridLineKind {
    Region,
    Chunk,
    Octochunk,
    Minor,
}

/// Состояние оверлея сетки.
/// По умолчанию:
/// - включена только мелкая сетка (воксельная)
/// - Chunk / Octochunk / кресты выключены
pub struct GridOverlayState {
    /// Показывать мелкую сетку (1.0)
    pub show_minor: bool,

    /// Показывать линии Octochunk (32×32, пунктир)
    pub show_octochunk: bool,

    /// Показывать границы Chunk (64×64, сплошные)
    pub show_chunk: bool,

    /// Показывать крестики в узлах Octochunk (перекрёстки 32×32)
    pub show_octochunk_crosses: bool,
}

impl Default for GridOverlayState {
    fn default() -> Self {
        Self {
            show_minor: true,          // воксельная сетка включена
            show_octochunk: false,     // крупные уровни выключены
            show_chunk: false,
            show_octochunk_crosses: false,
        }
    }
}

/// Определяем, к какому уровню топологии относится линия по координате.
fn classify_line(coord: f32) -> GridLineKind {
    let c = coord.round();

    if (c % GRID_REGION_STEP).abs() < 0.001 {
        GridLineKind::Region
    } else if (c % GRID_CHUNK_STEP).abs() < 0.001 {
        GridLineKind::Chunk
    } else if (c % GRID_OCTOCHUNK_STEP).abs() < 0.001 {
        GridLineKind::Octochunk
    } else {
        GridLineKind::Minor
    }
}

/// Цвет для каждого типа линий.
fn color_for_line(kind: GridLineKind) -> Color {
    match kind {
        GridLineKind::Region    => Color::rgba(1.0, 0.3, 0.3, 0.9),   // красный "маяк"
        GridLineKind::Chunk     => Color::rgba(0.95, 0.95, 0.95, 0.9),// почти белый, сплошной
        GridLineKind::Octochunk => Color::rgba(0.9, 0.9, 0.9, 0.8),   // белый пунктир
        GridLineKind::Minor     => Color::rgba(0.45, 0.45, 0.45, 0.25),// тусклая "пыль"
    }
}

/// Основная система отрисовки сетки.
pub fn debug_grid_system(
    mut gizmos: Gizmos,
    camera_q: Query<&Transform, With<Camera3d>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: Local<GridOverlayState>,
) {
    // F3 — переключаем режим:
    // OFF: только воксельная сетка (minor)
    // ON:  Chunk + Octochunk + кресты + (minor остаётся)
    if keyboard.just_pressed(KeyCode::F3) {
        let enable = !state.show_octochunk;
        state.show_octochunk = enable;
        state.show_chunk = enable;
        state.show_octochunk_crosses = enable;
        // мелкую сетку не трогаем — она всегда включена по state.show_minor
    }

    let y = 0.0;
    let extent = GRID_EXTENT;

    // Положение камеры по высоте — используем как простую метрику "насколько близко".
    // Это LOD: даже если режим включен, слишком высоко мелкие детали не рисуем.
    let cam_y = camera_q
        .get_single()
        .map(|t| t.translation.y.abs())
        .unwrap_or(64.0);

    let lod_octochunk = cam_y < 200.0;
    let lod_minor     = cam_y < 80.0;

    // Фактические флаги отрисовки с учётом LOD и состояния.
    let draw_minor     = lod_minor && state.show_minor;
    let draw_octochunk = lod_octochunk && state.show_octochunk;
    let draw_chunk     = state.show_chunk; // при желании сюда тоже можно прикрутить LOD

    // Оси X (красная) и Z (синяя) — для ориентира
    gizmos.line(
        Vec3::new(-extent, y, 0.0),
        Vec3::new( extent, y, 0.0),
        Color::RED,
    );
    gizmos.line(
        Vec3::new(0.0, y, -extent),
        Vec3::new(0.0, y,  extent),
        Color::BLUE,
    );

    // Линии, параллельные оси Z (x меняется)
    let mut x = -extent;
    while x <= extent {
        let kind = classify_line(x);

        match kind {
            GridLineKind::Minor if !draw_minor => {
                x += GRID_MINOR_STEP;
                continue;
            }
            GridLineKind::Octochunk if !draw_octochunk => {
                x += GRID_MINOR_STEP;
                continue;
            }
            GridLineKind::Chunk if !draw_chunk => {
                x += GRID_MINOR_STEP;
                continue;
            }
            _ => {}
        }

        let color = color_for_line(kind);

        match kind {
            GridLineKind::Octochunk => {
                // Для Octochunk-линий рисуем пунктир вдоль Z
                draw_octochunk_dotted_line_z(&mut gizmos, x, y, extent, color);
            }
            _ => {
                // Остальные — сплошные
                gizmos.line(
                    Vec3::new(x, y, -extent),
                    Vec3::new(x, y,  extent),
                    color,
                );
            }
        }

        x += GRID_MINOR_STEP;
    }

    // Линии, параллельные оси X (z меняется)
    let mut z = -extent;
    while z <= extent {
        let kind = classify_line(z);

        match kind {
            GridLineKind::Minor if !draw_minor => {
                z += GRID_MINOR_STEP;
                continue;
            }
            GridLineKind::Octochunk if !draw_octochunk => {
                z += GRID_MINOR_STEP;
                continue;
            }
            GridLineKind::Chunk if !draw_chunk => {
                z += GRID_MINOR_STEP;
                continue;
            }
            _ => {}
        }

        let color = color_for_line(kind);

        match kind {
            GridLineKind::Octochunk => {
                // Для Octochunk-линий рисуем пунктир вдоль X
                draw_octochunk_dotted_line_x(&mut gizmos, z, y, extent, color);
            }
            _ => {
                gizmos.line(
                    Vec3::new(-extent, y, z),
                    Vec3::new( extent, y, z),
                    color,
                );
            }
        }

        z += GRID_MINOR_STEP;
    }

    // Крестики в пересечениях Octochunk-сетки (между 4 Octochunk'ами).
    if draw_octochunk && state.show_octochunk_crosses {
        draw_octochunk_crosses(&mut gizmos, y, extent);
    }
}

/// Пунктир вдоль оси Z при фиксированном X.
/// Паттерн: 2 клетки линия → 1 пустая → 1 "точка" → 1 пустая.
fn draw_octochunk_dotted_line_z(
    gizmos: &mut Gizmos,
    x: f32,
    y: f32,
    extent: f32,
    color: Color,
) {
    let step = GRID_MINOR_STEP;

    let mut z = -extent;
    let mut cell_index: i32 = 0;

    while z < extent {
        let next_z = (z + step).min(extent);
        let pattern = cell_index.rem_euclid(PATTERN_LEN);

        match pattern {
            // 2 полные клетки
            0 | 1 => {
                gizmos.line(
                    Vec3::new(x, y, z),
                    Vec3::new(x, y, next_z),
                    color,
                );
            }
            // пустые клетки — ничего не рисуем
            2 | 4 => { /* gap */ }
            // "точка" — короткий сегмент по центру клетки
            3 => {
                let mid = (z + next_z) * 0.5;
                let half_len = (next_z - z) * DOT_FRACTION * 0.5;
                gizmos.line(
                    Vec3::new(x, y, mid - half_len),
                    Vec3::new(x, y, mid + half_len),
                    color,
                );
            }
            _ => {}
        }

        z = next_z;
        cell_index += 1;
    }
}

/// Пунктир вдоль оси X при фиксированном Z.
/// Тот же паттерн, но по X.
fn draw_octochunk_dotted_line_x(
    gizmos: &mut Gizmos,
    z: f32,
    y: f32,
    extent: f32,
    color: Color,
) {
    let step = GRID_MINOR_STEP;

    let mut x = -extent;
    let mut cell_index: i32 = 0;

    while x < extent {
        let next_x = (x + step).min(extent);
        let pattern = cell_index.rem_euclid(PATTERN_LEN);

        match pattern {
            0 | 1 => {
                gizmos.line(
                    Vec3::new(x,      y, z),
                    Vec3::new(next_x, y, z),
                    color,
                );
            }
            2 | 4 => { /* gap */ }
            3 => {
                let mid = (x + next_x) * 0.5;
                let half_len = (next_x - x) * DOT_FRACTION * 0.5;
                gizmos.line(
                    Vec3::new(mid - half_len, y, z),
                    Vec3::new(mid + half_len, y, z),
                    color,
                );
            }
            _ => {}
        }

        x = next_x;
        cell_index += 1;
    }
}

/// Рисуем крестики в узлах Octochunk-сетки:
/// в каждой точке (k*32, m*32) — небольшой крест.
fn draw_octochunk_crosses(
    gizmos: &mut Gizmos,
    y: f32,
    extent: f32,
) {
    let max_index = (extent / GRID_OCTOCHUNK_STEP).floor() as i32;
    let color = color_for_line(GridLineKind::Octochunk);

    for ix in -max_index..=max_index {
        let x = ix as f32 * GRID_OCTOCHUNK_STEP;

        for iz in -max_index..=max_index {
            let z = iz as f32 * GRID_OCTOCHUNK_STEP;

            // Горизонтальная часть креста
            gizmos.line(
                Vec3::new(x - OCTO_CROSS_HALF, y, z),
                Vec3::new(x + OCTO_CROSS_HALF, y, z),
                color,
            );

            // Вертикальная часть креста
            gizmos.line(
                Vec3::new(x, y, z - OCTO_CROSS_HALF),
                Vec3::new(x, y, z + OCTO_CROSS_HALF),
                color,
            );
        }
    }
}

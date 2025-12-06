// src/main.rs
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod app;
mod mvp0;
mod tools;
mod voxel;
mod render;
mod physics;
mod dun;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Физика
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // Визуализация коллайдеров (можно закомментировать при желании)
        .add_plugins(RapierDebugRenderPlugin::default())
        // Наша сцена MVP0
        .add_plugins(mvp0::scene::Mvp0ScenePlugin)
        .run();
}

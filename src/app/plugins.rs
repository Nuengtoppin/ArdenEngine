// src/app/plugins.rs
use bevy::prelude::*;

/// Общий плагин Arden/Arden Engine для будущих расширений:
/// сюда позже добавим egui, inspector, Rapier и т.п.
pub struct CorePlugins;

impl Plugin for CorePlugins {
    fn build(&self, _app: &mut App) {
        // Пока ничего не добавляем.
        // На следующих фазах здесь будут регистрироваться дополнительные плагины.
    }
}

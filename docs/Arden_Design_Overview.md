# Arden Engine — Design Overview
## (RU) Обзор проекта

Arden — экспериментальный воксельно-векторный движок с упором на:
- разрушаемость мира,
- масштабирование на большие пространства,
- оптимизацию на основе внимания игрока.

### Основная идея
Мир существует только там, где это важно игроку.
Остальная часть мира поддерживается в оптимальном, архивном или спящем состоянии.

### Технологический подход
Гибридная архитектура:
- Voxel Layer хранит геометрию и материалы
- Vector Layer управляет поведением и устойчивостью
- HAOS распределяет внимание
- DTO сохраняет память мира
- Lighting и PFO формируют восприятие

### MVP цель
Создать функциональный прототип:
- 4 сектора внимания
- динамический LOD
- базовые разрушения
- тест объектов в Test Room

---

## (EN) Overview

Arden is an experimental voxel-vector hybrid engine designed for:
- destructible environments,
- scalable world simulation,
- attention-driven performance management.

### Core principle
The world is only fully simulated where the player cares.
Everything else remains dormant or archived.

### Hybrid architecture
- Voxel Layer — geometry & material source of truth
- Vector Layer — behavior & proxy physics
- HAOS — sector lifecycle management
- DTO — archived world data
- Lighting & PFO — optimized perception

### MVP Goal
Functional demo featuring:
- 4 attention sectors
- dynamic LOD
- basic destruction
- Test Room for object validation

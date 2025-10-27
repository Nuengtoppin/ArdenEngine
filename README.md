RU
ArdenEngine

Arden — экспериментальный движок/песочница для модульной симуляции и визуализации мира. Цель — собрать MVP из готовых библиотек (Bevy/wgpu/egui и др.), документировать архитектуру и поэтапно наращивать сложность: от тест‑комнаты и 32х32х16 «окточанков» до фильтра восприятия (PFO), слоя вокселей, и «дворецкого» (Butler) как валидационного/координационного слоя.

Ключевые идеи

Модульность и этапность (ядро → утилиты → визуализация → оптимизации).

«Aspectrolog» — вспомогательная утилита/подмодуль: из словаря тегов собирает «аспекты» и пакует их в библиотеку, которую использует ядро.

«Butler» — валидационный слой/оркестратор: наблюдает, проверяет целостность, помогает отлаживать.

Воксельный слой + LOD/SVO/октри, связка с векторным слоем, деградация/упрощение для производительности.

Клиент/сервер‑разделение: сервер — источник истины, клиент — точка восприятия с локальными оптимизациями.

Статус: ранний черновой MVP.

📚 Документация: см. docs/README.md

🧭 Дорожная карта: docs/mvp/MVP_Roadmap_Sectors_RU.md

🧪 Пример: examples/test_room

EN
ArdenEngine

Arden is an experimental sandbox/engine for modular world simulation and visualization. The goal is to assemble an MVP from existing libraries (Bevy/wgpu/egui, etc.), document the architecture, and scale step‑by‑step—from a test room and 32x32x16 "octochunks" to a Perceptual Filter (PFO), voxel layer, and a Butler (validation/coordination layer).

Key ideas

Modularity & phased growth (core → utilities → visualization → optimizations).

Aspectrolog — a helper submodule/tool that builds "aspects" from a tag dictionary and packages them into a reusable library for the core.

Butler — validation/orchestration layer for integrity checks and developer‑friendly debugging.

Voxel layer + LOD/SVO/octree with vector layer links and degradation strategies for performance.

Client/Server split: server is authoritative; client applies local optimizations as a point of perception.

Status: early draft MVP.

📚 Docs: see docs/README.md

🧭 Roadmap: docs/mvp/MVP_Roadmap_Sectors_EN.md

🧪 Example: examples/test_room

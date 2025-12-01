Структура папок тома Disk
Серийный номер тома: 0000002D 5A06:184E
D:\AE\ARDEN\ARDEN_VOXELENGINE
|   .gitignore
|   Cargo.toml
|   CODE_OF_CONDUCT.md
|   CONTRIBUTING.md
|   LICENSE-APACHE
|   LICENSE-CC-BY
|   LICENSE-MIT
|   README.md
|   repo_structure.md
|   
+---assets
|       Arden_banner_crop_1280x640.jpg
|       
+---docs
|   +---EN
|   |   |   roadmap.md
|   |   |   
|   |   +---ARCHITECTURE
|   |   |   |   readme.md
|   |   |   |   
|   |   |   +---1_TopologyLogic_Route_Rotation
|   |   |   |       Examples.md
|   |   |   |       README.md
|   |   |   |       Rotation.md
|   |   |   |       Routing.md
|   |   |   |       Torology.md
|   |   |   |       
|   |   |   \---2_Dynamic_unit_node
|   |   |           DUN.md
|   |   |           README.md
|   |   |           
|   |   +---CONCEPT
|   |   |       Concept_Overview.md
|   |   |       readme.md
|   |   |       
|   |   +---MVP
|   |   |       MVP_Structure_RU.md
|   |   |       
|   |   +---ROADMAP
|   |   |       roadmap_2025.md
|   |   |       
|   |   \---TERMS
|   |           Comparisons.md
|   |           Glossary.md
|   |           
|   \---RU
|       |   roadmap.md
|       |   
|       +---ARCHITECTURE
|       |   |   readme.md
|       |   |   
|       |   +---1_TopologyLogic_Route_Rotation
|       |   |       Examples.md
|       |   |       README.md
|       |   |       Rotation.md
|       |   |       Routing.md
|       |   |       Topology.md
|       |   |       
|       |   \---2_Dynamic_unit_node
|       |           DUN.md
|       |           README.md
|       |           
|       +---CONCEPT
|       |       Concept_Overview.md
|       |       readme.md
|       |       
|       +---MVP
|       |       MVP_0.1.md
|       |       MVP_Structure.md
|       |       
|       +---ROADMAP
|       |       roadmap_2025.md
|       |       
|       \---TERMS
|               Comparisons.md
|               Glossary.md
|               
+---meta
|       focus_2025.md
|       git_history.txt
|       log_2025.md
|       log_2026.md
|       origin_and_rights.md
|       project_status.md
|       README.md
|       roles_and_structure.md
|       status_index.md
|       status_system.md
|       
+---root
|       README_EN.md
|       README_RU.md
|       
\---src
    |   main.rs
    |   
    +---app
    |       mod.rs
    |       plugins.rs
    |       setup.rs
    |       
    +---dun
    |       dun.rs
    |       kind.rs
    |       mod.rs
    |       route_lite.rs
    |       spawn.rs
    |       transform.rs
    |       
    +---mvp0
    |       mod.rs
    |       scene.rs
    |       stress_test.rs
    |       
    +---physics
    |       collider_builder.rs
    |       mod.rs
    |       
    +---render
    |       mesh_builder.rs
    |       mod.rs
    |       
    +---tools
    |       debug_grid.rs
    |       egui_panel.rs
    |       mod.rs
    |       
    \---voxel
            grid.rs
            mesher.rs
            mod.rs
            

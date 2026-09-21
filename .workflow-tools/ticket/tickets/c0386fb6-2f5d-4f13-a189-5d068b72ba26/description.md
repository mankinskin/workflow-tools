# Doc Editor: Documentation as Frosted Glass Panels in Voxel-Splatted World

## Problem

Documentation pages (from doc-viewer / MCP doc sources) are displayed as frosted glass panels in the 3D Voxel-splatted scene. Docs are read-heavy, so they use higher roughness for readability — the mipmap-blurred voxel-splatted background provides ambient context without distracting from text.

## Architecture

### Doc Panel Entity

```rust
#[derive(Component)]
pub struct DocPanel {
    pub doc_id: String,
    pub doc_type: DocType,  // AgentDoc, CrateDoc, Guide
    pub scroll_offset: f32,
}

#[derive(Bundle)]
pub struct DocPanelBundle {
    pub doc: DocPanel,
    pub world_panel: WorldPanel,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
```

### Doc Rendering

Doc content is rendered to a texture via the Dioxus→Taffy pipeline (T9). Markdown is parsed and laid out with styled text runs, code blocks (with syntax highlighting), and section headers.

```rust
fn spawn_doc_panel(doc: &Document, pos: Vec3) -> DocPanelBundle {
    DocPanelBundle {
        doc: DocPanel {
            doc_id: doc.id.clone(),
            doc_type: doc.doc_type,
            scroll_offset: 0.0,
        },
        world_panel: WorldPanel {
            half_extents: Vec2::new(2.0, 3.0),  // tall panel for text
            corner_radius: 0.08,
            content_texture: render_doc_content(doc),
            roughness: 0.6,   // frosted — readable text over blurred voxel-splatted scene
            tint: Vec3::new(0.95, 0.95, 1.0),  // slight cool tint
            anchor: PanelAnchor::WorldFixed(pos, Quat::IDENTITY),
        },
        transform: Transform::from_translation(pos),
        global_transform: GlobalTransform::default(),
    }
}
```

### Frosted Glass for Readability

Docs use roughness 0.6 which translates to mipmap level ~5.4 (roughness × 9.0). The voxel-splatted scene behind the panel is heavily blurred, providing ambient visual context (you can see shapes/colors) without interfering with text readability. Curvature-adaptive blur at panel edges (fwidth(panel_normal) × 4.0) gives a polished glass-edge look.

### Scroll

Scroll offset shifts UV in the content texture. Long documents use virtual scrolling — only the visible region is rendered to the content texture.

### Cross-References

Doc panels can link to other doc panels or ticket panels. Clicking a cross-reference navigates the camera to the referenced panel (smooth camera lerp).

## Dependencies
- T10 (3D UI): WorldPanel, glass SDF, billboard
- T3 (liquid glass): Mipmap frosted blur for panel background
- T9 (bridge): Dioxus→Taffy for content texture rendering
- doc-viewer MCP / doc sources: Document data

## Acceptance Criteria
1. Doc panels render as frosted glass (roughness ~0.6) in 3D world
2. Text is readable over blurred voxel-splatted background
3. Scroll works within panels
4. Code blocks have syntax highlighting
5. Cross-references navigate camera to other panels
6. Panel edge has curvature-adaptive blur (polished glass look)

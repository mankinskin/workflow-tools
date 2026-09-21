# Panel Interaction: 3D Ray-Cast Hit Testing and Input Handling

## Problem

World panels must respond to mouse clicks, hovers, and drags. This requires ray-casting from the camera through mouse position, intersecting with panel planes, and dispatching input events to the correct panel's content layer.

## Scope

### 3D Ray-Cast Hit Testing

```rust
fn panel_raycast_system(
    mouse: Res<MousePosition>,
    camera: Query<(&Transform, &Projection), With<Camera3d>>,
    panels: Query<(&WorldPanel, &Transform, Entity)>,
    mut hovered: ResMut<HoveredPanel>,
) {
    let ray = camera_ray(&mouse, &camera);
    let mut closest: Option<(Entity, f32, Vec2)> = None;

    for (panel, tf, entity) in panels.iter() {
        if let Some((dist, panel_uv)) = ray_panel_intersect(&ray, tf, panel) {
            if closest.is_none() || dist < closest.unwrap().1 {
                closest = Some((entity, dist, panel_uv));
            }
        }
    }

    hovered.entity = closest.map(|(e, _, uv)| (e, uv));
}
```

### Panel Priority Over World Geometry

Panels are tested before SVO raycasts. If a panel is hit, the click goes to the panel; otherwise it falls through to world interaction (e.g., voxel editing).

### Input Event Dispatch

```rust
fn panel_input_system(
    hovered: Res<HoveredPanel>,
    input: Res<InputState>,
    mut click_events: EventWriter<PanelClickEvent>,
    mut hover_events: EventWriter<PanelHoverEvent>,
) {
    if let Some((entity, uv)) = hovered.entity {
        hover_events.send(PanelHoverEvent { entity, uv });
        if input.mouse_left_just_pressed {
            click_events.send(PanelClickEvent { entity, uv });
        }
    }
}
```

### Drag to Reposition

Panels can be dragged in 3D space (constrained to a plane perpendicular to the camera).

## Dependencies
- T10a (WorldPanel rendering): WorldPanel component, panel transforms
- T8 (character): Camera transform for ray construction

## Acceptance Criteria
1. Mouse hover highlights the correct panel
2. Click dispatches PanelClickEvent with panel UV coordinates
3. Panels have priority over world geometry for input
4. Drag-to-reposition moves panels smoothly in 3D space
5. No false hits on panels behind the camera

# Ticket Editor: Tickets as Glass Panels in Voxel-Splatted World

## Problem

Ticket data from the ticket-api is rendered as interactive 3D panels within the Voxel-splatted world. Each ticket becomes a glass SDF panel (T10) displaying ticket fields, with dependency edges visualized as voxel connections that generate splats.

## Architecture

### Ticket Panel Entity

```rust
#[derive(Component)]
pub struct TicketPanel {
    pub ticket_id: String,
    pub state: TicketState,
    pub priority: Priority,
}

#[derive(Bundle)]
pub struct TicketPanelBundle {
    pub ticket: TicketPanel,
    pub world_panel: WorldPanel,  // from T10 — glass SDF + content texture
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
```

### Ticket → Panel Mapping

```rust
fn spawn_ticket_panels(
    tickets: Res<TicketStore>,
    mut commands: Commands,
    layout: Res<GraphLayout>,  // force-directed or grid layout
) {
    for ticket in tickets.iter() {
        let pos = layout.position_for(&ticket.id);
        commands.spawn(TicketPanelBundle {
            ticket: TicketPanel {
                ticket_id: ticket.id.clone(),
                state: ticket.state,
                priority: ticket.priority,
            },
            world_panel: WorldPanel {
                half_extents: Vec2::new(1.5, 1.0),
                corner_radius: 0.1,
                content_texture: render_ticket_content(&ticket),
                roughness: match ticket.priority {
                    Priority::Critical => 0.0,  // clear glass — urgent, see through
                    Priority::High => 0.15,
                    Priority::Medium => 0.4,
                    Priority::Low => 0.7,       // frosted — low urgency
                    _ => 0.5,
                },
                tint: state_to_tint(ticket.state),
                anchor: PanelAnchor::WorldFixed(pos, Quat::IDENTITY),
            },
            transform: Transform::from_translation(pos),
            global_transform: GlobalTransform::default(),
        });
    }
}
```

### Dependency Edges as Voxel Lines

Dependency edges between tickets are drawn as thin voxel lines in the SVO. These voxels participate in splat generation — each edge voxel produces a splat, so dependency lines appear as soft glowing connections in the 3D scene:

```rust
fn draw_ticket_edges(
    tickets: Res<TicketStore>,
    panels: Query<(&TicketPanel, &Transform)>,
    mut svo: ResMut<VoxelWorld>,
) {
    for edge in tickets.edges() {
        let from_pos = find_panel_pos(&panels, &edge.from);
        let to_pos = find_panel_pos(&panels, &edge.to);
        // Bresenham 3D line in SVO
        svo.draw_voxel_line(from_pos, to_pos, edge_color(edge.kind));
    }
}
```

The splats generated from edge voxels inherit their PBR material parameters from the edge color/material, creating colored glowing connections. Critical path edges could have metallic PBR (bright specular highlights) while normal edges use diffuse PBR.

### Interaction

- Click on panel → open ticket details (full description in Dioxus side panel)
- Drag panel → reposition in 3D space
- Double-click → edit ticket fields inline
- Hover → show tooltip with summary

### Glass Visual Encoding

| Priority | Roughness | Visual |
|----------|-----------|--------|
| Critical | 0.0 | Crystal clear glass — splats fully visible through panel |
| High | 0.15 | Slight frost — barely blurred |
| Medium | 0.4 | Moderate frost — background blurred |
| Low | 0.7 | Heavy frost — mostly opaque |

| State | Tint Color |
|-------|------------|
| New | Blue |
| In-progress | Yellow |
| In-review | Orange |
| Done | Green |
| Cancelled | Gray |

## Dependencies
- T10 (3D UI): WorldPanel + glass SDF for each ticket
- T3 (liquid glass): Glass refraction + mipmap blur for panel backgrounds
- T6 (3D scene): Edge voxels → splats via generation pipeline
- T7 (physics): SVO for edge voxel storage
- ticket-api: Ticket data source

## Acceptance Criteria
1. Each ticket renders as a glass panel in 3D world
2. Priority maps to glass roughness (clear=critical, frosted=low)
3. State maps to tint color
4. Dependency edges rendered as voxel lines that generate splats
5. Click interaction opens ticket details
6. Panel content (title, state, priority) rendered via Dioxus→texture
7. Force-directed or grid layout positions panels without overlap

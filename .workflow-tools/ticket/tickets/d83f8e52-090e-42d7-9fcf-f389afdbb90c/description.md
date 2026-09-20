# T8: Port — ticket-viewer Leptos Frontend

## Problem

The ticket-viewer currently uses a Preact/TS frontend with WebGPU 3D dependency graph visualization, SVG fallback, SSE live streaming, and a ticket list grouped by state. This needs a Leptos/WASM port that reuses the shared WgpuOverlay from viewer-api-leptos (T6), implements force-directed graph layout, SSE integration, and full ticket CRUD interaction.

## Reference: TS Implementation

### Backend (tools/viewer/ticket-viewer/src/main.rs)
- **L42–53**: `CliOptions` — port, static_dir, workspace, index_root
- **L95–125**: Server setup: TicketStore, StreamBroker for SSE, ticket_http router, static files

### API endpoints (via ticket-http)
- `GET /api/workspaces` — list workspaces
- `GET /api/tickets?workspace=...` — list tickets with filters
- `GET /api/tickets/{id}` — single ticket detail
- `GET /api/tickets/{id}/description` — ticket description.md content
- `GET /api/edges?workspace=...` — all edges
- `GET /api/graph/subgraph?root=...&depth=...` — subgraph BFS
- `GET /api/graph/topgraph?root=...&depth=...` — reverse subgraph
- `GET /api/graph/health?all=true` — health check
- `GET /api/stream?workspace=...` — SSE event stream (ticket updates, creates, deletes)
- `PUT /api/tickets/{id}` — update ticket
- `POST /api/tickets` — create ticket

### App.tsx — Root layout
- **L39–110**: Tri-pane: Header (workspace picker + theme) → Sidebar (TicketTree) → Center (TicketContent) → Right (DependencyGraph resizable)

### DependencyGraph.tsx — GPU + SVG fallback
- **L22–24**: `hasWebGPU()` — checks navigator.gpu
- **L32–65**: Fetches subgraph via `/api/graph/subgraph?root={id}&depth=3`
- **L67–92**: Maps response → GraphSnapshot (nodes + edges with layout indexes)
- **L94–99**: Custom `renderNode()` — renders TicketCard (state color + title) instead of default badges
- **L107–145**: Conditional: falls back to GraphView SVG if !canUse3D, else HypergraphViewCore

### GraphView.tsx — SVG fallback
- **L1–17**: STATE_COLORS map: new→#4a9eff, in-refinement→#f0c040, in-implementation→#f0a500, ready→#80d4ff, closed→#68d391, cancelled→#a0a0a0
- **L47–132**: Force-directed layout: groups by depth, positions in column grid, draws SVG edges with arrow markers
- **L134–220**: SVG rendering: circles (root 14px, others 11px), labels, state badges, click handlers

### TicketContent.tsx — Detail panel
- **L30–44**: TABS: [description, fields]
- **L46–120**: Tabbed panel: markdown description via marked.parse() + highlighted code, or fields as TOML view

### TicketTree.tsx — Ticket list grouped by state
- **L39–48**: State badge colors
- **L54–60**: TICKET_SORT_OPTIONS: title, updated_at, created_at
- **L62–105**: `buildTree()` — groups by state, sorts within groups, builds TreeNode hierarchy
- **L107–135**: `openTicket(id)` — loads detail + description in parallel
- **L137+**: Component: FileTree + search input + state filter buttons + sort dropdown

### store.ts — State management
- **L16–19**: Ticket signals: treeFilter, treeStateFilter, treeSortState
- **L21–35**: `filteredTickets` computed
- **L37–43**: Open ticket: openTicketId, openTicketDetail, openTicketDescription, activeTab
- **L49–72**: WorkspaceUIState + localStorage persistence
- **L74–100**: URL hash routing sync

### theme.ts — Theme with ticket-specific presets
- **L13–120**: 3 ticket-specific presets: DARK_COLORS, PAPER_COLORS, SCRATCHBOARD_COLORS
- **L142–155**: Saved themes + localStorage persistence
- **L157–240**: Full save/load/export/import store

## Design

### Step 1: Crate setup

```
tools/viewer/ticket-viewer/frontend-leptos/
├── Cargo.toml
├── Trunk.toml  
├── index.html
├── src/
│   ├── lib.rs
│   ├── app.rs
│   ├── api.rs           # Fetch wrappers for all ticket API endpoints
│   ├── store.rs         # TicketStore signals
│   ├── types.rs         # Ticket, Edge, SubgraphResponse, GraphSnapshot
│   ├── actions.rs       # load_tickets, open_ticket, update_ticket, create_ticket
│   ├── sse.rs           # SSE stream consumer
│   ├── graph/
│   │   ├── mod.rs
│   │   ├── layout.rs    # Force-directed layout algorithm
│   │   ├── gpu_graph.rs # WgpuOverlay integration with ticket node rendering
│   │   └── svg_graph.rs # SVG fallback
│   └── components/
│       ├── mod.rs
│       ├── workspace_picker.rs  # Workspace selector dropdown
│       ├── ticket_tree.rs       # Ticket list grouped by state
│       ├── ticket_content.rs    # Detail panel (description + fields tabs)
│       ├── dependency_graph.rs  # GPU or SVG graph wrapper
│       └── ticket_card.rs       # Ticket card for graph nodes
├── style.css
└── static/
```

```toml
[package]
name = "ticket-viewer-leptos"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
viewer-api-leptos = { path = "../../viewer-api/frontend-leptos", features = ["gpu"] }
leptos = { version = "0.8", features = ["csr"] }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = [
    "Window", "Document", "HtmlElement", "EventSource",
    "MessageEvent", "Event", "SvgElement", "SvgsvgElement",
    "MouseEvent", "WheelEvent", "Navigator"
]}
js-sys = "0.3"
gloo-net = "0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
console_log = "1"
log = "0.4"
```

### Step 2: Store — global signals

```rust
// src/store.rs
pub struct TicketStore {
    // Workspaces
    pub workspaces: RwSignal<Vec<String>>,
    pub active_workspace: RwSignal<Option<String>>,
    
    // Ticket list
    pub tickets: RwSignal<Vec<Ticket>>,
    pub edges: RwSignal<Vec<Edge>>,
    pub tree_filter: RwSignal<String>,
    pub tree_state_filter: RwSignal<Option<String>>,
    pub tree_sort: RwSignal<SortField>,
    
    // Filtered/sorted tickets (derived)
    // pub filtered_tickets: Memo<Vec<Ticket>>,  — built in new()
    
    // Open ticket detail
    pub open_ticket_id: RwSignal<Option<String>>,
    pub open_ticket_detail: RwSignal<Option<Ticket>>,
    pub open_ticket_description: RwSignal<Option<String>>,
    pub active_tab: RwSignal<DetailTab>,  // Description | Fields
    pub detail_loading: RwSignal<bool>,
    
    // Graph
    pub graph_snapshot: RwSignal<Option<GraphSnapshot>>,
    pub graph_root_id: RwSignal<Option<String>>,
    
    // URL hash sync
    // pub url_state: ...
    
    // Theme
    pub theme_store: ThemeStore,
    
    // Global
    pub is_loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
}
```

### Step 3: SSE live streaming

```rust
// src/sse.rs
use web_sys::EventSource;

pub fn connect_sse(store: &TicketStore) -> EventSource {
    let workspace = store.active_workspace.get().unwrap_or_default();
    let url = format!("/api/stream?workspace={}", workspace);
    let es = EventSource::new(&url).expect("EventSource");
    
    // Ticket update events
    let store_clone = store.clone();
    let on_ticket_update = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
            let text: String = text.into();
            if let Ok(event) = serde_json::from_str::<SseEvent>(&text) {
                match event {
                    SseEvent::TicketCreated(t) => {
                        store_clone.tickets.update(|ts| ts.push(t));
                    }
                    SseEvent::TicketUpdated(t) => {
                        store_clone.tickets.update(|ts| {
                            if let Some(pos) = ts.iter().position(|x| x.id == t.id) {
                                ts[pos] = t;
                            }
                        });
                        // Refresh open detail if it's the same ticket
                        if store_clone.open_ticket_id.get().as_deref() == Some(&t.id) {
                            store_clone.open_ticket_detail.set(Some(t));
                        }
                    }
                    SseEvent::TicketDeleted(id) => {
                        store_clone.tickets.update(|ts| ts.retain(|x| x.id != id));
                    }
                    SseEvent::EdgeCreated(edge) => {
                        store_clone.edges.update(|es| es.push(edge));
                    }
                }
            }
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    
    es.add_event_listener_with_callback("message", on_ticket_update.as_ref().unchecked_ref())
        .unwrap();
    on_ticket_update.forget();  // Leak — lives for app lifetime
    
    es
}
```

### Step 4: Force-directed graph layout

Port the TS layout algorithm:

```rust
// src/graph/layout.rs

pub struct LayoutNode {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub state: String,
    pub title: String,
    pub is_root: bool,
}

pub struct LayoutEdge {
    pub from_idx: usize,
    pub to_idx: usize,
}

pub fn compute_layout(snapshot: &GraphSnapshot) -> (Vec<LayoutNode>, Vec<LayoutEdge>) {
    // 1. BFS from root to assign depth levels
    // 2. Group nodes by depth
    // 3. Position: x = depth * COLUMN_SPACING, y = index_in_group * ROW_SPACING
    // 4. Center each depth group vertically
    // 5. Build edge list with node indexes
    
    let column_spacing = 200.0;
    let row_spacing = 80.0;
    // ...
}
```

### Step 5: GPU graph with WgpuOverlay

```rust
// src/graph/gpu_graph.rs
use viewer_api_leptos::gpu::WgpuOverlay;

#[component]
pub fn GpuGraph(
    snapshot: Signal<Option<GraphSnapshot>>,
    on_node_click: Callback<String>,
) -> impl IntoView {
    // WgpuOverlay renders the background (ambient particles, connections)
    // Ticket nodes are rendered as HTML overlays on top (positioned via world_to_screen)
    
    let overlay_ref = create_node_ref::<html::Div>();
    
    // Compute layout when snapshot changes
    let layout = create_memo(move |_| {
        snapshot.get().map(|s| compute_layout(&s))
    });
    
    // Node positions for HTML overlay
    let node_views = move || {
        layout.get().map(|(nodes, _)| {
            nodes.iter().map(|node| {
                let id = node.id.clone();
                let color = state_color(&node.state);
                view! {
                    <div class="graph-ticket-card"
                         style=format!("left:{}px;top:{}px;border-color:{}", node.x, node.y, color)
                         on:click=move |_| on_node_click.call(id.clone())>
                        <span class="card-state" style=format!("background:{}", color)>
                            {&node.state}
                        </span>
                        <span class="card-title">{&node.title}</span>
                    </div>
                }
            }).collect_view()
        })
    };
    
    view! {
        <div class="gpu-graph-container" node_ref=overlay_ref>
            <WgpuOverlay container=overlay_ref mode=OverlayMode::Graph />
            <div class="graph-nodes-overlay">
                {node_views}
            </div>
        </div>
    }
}
```

### Step 6: SVG fallback graph

```rust
// src/graph/svg_graph.rs

fn state_color(state: &str) -> &'static str {
    match state {
        "new" => "#4a9eff",
        "in-refinement" => "#f0c040",
        "in-implementation" => "#f0a500",
        "ready" => "#80d4ff",
        "closed" => "#68d391",
        "cancelled" => "#a0a0a0",
        _ => "#888888",
    }
}

#[component]
pub fn SvgGraph(
    snapshot: Signal<Option<GraphSnapshot>>,
    on_node_click: Callback<String>,
) -> impl IntoView {
    let layout = create_memo(move |_| {
        snapshot.get().map(|s| compute_layout(&s))
    });
    
    view! {
        <svg class="graph-svg" viewBox=move || compute_viewbox(&layout.get())>
            // Arrow marker definition
            <defs>
                <marker id="arrow" viewBox="0 0 10 10" refX="10" refY="5"
                        markerWidth="6" markerHeight="6" orient="auto-start-reverse">
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="var(--text-muted)" />
                </marker>
            </defs>
            
            // Edges
            <For each=move || layout.get().map(|(_, edges)| edges).unwrap_or_default()
                 key=|e| (e.from_idx, e.to_idx)
                 let:edge>
                // Line from source node center to target node center
                <line ... marker-end="url(#arrow)" />
            </For>
            
            // Nodes
            <For each=move || layout.get().map(|(nodes, _)| nodes).unwrap_or_default()
                 key=|n| n.id.clone()
                 let:node>
                <g class="graph-node" on:click=...>
                    <circle r=if node.is_root { 14 } else { 11 }
                            fill=state_color(&node.state) />
                    <text dy="24" text-anchor="middle">{&node.title}</text>
                </g>
            </For>
        </svg>
    }
}
```

### Step 7: TicketTree — grouped list

```rust
// src/components/ticket_tree.rs
use viewer_api_leptos::components::{TreeView, TreeNode};

const STATE_ORDER: &[&str] = &[
    "new", "in-refinement", "ready", "in-implementation", "closed", "cancelled"
];

fn build_tree(tickets: &[Ticket], filter: &str, state_filter: Option<&str>, sort: SortField) -> Vec<TreeNode> {
    let mut filtered: Vec<&Ticket> = tickets.iter()
        .filter(|t| {
            let matches_text = filter.is_empty() || 
                t.title.to_lowercase().contains(&filter.to_lowercase());
            let matches_state = state_filter.map_or(true, |s| t.state == s);
            matches_text && matches_state
        })
        .collect();
    
    // Sort within groups
    filtered.sort_by(|a, b| match sort {
        SortField::Title => a.title.cmp(&b.title),
        SortField::UpdatedAt => b.updated_at.cmp(&a.updated_at),
        SortField::CreatedAt => b.created_at.cmp(&a.created_at),
    });
    
    // Group by state
    STATE_ORDER.iter()
        .filter_map(|state| {
            let group: Vec<TreeNode> = filtered.iter()
                .filter(|t| t.state == *state)
                .map(|t| TreeNode {
                    id: t.id.clone(),
                    label: t.title.clone(),
                    icon: TreeNodeIcon::Ticket,
                    children: vec![],
                    is_expanded: false,
                })
                .collect();
            
            if group.is_empty() { return None; }
            Some(TreeNode {
                id: format!("state:{}", state),
                label: format!("{} ({})", state, group.len()),
                icon: TreeNodeIcon::Folder,
                children: group,
                is_expanded: true,
            })
        })
        .collect()
}
```

### Step 8: TicketContent — detail panel

```rust
// src/components/ticket_content.rs

#[component]
pub fn TicketContent(store: TicketStore) -> impl IntoView {
    view! {
        <div class="ticket-content">
            <Show when=move || store.open_ticket_detail.get().is_some()
                  fallback=|| view! { <div class="placeholder">"Select a ticket"</div> }>
                <div class="content-tabs">
                    <button class:active=move || store.active_tab.get() == DetailTab::Description
                            on:click=move |_| store.active_tab.set(DetailTab::Description)>
                        "Description"
                    </button>
                    <button class:active=move || store.active_tab.get() == DetailTab::Fields
                            on:click=move |_| store.active_tab.set(DetailTab::Fields)>
                        "Fields"
                    </button>
                </div>
                <div class="content-body">
                    {move || match store.active_tab.get() {
                        DetailTab::Description => view! {
                            <DescriptionPanel description=store.open_ticket_description />
                        }.into_any(),
                        DetailTab::Fields => view! {
                            <FieldsPanel ticket=store.open_ticket_detail />
                        }.into_any(),
                    }}
                </div>
            </Show>
        </div>
    }
}

#[component]
fn DescriptionPanel(description: RwSignal<Option<String>>) -> impl IntoView {
    // Render markdown description using pulldown-cmark (reuse from T7 or inline)
    move || {
        description.get().map(|md| {
            // Use pulldown-cmark → Leptos DOM (same approach as T7)
            render_markdown(&md)
        })
    }
}

#[component]
fn FieldsPanel(ticket: RwSignal<Option<Ticket>>) -> impl IntoView {
    move || {
        ticket.get().map(|t| {
            view! {
                <div class="fields-panel">
                    <div class="field"><label>"ID"</label><span>{&t.id}</span></div>
                    <div class="field"><label>"State"</label><span class="state-badge">{&t.state}</span></div>
                    <div class="field"><label>"Type"</label><span>{&t.ticket_type}</span></div>
                    <div class="field"><label>"Priority"</label><span>{t.priority.as_deref().unwrap_or("-")}</span></div>
                    <div class="field"><label>"Created"</label><span>{&t.created_at}</span></div>
                    <div class="field"><label>"Updated"</label><span>{&t.updated_at}</span></div>
                </div>
            }
        })
    }
}
```

### Step 9: DependencyGraph wrapper — GPU vs SVG

```rust
// src/components/dependency_graph.rs

#[component]
pub fn DependencyGraph(store: TicketStore) -> impl IntoView {
    let can_use_gpu = create_resource(|| (), |_| async {
        // Check WebGPU availability
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        js_sys::Reflect::get(&navigator, &"gpu".into()).is_ok()
    });
    
    // Fetch subgraph when ticket changes
    let snapshot = create_resource(
        move || store.open_ticket_id.get(),
        move |ticket_id| async move {
            let id = ticket_id?;
            let workspace = store.active_workspace.get()?;
            let url = format!("/api/graph/subgraph?workspace={}&root={}&depth=3", workspace, id);
            let resp = gloo_net::http::Request::get(&url).send().await.ok()?;
            resp.json::<SubgraphResponse>().await.ok()
        }
    );
    
    let on_node_click = Callback::new(move |id: String| {
        spawn_local(open_ticket(&store, &id));
    });
    
    view! {
        <div class="dependency-graph">
            <Suspense fallback=|| view! { <Spinner /> }>
                {move || {
                    let snap = snapshot.get().flatten();
                    let gpu = can_use_gpu.get().unwrap_or(false);
                    
                    if gpu {
                        view! { <GpuGraph snapshot=Signal::derive(move || snap.clone()) on_node_click=on_node_click /> }.into_any()
                    } else {
                        view! { <SvgGraph snapshot=Signal::derive(move || snap.clone()) on_node_click=on_node_click /> }.into_any()
                    }
                }}
            </Suspense>
        </div>
    }
}
```

### Step 10: App root

```rust
// src/app.rs
use viewer_api_leptos::components::{TriPaneLayout, Header, ResizeHandle};

#[component]
pub fn App() -> impl IntoView {
    let store = TicketStore::new();
    provide_context(store.clone());
    
    // Load initial data
    spawn_local(async move {
        load_workspaces(&store).await;
        if let Some(ws) = store.active_workspace.get() {
            load_tickets(&store, &ws).await;
        }
    });
    
    // Connect SSE
    create_effect(move |prev_es: Option<Option<EventSource>>| {
        // Disconnect previous connection on workspace change
        if let Some(Some(es)) = prev_es { es.close(); }
        store.active_workspace.get().map(|_| connect_sse(&store))
    });
    
    let sidebar_width = create_rw_signal(300.0);
    let graph_panel_width = create_rw_signal(400.0);
    
    view! {
        <TriPaneLayout
            header=ViewFn::from(move || view! {
                <Header title="Ticket Viewer".to_string()>
                    <WorkspacePicker store=store.clone() />
                    <ThemeButton />
                </Header>
            })
            sidebar=ViewFn::from(move || view! {
                <TicketTree store=store.clone() />
            })
            main_content=ViewFn::from(move || view! {
                <div class="ticket-main-split">
                    <TicketContent store=store.clone() />
                    <ResizeHandle direction="horizontal" on_resize=... />
                    <div class="graph-panel" style=move || format!("width:{}px", graph_panel_width.get())>
                        <DependencyGraph store=store.clone() />
                    </div>
                </div>
            })
            sidebar_width=sidebar_width
        />
    }
}
```

### Step 11: URL hash routing

Sync workspace + ticket ID to URL hash for shareable links:

```rust
// In store.rs or a separate url.rs module
fn sync_url_hash(store: &TicketStore) {
    // Write: workspace + openTicketId → #workspace=X&ticket=Y
    create_effect(move |_| {
        let mut hash = String::new();
        if let Some(ws) = store.active_workspace.get() {
            hash.push_str(&format!("workspace={}", ws));
        }
        if let Some(id) = store.open_ticket_id.get() {
            if !hash.is_empty() { hash.push('&'); }
            hash.push_str(&format!("ticket={}", id));
        }
        let _ = web_sys::window().unwrap().location().set_hash(&hash);
    });
    
    // Read: parse hash on load → restore state
    let hash = web_sys::window().unwrap().location().hash().unwrap_or_default();
    // Parse workspace=X&ticket=Y and set signals
}
```

## Files to Create

| File | Purpose |
|------|---------|
| `frontend-leptos/Cargo.toml` | Crate manifest |
| `frontend-leptos/Trunk.toml` | Trunk build config |
| `frontend-leptos/index.html` | WASM entry |
| `frontend-leptos/src/lib.rs` | Module declarations |
| `frontend-leptos/src/app.rs` | Root App with SSE + tri-pane |
| `frontend-leptos/src/api.rs` | All ticket API fetch wrappers |
| `frontend-leptos/src/store.rs` | TicketStore with all signals |
| `frontend-leptos/src/types.rs` | Ticket, Edge, SubgraphResponse, SseEvent types |
| `frontend-leptos/src/actions.rs` | Async actions (load, open, update, create) |
| `frontend-leptos/src/sse.rs` | EventSource SSE consumer |
| `frontend-leptos/src/graph/layout.rs` | Force-directed layout algorithm |
| `frontend-leptos/src/graph/gpu_graph.rs` | WgpuOverlay integration |
| `frontend-leptos/src/graph/svg_graph.rs` | SVG fallback |
| `frontend-leptos/src/components/*.rs` | All 5 components |
| `frontend-leptos/style.css` | Full CSS |

## Files to Modify

| File | Change |
|------|--------|
| `tools/viewer/ticket-viewer/Cargo.toml` | Add optional leptos frontend feature |
| `tools/viewer/ticket-viewer/src/main.rs` | Serve Leptos dist as static fallback |
| Workspace `Cargo.toml` | Add ticket-viewer-leptos to members |

## Acceptance Criteria

1. WebGPU 3D dependency graph via WgpuOverlay (viewer-api-leptos `gpu` feature)
2. SVG fallback renders when WebGPU unavailable — circles with state colors, arrow edges, labels
3. Force-directed layout: depth-based column positioning, centered groups
4. Custom ticket card nodes rendered as HTML overlays on GPU graph
5. SSE live streaming: ticket create/update/delete events update UI in real time
6. EventSource reconnects on disconnect
7. TicketTree: grouped by state, searchable, filterable by state, sortable (title/updated/created)
8. TicketContent: tabbed (Description with markdown, Fields with structured display)
9. Click node in graph → opens ticket detail
10. URL hash routing: `#workspace=X&ticket=Y` — shareable deep links
11. WorkspacePicker dropdown with workspace list
12. Theme system via shared ThemeStore with 3 ticket-specific presets (dark, paper, scratchboard)
13. ResizeHandle between content panel and graph panel
14. trunk build produces working WASM bundle

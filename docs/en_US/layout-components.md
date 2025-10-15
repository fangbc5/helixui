# Layout Components Design (Flex / Layout / Legacy Grid / Grid / Space / Split)

## Goals and Design Principles
- **High Cohesion, Low Coupling**: Each component focuses on a single responsibility, with light coupling through context (such as gutter and breakpoint sharing)
- **Responsive First**: Core dimensions and layout behaviors support breakpoints (sm/md/lg/xl/xxl), SSR-friendly
- **Controlled/Uncontrolled Dual Mode**: Such as `Split` ratio, `Sider` collapse, with callback events provided
- **Semantic and Accessible**: Using semantic tags and ARIA attributes, keyboard accessible
- **Themable**: Relying on unified design tokens to generate styles, supporting dark/size themes
- **Performance Friendly**: Styles are statically concatenated as much as possible; context avoids deep props passing

## Directory Structure
```
helixui-lib/src/components/layout/
  mod.rs
  tokens.rs          # Design tokens for breakpoints/spacing/layers/corners
  context.rs         # Layout/Gutter/Breakpoint context
  flex.rs            # Flex component
  layout.rs          # Layout / Header / Sider / Content / Footer
  legacy_grid.rs     # Legacy grid (compatibility)
  grid.rs            # New grid (modern API)
  space.rs           # Space (spacing)
  split.rs           # Split (draggable split)
  utils.rs           # Breakpoint parsing/style concatenation/controlled bridging
```

## Design Tokens
- **Breakpoints**: `sm=640`, `md=768`, `lg=1024`, `xl=1280`, `xxl=1536`
- **Spacing**: `xs=4`, `sm=8`, `md=12`, `lg=16`, `xl=24`, `xxl=32`
- **Container Max Width**: `sm=640`, `md=768`, `lg=1024`, `xl=1280`, `xxl=1536`
- **Layers**: `affix=1000`, `sider=900`, `header=800`, `split-handle=1100`
- **Border Radius/Borders/Shadows**: Aligned with existing theme system, mapping styles from tokens

## Context Model
- **BreakpointContext**: Current breakpoint, SSR initial breakpoint, breakpoint calculation utilities
- **GutterContext**: `Row` provides `gutter_x/gutter_y` to `Col`
- **LayoutContext**: `has_sider`, `sider_collapsed`, `sider_width/collapsed_width`, `header_height/footer_height`, `is_rtl`

---

## Component 1: Flex (Flexible Layout)
**Purpose**: One-dimensional (horizontal/vertical) distribution, alignment, and spacing control within containers, replacing some simple grid scenarios.

**Core Properties (Draft)**:
- direction: `Row | Column`
- wrap: `bool`
- gap: `Spacing | (x, y)`, supports responsive map
- justify: `Start | Center | End | SpaceBetween | SpaceAround | SpaceEvenly`
- align: `Start | Center | End | Stretch | Baseline`
- inline: `bool` (inline-flex)
- reverse: `bool`

**Key Behaviors**:
- Prioritize native CSS `gap`; fallback to child element margin when not supported (downgrade strategy in utils)
- All alignment semantics consistent with Tailwind/Naive, providing class passthrough

---

## Component 2: Layout Family
Includes: `Layout`, `LayoutHeader`, `LayoutSider`, `LayoutContent`, `LayoutFooter`

**Key Properties (Selected)**:
- Layout: `direction` (horizontal/vertical), `full_height`, `gap`, `padded`, `bordered`, `has_sider` (auto/explicit)
- Header/Footer: `height`, `sticky`, `bordered`, `z_index`
- Sider: `width`, `collapsed_width`, `default_collapsed`, `collapsed(Option)`, `on_collapsed_change`, `collapsible`, `breakpoint`, `resizable`, `min_width/max_width`
- Content: `padded`, `scroll`

**Interaction Design**:
- Sider collapse: Controlled/uncontrolled dual mode; automatically collapses when viewport is below `breakpoint` and restores when returning
- Sider draggable: Change width within `min/max` range; `collapsed_width` has highest priority
- Header sticky: Prioritize `position: sticky`, downgrade to `fixed` in unsupported environments

**Accessibility**:
- Use semantic tags `header/nav/main/footer/aside`; collapse button uses `aria-expanded` and `aria-controls`

---

## Component 3: Legacy Grid (Legacy Grid)
**Goal**: Compatible with historical grid API, smooth migration to new Grid. Default 24 columns.

**Row Properties**: `gutter`(x,y|responsive), `justify`, `align`, `wrap`

**Col Properties**: `span/offset/push/pull` (1..=24), and breakpoint specifications for `sm/md/lg/xl/xxl` (each specification contains the same fields)

**Implementation Points**:
- Pass spacing to child columns through GutterContext; breakpoint priority falls back from large to small
- Only fix compatibility issues, no new complex semantics

---

## Component 4: Grid (New Grid)
**Goal**: Provide more modern API and maintainable implementation, supporting automatic/explicit column width, proportional and fixed mixed layout, built-in responsive.

**Grid Container Properties (Draft)**:
- cols: `Auto | Fixed(u16) | Template(Vec<Track>)` (such as fr/px/minmax)
- gap: `Spacing | (x,y)` (can be responsive)
- auto_rows/auto_cols: Automatic track size (such as minmax)
- align/justify: Container alignment

**GridItem Properties**:
- col: `RangeInclusive<u16>` or `Span(u16)`
- row: Same as above
- order: `i32`

**Implementation Points**:
- Based on CSS Grid; provide minimal downgrade in environments that don't support CSS Grid (optional)
- Coexist with Legacy Grid, guide migration through codemod/documentation later

---

## Component 5: Space (Spacing)
**Purpose**: Control spacing, alignment, and wrapping between a group of child elements, supporting separators.

**Properties**:
- direction: `Horizontal | Vertical`
- size: `Spacing | (x,y)` | responsive map
- wrap: `bool`
- align: Optional alignment
- split: Optional separator vnode

**Implementation Points**:
- Prioritize CSS `gap`; fallback to margin when not supported
- Insert `split` as pseudo-element or insert between child nodes (choose based on rendering limitations)

---

## Component 6: Split (Panel Split)
**Purpose**: Split areas proportionally in horizontal/vertical directions, supporting drag adjustment.

**Properties**:
- direction: `Horizontal | Vertical`
- initial: `f32` (0.0..=1.0)
- min_a/min_b: Minimum size
- resizable: `bool`
- on_change: Callback (throttled during drag); on_commit (on release) optional

**Implementation Points**:
- Use absolute positioning handles; support mouse and touch events; consider iframe/text selection boundaries
- Use tokens for z-index; handles can be custom nodes
- Controlled/uncontrolled dual mode; SSR initial value consistent with hydration

**Accessibility**:
- Handles provide `aria-valuenow/min/max`, support keyboard arrows and PageUp/Down for ratio adjustment

---

## Accessibility and Internationalization
- **RTL Support**: `is_rtl` provided by LayoutContext; reverse in push/pull, alignment, and drag directions
- **Semantic Tags**: Prioritize `header/main/footer/aside/nav` etc.; supplement with `role` when necessary

## Engineering and Performance
- **Style Caching**: Generate keys based on props, avoid duplicate calculations
- **Event Throttling**: Use throttling/debouncing for Split drag, window resize
- **Class Concatenation**: Static as much as possible, control dynamic parts in few branches

## Implementation Roadmap
1. Build `tokens.rs`, `context.rs` and utility functions
2. Implement Flex and Space as basic capabilities (gap/alignment/wrapping)
3. Implement Legacy Grid, ensure compatibility; then implement new Grid
4. Implement Layout family and Sider collapse/breakpoint/draggable capabilities
5. Implement Split (drag and keyboard interaction)
6. Complete documentation and example pages, provide migration guides and best practices

## Example (Pseudo-code snippet)
```rust
// Common page layout
rsx! {
    Layout { full_height: true,
        LayoutSider { collapsible: true, breakpoint: Some(Breakpoint::md) }
        Layout { direction: LayoutDirection::Vertical,
            LayoutHeader { sticky: true }
            LayoutContent { padded: true,
                Space { direction: Axis::Vertical, size: Spacing::Lg, wrap: false }
            }
            LayoutFooter {}
        }
    }
}
```

This design document will serve as the basis for implementation and review. Specific APIs may be fine-tuned during development, but the overall philosophy and contracts remain unchanged.

---

## SSR Fallback Logic
**Scenario**: During SSR rendering, real viewport and container dimensions are unknown, involving breakpoint judgment (Layout/Sider/Row/Col), `position: sticky/fixed` behavior, Split initial ratio, etc.

**Strategy**:
- Provide `initial_breakpoint` for `BreakpointContext`, injected by server during SSR (configurable as `md` or estimated based on UA)
- After client hydration, read real dimensions and recalculate breakpoints; if different from SSR value, perform one-time non-animated style sync (avoid flickering)
- Sider collapse: If `breakpoint` takes effect causing state change, update after hydration with "controlled priority" principle; for non-controlled mode, internal state overrides SSR initial value with real breakpoint
- Sticky/Header: Render as static positioning during SSR; switch to `sticky` or `fixed` after hydration, and cancel transitions during switch frame
- Split: Use `initial` ratio for static layout during SSR; bind events after hydration, align with external value if controlled

**Implementation Points**:
- Provide `ssr_fallback_class(strategy)` in `utils`: can insert placeholder classes, remove after hydration
- For components requiring dimension participation in calculation (such as Grid template columns), prioritize CSS functions (fr/minmax), reduce JS participation and minimize SSR differences

## Controlled Generic Encapsulation
To unify controlled/uncontrolled dual mode, provide universal bridging:

```rust
// Pseudo-code: Unified management of controlled/uncontrolled values and callbacks
pub struct Controlled<T> {
    pub value: Option<T>,              // Controlled value
    pub default_value: Option<T>,      // Uncontrolled initial value
    pub on_change: Option<EventHandler<T>>,
}

impl<T: Clone + PartialEq> Controlled<T> {
    pub fn resolve(&self, inner_state: &T) -> T {
        self.value.clone().unwrap_or_else(|| inner_state.clone())
    }

    pub fn set_and_emit(&self, next: T, set_inner: impl FnOnce(T)) {
        if self.value.is_some() {
            if let Some(cb) = &self.on_change { cb.call(next); }
        } else {
            set_inner(next.clone());
            if let Some(cb) = &self.on_change { cb.call(next); }
        }
    }
}
```

**Application**:
- `Sider.collapsed`, `Split.ratio`, `GridItem.order` etc. all implement consistent behavior through `Controlled<T>`

## Basic Hooks

### use_drag (Drag)
**Responsibility**: Unify mouse/touch drag events, provide throttling and boundary control; reuse in Split, resizable Sider, etc.

**Interface (Draft)**:
- Input: `on_start`, `on_move(DragState)`, `on_end`, `axis: Horizontal|Vertical|Both`, `bounds`, `capture_pointer`, `throttle_ms`
- Output: `bind` (event attribute collection to bind to handle), `is_dragging` state

**Key Implementation Points**:
- Handle Pointer/Mouse/Touch three types of events; use transparent overlay layer to prevent misselection in text selection, iframe scenarios
- Support `prevent_default` and `stop_propagation` switches; support RTL direction correction

### use_resize_observer (Size Monitoring)
**Responsibility**: Monitor element size changes, drive breakpoints and adaptive layout (such as Container, Grid automatic tracks)

**Interface (Draft)**:
- Input: `node_ref`, `on_resize(ResizeEntry)`, `debounce_ms`
- Output: `content_rect` (latest size), `disconnect()`

**Key Implementation Points**:
- Native `ResizeObserver` priority; fallback to `window.resize + getBoundingClientRect` when not supported, with debouncing
- Collaborate with `BreakpointContext`: when `Container` is specified as breakpoint reference container, calculate breakpoints based on its width rather than global viewport

## tokens → theme config abstraction
**Goal**: Abstract static tokens into configurable themes, supporting multiple themes and dynamic switching

**Layers**:
1. tokens (design value collection): breakpoints, spacing, radius, shadows, color palette reference keys, etc.
2. theme config (runtime theme): based on tokens, combined with brand colors/dark themes to generate final style parameters
3. theme provider: inject `ThemeContext` into application tree, components only depend on provider output, not directly on static constants

**Interface (Draft)**:
- `ThemeTokens`: Pure data structure
- `ThemeConfig`: Derived from tokens, provides methods like `spacing(scale) -> px`, `z_index(key) -> i32`, `breakpoint(name) -> px`
- `ThemeProvider::new(config)`: Inject into component tree; `use_theme()` get reference
- Theme switching: `set_theme(config)` or drive provider updates through outer state; components perform minimal re-rendering based on `PartialEq` differences

**Implementation Suggestions**:
- First implement read-only theme (single theme); then extend to switchable themes and support on-demand merging (such as `ThemeConfig::extend`)
- Uniformly get values from `use_theme()` at component style generation, avoid direct dependency on constants

---

## Development Progress Status

### ✅ Completed Components

#### 1. Flex (Flexible Layout)
- **Status**: ✅ Complete
- **Features**: Supports direction, wrap, gap, justify, align and other properties
- **Highlights**: Uses Tailwind CSS styling, supports responsive gap calculation
- **Documentation**: Complete demo pages including main axis alignment, cross axis alignment, wrapping examples

#### 2. Space (Spacing)
- **Status**: ✅ Complete
- **Features**: Supports horizontal/vertical spacing, separators, responsive sizing
- **Highlights**: Based on Flex implementation, uses CSS `::after` pseudo-elements for separators
- **Documentation**: Complete demo pages including basic usage, separators, wrapping examples

#### 3. Layout Family
- **Status**: ✅ Complete
- **Components**: Layout, Header, Content, Footer, Sider, SiderTrigger
- **Features**: Page layout, fixed header/footer, sidebar collapse, theme support
- **Highlights**: Uses Tailwind CSS styling, supports light/dark theme switching
- **Documentation**: Complete demo pages including basic layout, sidebar, fixed positioning examples

### ✅ Completed Infrastructure

#### 1. Design Token System
- **tokens.rs**: Breakpoint, spacing, layer definitions
- **theme.rs**: Theme configuration and context provision
- **context.rs**: Layout context management

#### 2. Utility Functions
- **utils.rs**: Gap calculation, responsive size handling
- **hooks/**: use_breakpoint, use_drag, use_resize_observer (basic framework)

#### 3. Website Integration
- **Component Pages**: Added layout component categories in components_page.rs
- **Sidebar Navigation**: Added layout component navigation in sidebar.rs
- **Internationalization**: Added layout component translations in i18n.rs
- **Route Configuration**: Added LayoutPage and SpacePage routes in main.rs

### 🔄 In Progress Tasks
- **None**: All planned tasks are currently completed

### ⏳ Components to Develop

#### 1. Legacy Grid (Legacy Grid)
- **Status**: ⏳ To be developed
- **Goal**: Compatible with historical grid API, 24-column system
- **Priority**: Medium

#### 2. Grid (New Grid)
- **Status**: ⏳ To be developed
- **Goal**: Modern CSS Grid API, supports automatic/explicit column width
- **Priority**: Medium

#### 3. Split (Panel Split)
- **Status**: ⏳ To be developed
- **Goal**: Draggable split panels, supports keyboard operation
- **Priority**: Low

### 📊 Completion Statistics
- **Completed Components**: 3/6 (50%)
- **Completed Infrastructure**: 100%
- **Website Integration**: 100%
- **Documentation Demos**: 100%

### 🎯 Next Steps
1. Decide whether to continue developing remaining components based on user needs
2. Optimize performance and accessibility of existing components
3. Improve dynamic switching functionality of theme system
4. Add more usage examples and best practices for layout components

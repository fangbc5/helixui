# Feedback Components Design (Badge / Message / Dialog / Modal / Notification / Alert)

## Goals and Design Principles

- **High Cohesion, Low Coupling**: Each component focuses on a single responsibility, maintaining visual consistency through unified theme system
- **User Experience First**: Provide intuitive feedback mechanisms, support multiple interaction methods
- **Accessibility**: Follow ARIA standards, support keyboard navigation and screen readers
- **Themable**: Support light/dark theme switching, maintain consistency with overall design system
- **Performance Optimization**: Reasonable use of animations and transitions, avoid performance issues

## Directory Structure

```
helixui-lib/src/components/feedback/
  mod.rs
  badge.rs          # Badge component
  message.rs        # Message component
  dialog.rs         # Dialog component
  modal.rs          # Modal component
  notification.rs   # Notification (to be implemented)
  alert.rs          # Alert (to be implemented)

helixui-lib/src/overlay/
  mod.rs
  config/           # Configuration system
    overlay_config.rs       # Main configuration
    z_index_config.rs       # Z-Index configuration
    responsive_config.rs    # Responsive configuration
    simple_types.rs         # Simple type definitions
  core/             # Core layer
    state_manager.rs        # State management
    event_system.rs         # Event system
    animation_manager.rs    # Animation management
    theme_manager.rs        # Theme management
    platform_adapter.rs     # Platform adaptation
  components/       # Component layer
    message_overlay.rs      # Message overlay
    dialog_overlay.rs       # Dialog overlay
    modal_overlay.rs        # Modal overlay
    notice_overlay.rs       # Notice overlay
    interactive_overlay.rs  # Interactive overlay
  plugins/          # Plugin system
    plugin_manager.rs       # Plugin manager
    animation_plugin.rs     # Animation plugin
    theme_plugin.rs         # Theme plugin
    accessibility_plugin.rs # Accessibility plugin
  utils/            # Utility modules
    error_handler.rs        # Error handling
    helpers.rs              # Helper functions
```

## Component Design

### 1. Badge

**Purpose**: Used to display status markers or quantity information, usually attached to other components.

**Core Properties**:

- `value`: `Option<String>` - Display value
- `type`: `BadgeType` - Type (Default, Success, Warning, Error, Info)
- `shape`: `BadgeShape` - Shape (Dot, Round, Square)
- `size`: `BadgeSize` - Size (Small, Medium, Large)
- `max`: `Option<u32>` - Maximum value, display "max+" when exceeded
- `processing`: `bool` - Whether to show processing state
- `offset`: `Option<(i32, i32)>` - Offset (x, y)

**Implementation Points**:

- Use absolute positioning to attach to parent element
- Support number badges and dot badges
- Processing state uses animation effects
- Support custom styles and positions

### 2. Message

**Purpose**: Lightweight global prompt feedback, used to display operation results or important information.

**Core Properties**:

- `type`: `MessageType` - Type (Info, Success, Warning, Error)
- `content`: `String` - Message content
- `duration`: `Option<u32>` - Display duration (milliseconds), None means no auto-close
- `closable`: `bool` - Whether to show close button
- `icon`: `Option<Element>` - Custom icon
- `on_close`: `Option<EventHandler<()>>` - Close callback

**Implementation Points**:

- Global message manager, supports queue management
- Automatically calculate positions, avoid overlap
- Support manual close and auto-close
- Provide convenient global methods (`show_message`, `close_message`)

### 3. Dialog

**Purpose**: Modal dialog, inform users and carry related operations while retaining current page state.

**Core Properties**:

- `visible`: `bool` - Whether to show
- `title`: `Option<String>` - Title
- `content`: `Element` - Content
- `footer`: `Option<Element>` - Footer content
- `width`: `Option<String>` - Width
- `closable`: `bool` - Whether to show close button
- `mask_closable`: `bool` - Whether to close when clicking mask
- `on_close`: `Option<EventHandler<()>>` - Close callback

**Implementation Points**:

- Use Portal to render to body
- Support ESC key to close
- Automatically manage focus and scroll lock
- Support custom animation effects

### 4. Modal

**Purpose**: Modal dialog, used to display important information or collect user input.

**Core Properties**:

- `visible`: `bool` - Whether to show
- `title`: `Option<String>` - Title
- `content`: `Element` - Content
- `footer`: `Option<Element>` - Footer content
- `width`: `Option<String>` - Width
- `centered`: `bool` - Whether to center vertically
- `closable`: `bool` - Whether to show close button
- `mask_closable`: `bool` - Whether to close when clicking mask
- `on_ok`: `Option<EventHandler<()>>` - OK callback
- `on_cancel`: `Option<EventHandler<()>>` - Cancel callback

**Implementation Points**:

- Extended based on Dialog, add OK/Cancel buttons
- Support form validation and submission
- Provide convenient methods for confirmation dialogs
- Support custom button text and behavior

### 5. Notification

**Purpose**: Display notification reminder messages, usually appear in page corners.

**Core Properties**:

- `type`: `NotificationType` - Type (Info, Success, Warning, Error)
- `title`: `Option<String>` - Title
- `content`: `String` - Content
- `duration`: `Option<u32>` - Display duration
- `closable`: `bool` - Whether to show close button
- `placement`: `NotificationPlacement` - Position (TopLeft, TopRight, BottomLeft, BottomRight)
- `on_close`: `Option<EventHandler<()>>` - Close callback

**Implementation Points**:

- Global notification manager
- Support multiple notifications displayed simultaneously
- Automatically calculate positions and animations
- Provide convenient global methods

### 6. Alert

**Purpose**: Static information prompts, used for important information display on pages.

**Core Properties**:

- `type`: `AlertType` - Type (Info, Success, Warning, Error)
- `title`: `Option<String>` - Title
- `content`: `Element` - Content
- `closable`: `bool` - Whether to show close button
- `show_icon`: `bool` - Whether to show icon
- `banner`: `bool` - Whether it's banner style
- `on_close`: `Option<EventHandler<()>>` - Close callback

**Implementation Points**:

- Inline component, doesn't occupy additional levels
- Support multiple style variants
- Two modes: closable and non-closable
- Naturally integrate with page content

## Theme System Integration

### Color System

```rust
pub enum FeedbackColor {
    Info,      // Information - Blue series
    Success,   // Success - Green series
    Warning,   // Warning - Orange series
    Error,     // Error - Red series
    Default,   // Default - Gray series
}
```

### Size System

```rust
pub enum FeedbackSize {
    Small,     // Small size
    Medium,    // Medium size
    Large,     // Large size
}
```

### Animation System

- **Fade In/Out**: For message and notification show/hide
- **Slide**: For dialog enter/exit
- **Scale**: For modal open/close
- **Pulse**: For processing state badges

## Accessibility Design

### Keyboard Navigation

- **Tab Key**: Switch focus between interactive elements
- **Enter/Space**: Activate buttons and links
- **ESC Key**: Close dialogs and modals
- **Arrow Keys**: Navigate in notification lists

### ARIA Attributes

- `role="alert"` - For important messages
- `role="dialog"` - For dialogs
- `aria-labelledby` - Associate with title
- `aria-describedby` - Associate with description
- `aria-expanded` - Indicate expanded state
- `aria-hidden` - Hide decorative elements

### Screen Reader Support

- Provide meaningful text descriptions
- Use `aria-live` regions to broadcast dynamic content
- Ensure correct focus management
- Provide clear indication of close operations

## Performance Optimization

### Rendering Optimization

- Use `Portal` to avoid unnecessary re-renders
- Implement virtualization for large numbers of messages/notifications
- Reasonable use of `memo` and `use_memo`
- Avoid complex calculations during rendering

### Animation Optimization

- Use CSS animations instead of JavaScript animations
- Reasonable animation duration and easing functions
- Support `prefers-reduced-motion` media query
- Avoid triggering multiple animations simultaneously

### Memory Management

- Clean up event listeners in time
- Reasonable global state management
- Avoid memory leaks
- Provide cleanup methods

## Usage Examples

### Badge Usage

```rust
rsx! {
    div {
        Button { "Messages" }
        Badge {
            value: Some("5".to_string()),
            type: BadgeType::Error,
            shape: BadgeShape::Round,
        }
    }
}
```

### Message Usage

```rust
// Show message
show_message(MessageType::Success, "Operation successful!".to_string());

// Custom message
rsx! {
    Message {
        type: MessageType::Info,
        content: "This is an information message".to_string(),
        duration: Some(3000),
        closable: true,
    }
}
```

### Dialog Usage

```rust
rsx! {
    Dialog {
        visible: dialog_visible,
        title: Some("Confirm Delete".to_string()),
        content: rsx! { "Are you sure you want to delete this item?" },
        footer: rsx! {
            Button { onclick: move |_| set_dialog_visible(false), "Cancel" }
            Button { onclick: move |_| handle_delete(), "Confirm" }
        },
        on_close: move |_| set_dialog_visible(false),
    }
}
```

### Modal Usage

```rust
rsx! {
    Modal {
        visible: modal_visible,
        title: Some("Edit User".to_string()),
        content: rsx! { /* Form content */ },
        on_ok: move |_| handle_save(),
        on_cancel: move |_| set_modal_visible(false),
    }
}
```

## Overlay System Architecture

### Design Philosophy

The Overlay system is a universal overlay management system that provides unified underlying support for all feedback components. It adopts a **layered architecture** design, separating configuration, core functionality, component implementation, and plugin extensions, achieving high modularity and extensibility.

### Core Features

1. **Unified State Management**: Global state manager that tracks the lifecycle of all overlays
2. **Smart Z-Index Management**: Automatic Z-Index allocation and management to avoid layer conflicts
3. **Event-Driven Architecture**: Event bus-based communication mechanism with loose coupling between components
4. **Animation System**: Supports multiple animation types and easing functions, configurable and extensible
5. **Theme Integration**: Deep integration with the global theme system, automatically responds to theme changes
6. **Platform Adaptation**: Cross-platform support (Web, Desktop, Mobile), automatic platform capability detection
7. **Plugin Mechanism**: Supports dynamic registration and extension of animation, theme, accessibility plugins
8. **Responsive Design**: Supports breakpoint configuration, adapts to different screen sizes

### Architecture Layers

#### 1. Configuration Layer

Responsible for providing various configuration items and default values:

**OverlayConfig**: Main configuration class

```rust
pub struct OverlayConfig {
    pub z_index: ZIndexConfig,       // Z-Index configuration
    pub animation: AnimationConfig,  // Animation configuration
    pub theme: ThemeConfig,          // Theme configuration
    pub responsive: ResponsiveConfig,// Responsive configuration
    pub platform: PlatformConfig,    // Platform configuration
}
```

**ZIndexConfig**: Manages layer order

- Base level: 1000
- Auto increment per overlay: +10
- Supports custom level range

**AnimationConfig**: Animation configuration

```rust
pub struct AnimationConfig {
    pub duration: u32,            // Duration (milliseconds)
    pub delay: u32,               // Delay time
    pub easing: EasingType,       // Easing type
    pub enter: AnimationType,     // Enter animation
    pub exit: AnimationType,      // Exit animation
    pub fill_mode: FillMode,      // Fill mode
    pub iteration_count: u32,     // Iteration count
}
```

**ResponsiveConfig**: Responsive configuration

- Supports multiple breakpoint configurations
- Layout strategies for different screen sizes
- Mobile optimization support

#### 2. Core Layer

Provides core functionality of the Overlay system:

**GlobalStateManager**: Global state management

```rust
pub struct GlobalStateManager {
    overlays: HashMap<String, OverlayState>,  // All overlay states
    z_index_stack: Vec<u32>,                  // Z-Index stack
    next_z_index: u32,                        // Next available Z-Index
}
```

Features:

- `add_overlay()`: Add new overlay
- `remove_overlay()`: Remove overlay
- `show_overlay()` / `hide_overlay()`: Show/hide overlay
- `get_visible_overlays()`: Get all visible overlays
- `get_overlays_by_type()`: Filter overlays by type
- `clear_all()`: Clear all overlays

**EventBus**: Event bus

```rust
pub enum EventType {
    OverlayShow,      // Overlay show
    OverlayHide,      // Overlay hide
    OverlayClose,     // Overlay close
    ZIndexChange,     // Z-Index change
    PlatformChange,   // Platform change
    AnimationStart,   // Animation start
    AnimationEnd,     // Animation end
    ThemeChange,      // Theme change
}
```

Features:

- Register/remove event listeners
- Emit events to subscribers
- Support event filtering and priority

**AnimationManager**: Animation manager

```rust
pub enum AnimationType {
    FadeIn / FadeOut,       // Fade in/out
    SlideIn / SlideOut,     // Slide in/out
    ScaleIn / ScaleOut,     // Scale in/out
    BounceIn / BounceOut,   // Bounce in/out
    Custom(String),         // Custom animation
}

pub enum EasingType {
    Linear, EaseIn, EaseOut, EaseInOut,
    EaseInQuad, EaseOutQuad, EaseInOutQuad,
    EaseInCubic, EaseOutCubic, EaseInOutCubic,
    Custom(String),
}
```

Features:

- Generate animation class names
- Detect platform animation support
- Create enter/exit animations
- Support custom easing functions

**ThemeManager**: Theme manager

- Manage light/dark theme switching
- Provide component style classes
- Integrate with global theme system
- Support custom themes

**PlatformAdapter**: Platform adapter

```rust
pub struct PlatformCapabilities {
    pub supports_gestures: bool,        // Supports gestures
    pub supports_haptic_feedback: bool, // Supports haptic feedback
    pub supports_animations: bool,      // Supports animations
    pub supports_touch: bool,           // Supports touch
    pub max_z_index: u32,              // Maximum Z-Index
    pub animation_duration: u32,       // Animation duration
}
```

Features:

- Auto-detect platform type (Web/Desktop/Mobile)
- Provide platform capability queries
- Adapt to different platform behaviors

#### 3. Component Layer

Specific Overlay components implemented based on the core layer:

**MessageOverlay**: Message overlay

- Lightweight message prompts
- Auto-positioning and animation
- Support multiple message types
- Auto-close and manual close

**DialogOverlay**: Dialog overlay

- Modal dialog implementation
- Mask layer support
- Focus management
- ESC key close

**ModalOverlay**: Modal overlay

- Extended based on DialogOverlay
- Support OK/Cancel operations
- Form integration
- Validation support

**NoticeOverlay**: Notice overlay

- Corner notifications
- Support stacking display
- Custom positioning
- Multiple simultaneous displays

**InteractiveOverlay**: Interactive overlay

- Popover/Tooltip foundation
- Follow target elements
- Auto-position calculation
- Collision detection

#### 4. Plugin Layer

Provides extensible plugin mechanism:

**PluginManager**: Plugin manager

```rust
pub trait OverlayPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self, context: &OverlayContext) -> Result<(), String>;
    fn cleanup(&mut self) -> Result<(), String>;
    fn handle_event(&mut self, event: &str, data: &HashMap<String, String>) -> Result<(), String>;
}
```

**AnimationPlugin**: Animation plugin

- Extend animation types
- Custom animation effects
- Animation queue management

**ThemePlugin**: Theme plugin

- Custom themes
- Dynamic theme switching
- Theme presets

**AccessibilityPlugin**: Accessibility plugin

- Auto-add ARIA attributes
- Enhanced focus management
- Screen reader optimization
- Keyboard navigation support

### Usage Examples

#### Basic Usage

```rust
use helixui::overlay::*;

// Show message
show_message("Operation successful".to_string(), MessageType::Success);

// Show message with duration
show_message_with_duration("Saved successfully".to_string(), MessageType::Success, 3000);

// Show message at specific position
show_message_with_position(
    "Please note".to_string(),
    MessageType::Warning,
    MessagePosition::TopCenter
);
```

#### Advanced Configuration

```rust
use helixui::overlay::{config::*, core::*};

// Create custom configuration
let config = OverlayConfig::new()
    .with_animation(AnimationConfig {
        duration: 300,
        easing: EasingType::EaseInOut,
        enter: AnimationType::SlideIn,
        exit: AnimationType::FadeOut,
        ..Default::default()
    })
    .with_z_index(ZIndexConfig {
        base: 2000,
        increment: 20,
        max: 9999,
    });

// Use configuration to create overlay
// ...
```

#### Plugin Extension

```rust
use helixui::overlay::plugins::*;

// Create plugin manager
let mut plugin_manager = PluginManager::new();

// Register plugins
plugin_manager.register_plugin(Box::new(AnimationPlugin::new()))?;
plugin_manager.register_plugin(Box::new(ThemePlugin::new()))?;
plugin_manager.register_plugin(Box::new(AccessibilityPlugin::new()))?;

// Initialize all plugins
plugin_manager.initialize_all()?;
```

### Technical Advantages

1. **Layered Architecture**: Clear separation of responsibilities, easy to maintain and extend
2. **Centralized State Management**: Avoid inconsistencies from scattered state
3. **Event-Driven**: Components communicate through events, reducing coupling
4. **Platform Agnostic**: Unified API, consistent behavior across platforms
5. **Performance Optimization**:
   - Smart Z-Index allocation, avoiding unnecessary reflows
   - CSS-based animations with hardware acceleration
   - Lazy loading and on-demand rendering
6. **Testability**: Modular design facilitates unit testing
7. **Extensibility**: Plugin mechanism supports feature extensions

### Integration with Feedback Components

The Overlay system provides underlying support for all feedback components:

- **Message**: Uses `MessageOverlay` + `GlobalStateManager`
- **Dialog**: Uses `DialogOverlay` + `AnimationManager`
- **Modal**: Uses `ModalOverlay` + `ThemeManager`
- **Notification**: Uses `NoticeOverlay` + `EventBus`
- **Toast**: Lightweight variant of `MessageOverlay`

All components share:

- Unified Z-Index management
- Consistent animation effects
- Same theme system
- Unified event handling

## Development Progress Status

### ✅ Completed Components

- **Badge**: ✅ Complete, supports multiple types and shapes
- **Message**: ✅ Complete, supports global message management, integrated with Overlay system
- **Dialog**: ✅ Complete, supports custom content and operations
- **Modal**: ✅ Complete, extended based on Dialog
- **Overlay System**: ✅ Complete, provides full overlay management architecture

### ⏳ Components to Develop

- **Notification**: ⏳ To be developed (Overlay foundation ready)
- **Alert**: ⏳ To be developed

### 📊 Completion Statistics

- **Completed Components**: 4/6 (67%)
- **Overlay System**: 100% (Complete architecture)
- **Documentation Demos**: 100%
- **Accessibility**: Basic support + plugin enhancement
- **Theme Integration**: 100%
- **Cross-Platform Support**: 100%

## Best Practices

### Usage Recommendations

1. **Badge**: For status indication and quantity reminders, avoid overuse
2. **Message**: For operation feedback, keep concise and clear
3. **Dialog**: For important confirmations, avoid frequent popups
4. **Modal**: For complex forms and important operations
5. **Notification**: For system notifications, control quantity
6. **Alert**: For important page information, maintain visibility

### Design Principles

1. **Consistency**: Maintain visual and interaction consistency
2. **Hierarchy**: Reasonable use of different levels of feedback
3. **Timeliness**: Provide timely feedback, avoid user confusion
4. **Operability**: Provide clear subsequent operation guidance
5. **Closability**: Allow users to close non-critical information

---

This design document will serve as the basis for feedback component implementation and review. Specific APIs may be fine-tuned during development, but the overall philosophy and contracts remain unchanged.

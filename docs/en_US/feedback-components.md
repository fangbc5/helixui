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

## Development Progress Status

### ✅ Completed Components
- **Badge**: ✅ Complete, supports multiple types and shapes
- **Message**: ✅ Complete, supports global message management
- **Dialog**: ✅ Complete, supports custom content and operations
- **Modal**: ✅ Complete, extended based on Dialog

### ⏳ Components to Develop
- **Notification**: ⏳ To be developed
- **Alert**: ⏳ To be developed

### 📊 Completion Statistics
- **Completed Components**: 4/6 (67%)
- **Documentation Demos**: 100%
- **Accessibility**: Basic support
- **Theme Integration**: 100%

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

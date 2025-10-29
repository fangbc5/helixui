# Common Components (Dropdown / Collapsible / Button / Icon / Divider)

> This document defines the goals, API conventions, accessibility and styling guidelines for common components. It focuses on the shipped Dropdown and Collapsible, and serves as a template for future components.

## Goals & Principles

- Consistent API naming: trigger, content, disabled, controlled `open`/`on_open_change`
- Composable and nestable: arbitrary content inside dropdown content
- Accessible by default: keyboard navigation, focus ring, ARIA
- Themeable: light/dark, borders/shadows, rounded corners, hover/focus states

---

## Dropdown

### Use cases

- Action menu (Edit/Delete/Share)
- Foundation for selection family (Select/Combobox/Cascader)
- User menu, context menu, filter panel, and other overlay shells

### Composition & Semantics

- `Dropdown`: root container managing open/close and roving focus
- `DropdownTrigger`: trigger element (button-like), open with Enter/Space
- `DropdownContent`: floating container (listbox/menu semantics)
- `DropdownItem<T>`: item, supports `disabled` and `on_select(value)`

### Minimal example

```rust
rsx! {
    Dropdown { default_open: false,
        DropdownTrigger { "Open Menu" }
        DropdownContent {
            DropdownItem::<&'static str> { value: "edit",   index: 0usize, "Edit" }
            DropdownItem::<&'static str> { value: "delete", index: 1usize, "Delete" }
            DropdownItem::<&'static str> { value: "share",  index: 2usize, "Share" }
        }
    }
}
```

### Key Props

- `Dropdown`:
  - `open: Option<bool>` (controlled) / `default_open: bool` (uncontrolled)
  - `on_open_change: Option<EventHandler<bool>>`
  - `disabled: Signal<bool>`
  - `roving_loop: bool`
- `DropdownTrigger`: no strict children requirement (button role & focusable)
- `DropdownContent`: exposes `data-state=open|closed` for styling transitions
- `DropdownItem<T>`: `value`, `index`, `disabled`, `on_select`

### Styling & Positioning

- Tailwind visual defaults:
  - Trigger: bordered button, hover/focus ring
  - Content: card-like white background, shadow, rounded, enter/exit transitions
  - Item: hover/focus highlight, disabled gray
- Positioning (CSS only, no JS):
  - `Dropdown` root: `relative inline-block`
  - `DropdownContent`: `absolute top-full left-0 mt-1`
  - If an ancestor clips with `overflow: hidden`:
    - Prefer placing dropdown under a non-clipping container
    - Or use a Portal in future to render to the document root

### Accessibility

- `role="listbox"` with `aria-labelledby` bound to trigger
- Keyboard: ArrowUp/ArrowDown navigate, Enter select, Esc close
- Focus: transfer to item list when open, return to trigger on close

### Extensions

- Submenu, Group/Label, Checkbox/Radio items
- ContextMenu, Select/Combobox, FilterDropdown

---

## Collapsible

### Use cases

- FAQ/help sections
- Advanced options in forms

### Minimal example

```rust
rsx! {
    Collapsible { default_open: true,
        Collapsible::Trigger { "Toggle" }
        Collapsible::Content { "Content" }
    }
}
```

### Notes

- Controlled/uncontrolled; transition driven by `data-state`
- Shares design language with Dropdown (trigger/content/animation)

---

## Location in Repo

```
helixui-lib/src/components/common/
  dropdown.rs
  collapsible.rs
  mod.rs
```

---

## Roadmap (Common)

- Select / Combobox / Cascader (built on Dropdown)
- ContextMenu (right-click, pointer coordinate trigger)
- UserMenu (avatar dropdown)
- FilterDropdown (form filter panel)

> For new component docs, reuse this structure: Goals → API → Example → A11y → Styling/Positioning → Extensions.

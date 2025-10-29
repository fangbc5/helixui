//! Defines the [`Dropdown`] component and its subcomponents.

use std::rc::Rc;

use crate::components::focus::{use_focus_controlled_item, use_focus_provider, FocusState};
use crate::components::{use_animated_open, use_controlled, use_id_or, use_unique_id};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct DropdownContext {
    // State
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,

    // Focus state
    focus: FocusState,

    // Unique ID for the trigger button
    trigger_id: Signal<String>,
}

/// The props for the [`Dropdown`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    /// Whether the dropdown menu is open. If not provided, the component will be uncontrolled and use `default_open`.
    pub open: ReadSignal<Option<bool>>,

    /// Default open state if the component is not controlled.
    #[props(default)]
    pub default_open: bool,

    /// Callback when the open state changes. This is called when the dropdown menu is opened or closed.
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the dropdown menu is disabled. If true, the menu will not open and items will not be selectable.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// Additional attributes to apply to the dropdown menu element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the dropdown menu, which should include a [`DropdownTrigger`] and a [`DropdownContent`].
    pub children: Element,
}

/// # Dropdown
///
/// The `Dropdown` component is a container for a [`DropdownContent`] component activated by a [`DropdownTrigger`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     Dropdown, DropdownContent, DropdownItem, DropdownTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Dropdown { default_open: false,
///             DropdownTrigger { "Open Menu" }
///             DropdownContent {
///                 DropdownItem::<String> {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownItem::<String> {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Dropdown`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the dropdown menu. values are `open` or `closed`.
/// - `data-disabled`: Indicates if the dropdown menu is disabled. values are `true` or `false`.
#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);

    let disabled = props.disabled;
    let trigger_id = use_unique_id();
    let focus = use_focus_provider(props.roving_loop);
    let mut ctx = use_context_provider(|| DropdownContext {
        open,
        set_open,
        disabled,
        focus,
        trigger_id,
    });

    use_effect(move || {
        let focused = focus.any_focused();
        if *ctx.open.peek() != focused {
            (ctx.set_open)(focused);
        }
    });

    // Handle escape key to close the menu
    let handle_keydown = move |event: Event<KeyboardData>| {
        if disabled() {
            return;
        }
        match event.key() {
            Key::Enter => {
                let new_open = !(ctx.open)();
                ctx.set_open.call(new_open);
            }
            Key::Escape => ctx.set_open.call(false),
            Key::ArrowDown => {
                ctx.focus.focus_next();
            }
            Key::ArrowUp => {
                if open() {
                    ctx.focus.focus_prev();
                }
            }
            Key::Home => ctx.focus.focus_first(),
            Key::End => ctx.focus.focus_last(),
            _ => return,
        }
        event.prevent_default();
    };

    rsx! {
        div {
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            // .dropdown-menu
            class: "relative inline-block",
            onkeydown: handle_keydown,
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownTriggerProps {
    /// Additional attributes to apply to the trigger button element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the trigger button
    pub children: Element,
}

/// # DropdownTrigger
///
/// The trigger button for the parent [`Dropdown`]. This button toggles the visibility of the [`DropdownContent`].
///
/// This must be used inside a [`Dropdown`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     Dropdown, DropdownContent, DropdownItem, DropdownTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Dropdown { default_open: false,
///             DropdownTrigger { "Open Menu" }
///             DropdownContent {
///                 DropdownItem::<String> {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownItem::<String> {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownTrigger`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the dropdown menu. values are `open` or `closed`.
/// - `data-disabled`: Indicates if the dropdown menu is disabled. values are `true` or `false`.
#[component]
pub fn DropdownTrigger(props: DropdownTriggerProps) -> Element {
    let mut ctx: DropdownContext = use_context();
    let mut element = use_signal(|| None::<Rc<MountedData>>);

    rsx! {
        button {
            id: "{ctx.trigger_id}",
            type: "button",
            "data-state": if (ctx.open)() { "open" } else { "closed" },
            "data-disabled": (ctx.disabled)(),
            disabled: (ctx.disabled)(),
            aria_expanded: ctx.open,
            aria_haspopup: "listbox",

            // .dropdown-menu-trigger
            class: "px-4 py-2 rounded-lg cursor-pointer text-base \
                    bg-white border-2 border-blue-600 text-gray-900 \
                    transition-all duration-200 \
                    hover:bg-gray-50 hover:border-blue-700 \
                    focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2",

            onmounted: move |e: MountedEvent| {
                element.set(Some(e.data()));
            },
            onclick: move |_| {
                let new_open = !(ctx.open)();
                ctx.set_open.call(new_open);
                // Focus the element on click. Safari does not do this automatically. https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#clicking_and_focus
                if let Some(data) = element() {
                    spawn(async move {
                        _ = data.set_focus(true).await;
                    });
                }
            },
            onblur: move |_| {
                if !ctx.focus.any_focused() {
                    ctx.focus.blur();
                }
            },

            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`DropdownContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownContentProps {
    /// The ID of the dropdown menu content element. If not provided, a unique ID will be generated.
    pub id: ReadSignal<Option<String>>,
    /// Additional attributes to apply to the dropdown menu content element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the dropdown menu content, which should include one or more [`DropdownItem`] components.
    pub children: Element,
}

/// # DropdownTrigger
///
/// The contents of a [`Dropdown`]. The component will only be rendered when the parent [`Dropdown`] is open (as control by the [`DropdownTrigger`]).
///
/// This must be used inside a [`Dropdown`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     Dropdown, DropdownContent, DropdownItem, DropdownTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Dropdown { default_open: false,
///             DropdownTrigger { "Open Menu" }
///             DropdownContent {
///                 DropdownItem::<String> {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownItem::<String> {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the dropdown menu. values are `open` or `closed`.
#[component]
pub fn DropdownContent(props: DropdownContentProps) -> Element {
    let ctx: DropdownContext = use_context();

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);
    let render = use_animated_open(id, ctx.open);

    rsx! {
        if render() {
            div {
                id,
                role: "listbox",
                aria_labelledby: "{ctx.trigger_id}",
                "data-state": if (ctx.open)() { "open" } else { "closed" },
                // .dropdown-menu-content
                class: "absolute z-[9999] top-full left-0 min-w-[200px] p-1 rounded-lg mt-1 \
                        bg-white border border-gray-200 shadow-lg \
                        opacity-0 -translate-y-2 scale-[0.95] \
                        transition-all duration-200 ease-out \
                        data-[state=open]:opacity-100 data-[state=open]:translate-y-0 data-[state=open]:scale-100 \
                        data-[state=closed]:pointer-events-none \
                        will-change-transform",
                onpointerdown: move |event| {
                    // The user is starting a click inside the dropdown menu.
                    // Prevent the blur event from occurring during pointerdown,
                    // to keep the dropdown menu open until pointerup happens,
                    // thus enabling onclick/onselect events to fire.
                    event.prevent_default();
                    event.stop_propagation();
                },
                ..props.attributes,
                {props.children}
            }
        }
    }
}

/// The props for the [`DropdownItem`] component
#[derive(Props, Clone, PartialEq)]
pub struct DropdownItemProps<T: Clone + PartialEq + 'static> {
    /// The value of the item, which will be passed to the `on_select` callback when clicked.
    pub value: ReadSignal<T>,
    /// The index of the item within the [`DropdownContent`]. This is used to order the items for keyboard navigation.
    pub index: ReadSignal<usize>,

    /// Whether the item is disabled. If true, the item will not be clickable and will not respond to keyboard events.
    /// Defaults to false.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// The callback function that will be called when the item is selected. The value of the item will be passed as an argument.
    #[props(default)]
    pub on_select: Callback<T>,

    /// Additional attributes to apply to the item element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the item, which will be rendered inside the item element.
    pub children: Element,
}

/// # DropdownTrigger
///
/// An item within a [`DropdownContent`]. This component represents an individual selectable item in the dropdown menu.
///
/// This must be used inside a [`Dropdown`] component.
///
/// ## Example
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::dropdown_menu::{
///     Dropdown, DropdownContent, DropdownItem, DropdownTrigger,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Dropdown { default_open: false,
///             DropdownTrigger { "Open Menu" }
///             DropdownContent {
///                 DropdownItem::<String> {
///                     value: "edit".to_string(),
///                     index: 0usize,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Edit"
///                 }
///                 DropdownItem::<String> {
///                     value: "undo".to_string(),
///                     index: 1usize,
///                     disabled: true,
///                     on_select: move |value| {
///                         tracing::info!("Selected: {}", value);
///                     },
///                     "Undo"
///                 }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`DropdownItem`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates whether the item is disabled. Values are `true` or `false`.
#[component]
pub fn DropdownItem<T: Clone + PartialEq + 'static>(props: DropdownItemProps<T>) -> Element {
    let mut ctx: DropdownContext = use_context();

    let disabled = move || (ctx.disabled)() || (props.disabled)();
    let focused = move || ctx.focus.is_focused((props.index)());

    let onmounted = use_focus_controlled_item(props.index);

    rsx! {
        div {
            role: "option",
            "data-disabled": disabled(),
            tabindex: if focused() { "0" } else { "-1" },

            onmounted,

            // .dropdown-menu-item
            class: "flex items-center px-3 py-2 mx-1 rounded-md \
                    text-sm outline-none select-none cursor-pointer \
                    text-gray-900 \
                    data-[disabled=true]:text-gray-400 \
                    data-[disabled=true]:cursor-not-allowed \
                    hover:bg-gray-100 \
                    hover:text-gray-900 \
                    focus-visible:bg-gray-100 \
                    focus-visible:text-gray-900",

            onclick: move |e: Event<MouseData>| {
                e.stop_propagation();
                if !disabled() {
                    props.on_select.call((props.value)());
                    ctx.set_open.call(false);
                }
            },

            onkeydown: move |event: Event<KeyboardData>| {
                if event.key() == Key::Enter || event.key() == Key::Character(" ".to_string()) {
                    if !disabled() {
                        props.on_select.call((props.value)());
                        ctx.set_open.call(false);
                    }
                    event.prevent_default();
                    event.stop_propagation();
                }
            },

            onblur: move |_| {
                if focused() {
                    ctx.focus.blur();
                }
            },


            ..props.attributes,
            {props.children}
        }
    }
}

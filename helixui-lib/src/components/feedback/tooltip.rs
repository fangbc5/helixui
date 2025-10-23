//! Defines the [`Tooltip`] component and its sub-components, which provide contextual information when hovering or focusing on elements.

use crate::components::{
    use_animated_open, use_controlled, use_id_or, use_unique_id, ContentAlign, ContentSide,
};
use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct TooltipCtx {
    // State
    open: Memo<bool>,
    set_open: Callback<bool>,
    disabled: ReadSignal<bool>,

    // ARIA attributes
    tooltip_id: Signal<String>,
}

/// The props for the [`Tooltip`] component
#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    /// Whether the tooltip is open
    pub open: ReadSignal<Option<bool>>,

    /// Default open state when uncontrolled
    #[props(default)]
    pub default_open: bool,

    /// Callback when open state changes
    #[props(default)]
    pub on_open_change: Callback<bool>,

    /// Whether the tooltip is disabled
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Additional attributes for the tooltip
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tooltip component, which should include a [`TooltipTrigger`] and a [`TooltipContent`].
    pub children: Element,
}

/// # Tooltip
///
/// The `Tooltip` component provides contextual information when users hover or focus on an
/// element. It consists of a [`TooltipTrigger`] that activates the tooltip and a [`TooltipContent`]
/// that displays the message.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{tooltip::{Tooltip, TooltipContent, TooltipTrigger}, ContentSide};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Tooltip {
///             TooltipTrigger {
///                 "Rich content"
///             }
///             TooltipContent {
///                 side: ContentSide::Left,
///                 style: "width: 200px;",
///                 h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
///                 p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Tooltip`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the tooltip. Values are `open` or `closed`.
/// - `data-disabled`: Indicates if the tooltip is disabled. Values are `true` or `false`.
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let (open, set_open) = use_controlled(props.open, props.default_open, props.on_open_change);
    let tooltip_id = use_unique_id();

    let _ctx = use_context_provider(|| TooltipCtx {
        open,
        set_open,
        disabled: props.disabled,
        tooltip_id,
    });

    rsx! {
        div {
            class: "tooltip relative inline-block group",
            "data-state": if open() { "open" } else { "closed" },
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`TooltipTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct TooltipTriggerProps {
    /// Optional ID for the trigger element
    #[props(default)]
    pub id: Option<String>,

    /// Additional attributes for the trigger element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the trigger element
    pub children: Element,
}

/// # TooltipTrigger
///
/// The trigger element for the [`Tooltip`] component. When users hover over or focus on this element, the tooltip content will be displayed.
///
/// This must be used inside a [`Tooltip`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{tooltip::{Tooltip, TooltipContent, TooltipTrigger}, ContentSide};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Tooltip {
///             TooltipTrigger {
///                 "Rich content"
///             }
///             TooltipContent {
///                 side: ContentSide::Left,
///                 style: "width: 200px;",
///                 h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
///                 p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
///             }
///         }
///     }
/// }
/// ```
#[component]
pub fn TooltipTrigger(props: TooltipTriggerProps) -> Element {
    let ctx: TooltipCtx = use_context();

    // Handle mouse events
    let handle_mouse_enter = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_mouse_leave = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    // Handle focus events
    let handle_focus = move |_: Event<FocusData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    let handle_blur = move |_: Event<FocusData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    // Handle keyboard events
    let handle_keydown = move |event: Event<KeyboardData>| {
        if event.key() == Key::Escape && (ctx.open)() {
            event.prevent_default();
            ctx.set_open.call(false);
        }
    };

    rsx! {
        div {
            class: "tooltip-trigger inline-block",
            id: props.id.clone(),
            tabindex: "0",
            // Mouse events
            onmouseenter: handle_mouse_enter,
            onmouseleave: handle_mouse_leave,
            // Focus events
            onfocus: handle_focus,
            onblur: handle_blur,
            // Keyboard events
            onkeydown: handle_keydown,
            // ARIA attributes
            aria_describedby: ctx.tooltip_id.cloned(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`TooltipContent`] component
#[derive(Props, Clone, PartialEq)]
pub struct TooltipContentProps {
    /// Optional ID for the tooltip content
    #[props(default)]
    pub id: ReadSignal<Option<String>>,

    /// Side of the trigger to place the tooltip
    #[props(default = ContentSide::Top)]
    pub side: ContentSide,

    /// Alignment of the tooltip relative to the trigger
    #[props(default = ContentAlign::Center)]
    pub align: ContentAlign,

    /// Additional attributes for the tooltip content element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tooltip content
    pub children: Element,
}

/// # TooltipContent
///
/// The content component for the [`Tooltip`] that displays the actual tooltip message. The content will only be
/// rendered when the tooltip is open (as controlled by the [`TooltipTrigger`] component).
///
/// This must be used inside a [`Tooltip`] component.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::{tooltip::{Tooltip, TooltipContent, TooltipTrigger}, ContentSide};
///
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Tooltip {
///             TooltipTrigger {
///                 "Rich content"
///             }
///             TooltipContent {
///                 side: ContentSide::Left,
///                 style: "width: 200px;",
///                 h4 { style: "margin-top: 0; margin-bottom: 8px;", "Tooltip title" }
///                 p { style: "margin: 0;", "This tooltip contains rich HTML content with styling." }
///             }
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`TooltipContent`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the current state of the tooltip. Values are `open` or `closed`.
/// - `data-side`: Indicates which side of the trigger the tooltip is positioned. Values are `top`, `right`, `bottom`, or `left`.
/// - `data-align`: Indicates the alignment of the tooltip. Values are `start`, `center`, or `end`.
#[component]
pub fn TooltipContent(props: TooltipContentProps) -> Element {
    let mut ctx: TooltipCtx = use_context();

    let unique_id = use_unique_id();
    let id = use_id_or(unique_id, props.id);

    use_effect(move || {
        ctx.tooltip_id.set(id());
    });

    // Only render if the tooltip is open
    let render = use_animated_open(id, ctx.open);

    // 根据side和align计算定位样式和箭头样式
    let (positioning_class, arrow_class) = match props.side {
        ContentSide::Top => match props.align {
            ContentAlign::Start => ("bottom-full left-0 mb-2", "after:top-full after:left-4 after:border-t-gray-800 after:border-t-4 after:border-x-transparent after:border-x-4 after:border-b-0"),
            ContentAlign::Center => ("bottom-full left-1/2 mb-2 -translate-x-1/2", "after:top-full after:left-1/2 after:-translate-x-1/2 after:border-t-gray-800 after:border-t-4 after:border-x-transparent after:border-x-4 after:border-b-0"),
            ContentAlign::End => ("bottom-full right-0 mb-2", "after:top-full after:right-4 after:border-t-gray-800 after:border-t-4 after:border-x-transparent after:border-x-4 after:border-b-0"),
        },
        ContentSide::Right => match props.align {
            ContentAlign::Start => ("top-0 left-full ml-2", "after:top-4 after:left-0 after:-translate-x-1/2 after:border-r-gray-800 after:border-r-[6px] after:border-y-transparent after:border-y-[6px] after:border-l-0"),
            ContentAlign::Center => ("top-1/2 left-full ml-2 -translate-y-1/2", "after:top-1/2 after:left-0 after:-translate-x-1/2 after:-translate-y-1/2 after:border-r-gray-800 after:border-r-[6px] after:border-y-transparent after:border-y-[6px] after:border-l-0"),
            ContentAlign::End => ("bottom-0 left-full ml-2", "after:bottom-4 after:left-0 after:-translate-x-1/2 after:border-r-gray-800 after:border-r-[6px] after:border-y-transparent after:border-y-[6px] after:border-l-0"),
        },
        ContentSide::Bottom => match props.align {
            ContentAlign::Start => ("top-full left-0 mt-2", "after:bottom-full after:left-4 after:border-b-gray-800 after:border-b-4 after:border-x-transparent after:border-x-4 after:border-t-0"),
            ContentAlign::Center => ("top-full left-1/2 mt-2 -translate-x-1/2", "after:bottom-full after:left-1/2 after:-translate-x-1/2 after:border-b-gray-800 after:border-b-4 after:border-x-transparent after:border-x-4 after:border-t-0"),
            ContentAlign::End => ("top-full right-0 mt-2", "after:bottom-full after:right-4 after:border-b-gray-800 after:border-b-4 after:border-x-transparent after:border-x-4 after:border-t-0"),
        },
        ContentSide::Left => match props.align {
            ContentAlign::Start => ("top-0 right-full mr-2", "after:top-4 after:right-0 after:translate-x-1/2 after:border-l-gray-800 after:border-l-[6px] after:border-y-transparent after:border-y-[6px] after:border-r-0"),
            ContentAlign::Center => ("top-1/2 right-full mr-2 -translate-y-1/2", "after:top-1/2 after:right-0 after:translate-x-1/2 after:-translate-y-1/2 after:border-l-gray-800 after:border-l-[6px] after:border-y-transparent after:border-y-[6px] after:border-r-0"),
            ContentAlign::End => ("bottom-0 right-full mr-2", "after:bottom-4 after:right-0 after:translate-x-1/2 after:border-l-gray-800 after:border-l-[6px] after:border-y-transparent after:border-y-[6px] after:border-r-0"),
        },
    };

    // 处理鼠标进入tooltip内容
    let handle_mouse_enter = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(true);
        }
    };

    // 处理鼠标离开tooltip内容
    let handle_mouse_leave = move |_: Event<MouseData>| {
        if !(ctx.disabled)() {
            ctx.set_open.call(false);
        }
    };

    // Create the tooltip content
    rsx! {
        if render() {
            div {
                id,
                role: "tooltip",
                class: format!("absolute z-[1000] max-w-[250px] px-3 py-2 rounded-lg bg-gray-800 text-white text-sm leading-relaxed opacity-0 group-hover:opacity-100 transition-opacity duration-300 delay-300 group-hover:delay-0 group-hover:duration-200 after:absolute after:content-[''] after:w-0 after:h-0 {} {}", positioning_class, arrow_class),
                "data-state": if ctx.open.cloned() { "open" } else { "closed" },
                "data-side": props.side.as_str(),
                "data-align": props.align.as_str(),
                onmouseenter: handle_mouse_enter,
                onmouseleave: handle_mouse_leave,
                ..props.attributes,
                {props.children}
            }
        }
    }
}

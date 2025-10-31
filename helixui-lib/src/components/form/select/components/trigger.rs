//! SelectTrigger component implementation.

use dioxus::prelude::*;

use super::super::context::SelectContext;

/// The props for the [`SelectTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectTriggerProps {
    /// 自定义触发器宽度（例如 "12rem"、"16rem"）。可选
    #[props(default)]
    pub width: Option<String>,

    /// Additional attributes for the trigger button
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the trigger
    pub children: Element,
}

/// # SelectTrigger
///
/// The trigger button for the [`Select`](super::select::Select) component which controls if the [`SelectList`](super::list::SelectList) is rendered.
///
/// This must be used inside a [`Select`](super::select::Select) component.
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::select::{
///     Select, SelectGroup, SelectGroupLabel, SelectItemIndicator, SelectList, SelectOption,
///     SelectTrigger, SelectValue,
/// };
/// #[component]
/// fn Demo() -> Element {
///     rsx! {
///         Select::<String> {
///             placeholder: "Select a fruit...",
///             SelectTrigger {
///                 aria_label: "Select Trigger",
///                 width: "12rem",
///                 SelectValue {}
///             }
///             SelectList {
///                 aria_label: "Select Demo",
///                 SelectGroup {
///                     SelectGroupLabel { "Fruits" }
///                     SelectOption::<String> {
///                         index: 0usize,
///                         value: "apple",
///                         "Apple"
///                         SelectItemIndicator { "✔️" }
///                     }
///                     SelectOption::<String> {
///                         index: 1usize,
///                         value: "banana",
///                         "Banana"
///                         SelectItemIndicator { "✔️" }
///                     }
///                 }
///             }
///         }
///     }
/// }
/// ```
///
///
/// ## Styling
///
/// The [`SelectTrigger`] component defines a span with a `data-placeholder` attribute if a placeholder is set.
#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    let mut ctx = use_context::<SelectContext>();
    let mut open = ctx.open;

    let style_width = props.width.clone();
    let trigger_style = use_memo(move || {
        let mut style = String::new();
        if let Some(width) = &style_width {
            style.push_str(&format!("width: {}; ", width));
        }
        style
    });

    rsx! {
        button {
            class: "select-trigger inline-flex items-center justify-between w-56 px-4 py-2 rounded-xl bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-900 dark:text-gray-100 shadow-sm cursor-pointer gap-2 transition-colors duration-100 ease-out hover:bg-gray-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500",
            style: trigger_style,
            "data-disabled": (ctx.disabled)(),
            // Standard HTML attributes
            disabled: (ctx.disabled)(),
            type: "button",

            onclick: move |_| {
                open.toggle();
            },
            onkeydown: move |event| {
                match event.key() {
                    Key::ArrowUp => {
                        open.set(true);
                        ctx.focus_state.focus_last();
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    Key::ArrowDown => {
                        open.set(true);
                        ctx.focus_state.focus_first();
                        event.prevent_default();
                        event.stop_propagation();
                    }
                    _ => {}
                }
            },

            // ARIA attributes
            aria_haspopup: "listbox",
            aria_expanded: open(),
            aria_controls: ctx.list_id,

            // Pass through other attributes
            ..props.attributes,

            // Render children (value)
            div { class: "flex-1 text-left truncate",
                {props.children}
            }
            // caret icon
            svg {
                class: "select-expand-icon w-5 h-5 shrink-0 stroke-gray-500 dark:stroke-gray-400",
                view_box: "0 0 24 24",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "6 9 12 15 18 9", stroke_linecap: "round", stroke_linejoin: "round", stroke_width: "2" }
            }
        }
    }
}

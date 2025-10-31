//! SelectTrigger component implementation.

use dioxus::prelude::*;

use super::super::context::SelectContext;
use crate::components::{Icon, IconType};

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

    // 计算 input 的 value 值
    let input_value = use_memo(move || {
        if (ctx.filterable)() {
            (ctx.filter_query)()
        } else {
            let mut text = String::new();
            if (ctx.multiple)() {
                let options = ctx.options.read();
                let selected = (ctx.selected_values).read();
                let parts: Vec<String> = selected
                    .iter()
                    .filter_map(|v| {
                        options
                            .iter()
                            .find(|o| o.value == *v)
                            .map(|o| o.text_value.clone())
                    })
                    .collect();
                text = parts.join(", ");
            } else {
                let cur = ctx.value.read();
                if let Some(val) = cur.as_ref() {
                    if let Some(opt) = ctx.options.read().iter().find(|o| o.value == *val) {
                        text = opt.text_value.clone();
                    }
                }
            }
            if text.is_empty() {
                (ctx.placeholder)()
            } else {
                text
            }
        }
    });

    rsx! {
        Fragment {
            // 仅使用 dioxus 自带 input
            input {
                r#type: if (ctx.filterable)() { "text" } else { "button" },
                class: {
                    let size = (ctx.size)();
                    let size_class = match size {
                        super::super::context::SelectSize::Small => "px-3 py-1 text-sm",
                        super::super::context::SelectSize::Medium => "px-4 py-2 text-base",
                        super::super::context::SelectSize::Large => "px-5 py-3 text-lg",
                    };
                    // 右侧给图标预留 padding
                    format!("select-trigger inline-block align-middle w-56 pr-8 rounded-xl bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-900 dark:text-gray-100 shadow-sm placeholder:text-gray-400 {}", size_class)
                },
                style: trigger_style,
                placeholder: (ctx.placeholder)(),
                value: input_value(),
                readonly: !(ctx.filterable)(),
                onfocus: move |_| { open.set(true); },
                onclick: move |e| { e.stop_propagation(); open.set(true); },
                onpointerdown: move |e| { e.stop_propagation(); },
                oninput: move |e| { if (ctx.filterable)() { (ctx.filter_query).set(e.value().to_string()); } },
                onkeydown: move |event| {
                    match event.key() {
                        Key::ArrowUp => { open.set(true); ctx.focus_state.focus_last(); event.prevent_default(); event.stop_propagation(); }
                        Key::ArrowDown => { open.set(true); ctx.focus_state.focus_first(); event.prevent_default(); event.stop_propagation(); }
                        Key::Escape => { open.set(false); event.prevent_default(); event.stop_propagation(); }
                        _ => {}
                    }
                },
                onblur: move |_| {
                    // 若失焦（并非点击列表，因为列表已阻止默认），则关闭面板
                    if open() { open.set(false); }
                },
                "aria-haspopup": "listbox",
                "aria-expanded": open(),
                aria_controls: ctx.list_id,
                ..props.attributes,
            }
            // 自研图标：作为兄弟节点，通过负边距与相对定位覆盖到输入框右侧
            Icon {
                icon: IconType::ChevronDown,
                class: "inline-block align-middle -ml-7 -translate-x-1 relative text-gray-500 dark:text-gray-400".to_string()
            }
        }
    }
}

//! SelectValue component implementation.

use dioxus::prelude::*;

use super::super::context::RcPartialEqValue;
use super::super::context::SelectContext;
use crate::components::{Tag, TagSize, TagType, TagVariant};

/// The props for the [`SelectValue`] component
#[derive(Props, Clone, PartialEq)]
pub struct SelectValueProps {
    /// Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

/// # SelectValue
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
/// The [`SelectValue`] component defines a span with a `data-placeholder` attribute if a placeholder is set.
#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    let mut ctx = use_context::<SelectContext>();

    // 单选文本
    let single_text = use_memo(move || {
        let value = ctx.value.read();
        value.as_ref().and_then(|v| {
            ctx.options
                .read()
                .iter()
                .find(|opt| opt.value == *v)
                .map(|opt| opt.text_value.clone())
        })
    });

    // 多选条目：(value, text)
    let selected_items = use_memo(move || {
        let mut items: Vec<(RcPartialEqValue, String)> = Vec::new();
        if (ctx.multiple)() {
            let selected = (ctx.selected_values).read().clone();
            let options = ctx.options.read();
            for v in selected.into_iter() {
                if let Some(text) = options
                    .iter()
                    .find(|opt| opt.value == v)
                    .map(|opt| opt.text_value.clone())
                {
                    items.push((v, text));
                }
            }
        }
        items
    });

    let is_placeholder = if (ctx.multiple)() {
        (ctx.selected_values).read().is_empty()
    } else {
        ctx.value.read().is_none()
    };
    let display_value = single_text().unwrap_or_else(|| ctx.placeholder.cloned());

    let base_class = "select-value";
    let class = if is_placeholder {
        format!("{} {}", base_class, "text-[var(--secondary-color-5)]")
    } else {
        base_class.to_string()
    };

    rsx! {
        if (ctx.multiple)() {
            div { class: "flex flex-wrap gap-1 items-center",
                if is_placeholder {
                    span { class: "text-gray-400", {display_value} }
                } else {
                    for (val, text) in selected_items().into_iter() {
                        span {
                            onpointerdown: move |e| { e.prevent_default(); e.stop_propagation(); },
                            onclick: move |e| { e.prevent_default(); e.stop_propagation(); },
                            Tag { tag_type: TagType::Default, variant: TagVariant::Soft, size: TagSize::Small, round: true, closable: true, class: Some("max-w-full".to_string()),
                                on_close: move |_| {
                                    let mut list = (ctx.selected_values).read().clone();
                                    let remove_val = val.clone();
                                    if let Some(idx) = list.iter().position(|v| *v == remove_val) {
                                        list.remove(idx);
                                        (ctx.selected_values).set(list.clone());
                                        (ctx.set_selected_values).call(list);
                                    }
                                },
                                span { class: "truncate", "{text}" }
                            }
                        }
                    }
                }
            }
        } else {
            // 单选：保留原有行为
            span {
                class: "{class}",
                "data-placeholder": is_placeholder,
                ..props.attributes,
                {display_value}
            }
        }
    }
}

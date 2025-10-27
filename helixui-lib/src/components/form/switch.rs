//! Defines the [`Switch`] component and its sub-components.

use crate::components::use_controlled;
use dioxus::prelude::*;

/// The props for the [`Switch`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SwitchProps {
    /// The controlled checked state of the switch.
    pub checked: ReadSignal<Option<bool>>,

    /// The default checked state when uncontrolled.
    #[props(default = false)]
    pub default_checked: bool,

    /// Whether the switch is disabled.
    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub disabled: ReadSignal<bool>,

    /// Whether the switch is required in a form.
    #[props(default)]
    pub required: ReadSignal<bool>,

    /// The name attribute for form submission.
    #[props(default)]
    pub name: ReadSignal<String>,

    /// The value attribute for form submission.
    #[props(default = ReadSignal::new(Signal::new(String::from("on"))))]
    pub value: ReadSignal<String>,

    /// Callback fired when the checked state changes.
    #[props(default)]
    pub on_checked_change: Callback<bool>,

    /// Additional attributes to apply to the switch element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the switch component.
    pub children: Element,
}

/// # Switch
///
/// The `Switch` component is a toggle control that allows users to switch a state on or off.
///
/// ## Example
///
/// ```rust
/// use dioxus::prelude::*;
/// use dioxus_primitives::switch::{Switch, SwitchThumb};
/// #[component]
/// fn Demo() -> Element {
///     let mut checked = use_signal(|| false);
///     rsx! {
///         Switch {
///             checked: checked(),
///             aria_label: "Switch Demo",
///             SwitchThumb {}
///         }
///     }
/// }
/// ```
///
/// ## Styling
///
/// The [`Switch`] component defines the following data attributes you can use to control styling:
/// - `data-state`: Indicates the state of the switch. Values are `checked` or `unchecked`.
/// - `data-disabled`: Indicates if the switch is disabled. Values are `true` or `false`.
#[component]
pub fn Switch(props: SwitchProps) -> Element {
    let (checked, set_checked) = use_controlled(
        props.checked,
        props.default_checked,
        props.on_checked_change,
    );

    rsx! {
        div {
            class: "flex items-center gap-3",
            {props.children}

            button {
                type: "button",
                role: "switch",
                value: props.value,
                aria_checked: checked,
                aria_required: props.required,
                disabled: props.disabled,
                class: if (props.disabled)() {
                    "relative w-11 h-6 rounded-full transition-all duration-300 cursor-not-allowed opacity-50 shadow-inner"
                } else if checked() {
                    "relative w-11 h-6 rounded-full bg-blue-500 dark:bg-blue-600 transition-all duration-300 cursor-pointer shadow-inner hover:bg-blue-600 dark:hover:bg-blue-500"
                } else {
                    "relative w-11 h-6 rounded-full bg-gray-200 dark:bg-gray-700 transition-all duration-300 cursor-pointer shadow-inner hover:bg-gray-300 dark:hover:bg-gray-600"
                },
                "data-state": if checked() { "checked" } else { "unchecked" },
                "data-disabled": if (props.disabled)() { "true" } else { "false" },

                onclick: move |_| {
                    if !(props.disabled)() {
                        let new_checked = !checked();
                        set_checked.call(new_checked);
                    }
                },

                // Switches should only toggle on Space, not Enter
                onkeydown: move |e| {
                    if e.key() == Key::Enter {
                        e.prevent_default();
                    }
                },

                ..props.attributes,

                span {
                    class: if checked() {
                        "block w-5 h-5 rounded-full bg-white shadow-md transition-all duration-300 will-change-transform translate-x-[calc(2.75rem-1.25rem-2px)] transform"
                    } else {
                        "block w-5 h-5 rounded-full bg-white shadow-md transition-all duration-300 will-change-transform translate-x-[2px] transform"
                    },
                }
            }
        }

        // Hidden input for form submission
        input {
            type: "checkbox",
            aria_hidden: true,
            tabindex: -1,
            name: props.name,
            value: props.value,
            checked,
            disabled: props.disabled,
            style: "transform: translateX(-100%); position: absolute; pointer-events: none; opacity: 0; margin: 0; width: 0; height: 0;",
        }
    }
}

/// The props for the [`SwitchThumb`] component.
#[derive(Props, Clone, PartialEq)]
pub struct SwitchThumbProps {
    /// Additional attributes to apply to the thumb element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    /// The children of the thumb component.
    pub children: Element,
}

/// # SwitchThumb
///
/// The `SwitchThumb` component represents the visual thumb indicator that moves when the switch is toggled.
///
/// This must be used inside a [`Switch`] component.
///
/// ## Example
///
/// ```rust
///
/// use dioxus::prelude::*;
/// use dioxus_primitives::switch::{Switch, SwitchThumb};
/// #[component]
/// fn Demo() -> Element {
///     let mut checked = use_signal(|| false);
///     rsx! {
///         Switch {
///             checked: checked(),
///             aria_label: "Switch Demo",
///             SwitchThumb {}
///         }
///     }
/// }
/// ```
#[component]
pub fn SwitchThumb(props: SwitchThumbProps) -> Element {
    rsx! {
        span {
            class: "block w-[calc(1.15rem-2px)] h-[calc(1.15rem-2px)] rounded-full bg-white dark:bg-gray-300 transition-all duration-150 will-change-transform",
            ..props.attributes,
            {props.children}
        }
    }
}

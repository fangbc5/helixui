use dioxus::prelude::*;

use super::{Button, ButtonShape, ButtonSize, Icon, IconSize, IconType};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TagType {
    Default,
    Primary,
    Success,
    Warning,
    Error,
    Info,
}

impl TagType {
    fn solid_classes(&self) -> &'static str {
        match self {
            TagType::Default => "bg-gray-200 text-gray-800 dark:bg-gray-600 dark:text-gray-100",
            TagType::Primary => "bg-green-600 text-white",
            TagType::Success => "bg-emerald-600 text-white",
            TagType::Warning => "bg-amber-500 text-white",
            TagType::Error => "bg-red-600 text-white",
            TagType::Info => "bg-blue-600 text-white",
        }
    }

    fn soft_classes(&self) -> &'static str {
        match self {
            TagType::Default => "bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-200",
            TagType::Primary => {
                "bg-green-50 text-green-700 dark:bg-green-900/30 dark:text-green-200"
            }
            TagType::Success => {
                "bg-emerald-50 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-200"
            }
            TagType::Warning => {
                "bg-amber-50 text-amber-700 dark:bg-amber-900/30 dark:text-amber-200"
            }
            TagType::Error => "bg-red-50 text-red-700 dark:bg-red-900/30 dark:text-red-200",
            TagType::Info => "bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-200",
        }
    }

    fn outline_classes(&self) -> &'static str {
        match self {
            TagType::Default => {
                "border border-gray-300 text-gray-700 dark:border-gray-500 dark:text-gray-200"
            }
            TagType::Primary => "border border-green-500 text-green-600 dark:text-green-300",
            TagType::Success => "border border-emerald-500 text-emerald-600 dark:text-emerald-300",
            TagType::Warning => "border border-amber-500 text-amber-600 dark:text-amber-300",
            TagType::Error => "border border-red-500 text-red-600 dark:text-red-300",
            TagType::Info => "border border-blue-500 text-blue-600 dark:text-blue-300",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TagVariant {
    Solid,
    Soft,
    Outline,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TagSize {
    Small,
    Medium,
    Large,
}

impl TagSize {
    fn classes(&self) -> &'static str {
        match self {
            TagSize::Small => "px-2 py-0.5 text-xs gap-1",
            TagSize::Medium => "px-2.5 py-0.5 text-sm gap-1.5",
            TagSize::Large => "px-3 py-1 text-base gap-2",
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct TagProps {
    #[props(default = TagType::Default)]
    pub tag_type: TagType,
    #[props(default = TagVariant::Soft)]
    pub variant: TagVariant,
    #[props(default = TagSize::Medium)]
    pub size: TagSize,
    #[props(default = false)]
    pub round: bool,
    #[props(default = false)]
    pub closable: bool,
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub class: Option<String>,
    pub children: Element,
}

#[component]
pub fn Tag(props: TagProps) -> Element {
    let shape_class = if props.round {
        "rounded-full"
    } else {
        "rounded"
    };
    let size_class = props.size.classes();
    let base_class = "inline-flex items-center font-medium select-none align-middle whitespace-nowrap transition-colors";

    let variant_class = match props.variant {
        TagVariant::Solid => props.tag_type.solid_classes(),
        TagVariant::Soft => props.tag_type.soft_classes(),
        TagVariant::Outline => props.tag_type.outline_classes(),
    };

    let disabled_class = if props.disabled {
        "opacity-60 cursor-not-allowed"
    } else {
        ""
    };

    let user_class = props.class.unwrap_or_default();

    rsx! {
        span {
            class: "{base_class} {shape_class} {size_class} {variant_class} {disabled_class} {user_class}",
            // 内容
            span { {props.children} }

            // 关闭按钮（使用自研 Button 的纯图标用法）
            if props.closable {
                Button {
                    class: Some("ml-1".to_string()),
                    button_type: super::ButtonType::Tertiary,
                    size: match props.size {
                        TagSize::Small => ButtonSize::Tiny,
                        TagSize::Medium => ButtonSize::Small,
                        TagSize::Large => ButtonSize::Medium,
                    },
                    shape: ButtonShape::Circle,
                    outline: false,
                    disabled: props.disabled,
                    onclick: move |_| {
                        if let Some(cb) = &props.on_close { cb.call(()) }
                    },
                    Icon { icon: IconType::Close, size: match props.size { TagSize::Small => IconSize::Small, TagSize::Medium => IconSize::Small, TagSize::Large => IconSize::Medium }, class: "opacity-70".to_string() }
                }
            }
        }
    }
}

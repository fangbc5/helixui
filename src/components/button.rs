use dioxus::prelude::*;

/// Button 组件的类型
#[derive(Clone, PartialEq)]
pub enum ButtonType {
    Default,
    Primary,
    Info,
    Success,
    Warning,
    Error,
    Tertiary,
}

impl ButtonType {
    pub fn to_class(&self) -> &str {
        match self {
            ButtonType::Default => "bg-white text-gray-700 border border-gray-300 hover:bg-gray-50",
            ButtonType::Primary => "bg-green-500 text-white hover:bg-green-600",
            ButtonType::Info => "bg-blue-500 text-white hover:bg-blue-600",
            ButtonType::Success => "bg-emerald-500 text-white hover:bg-emerald-600",
            ButtonType::Warning => "bg-orange-500 text-white hover:bg-orange-600",
            ButtonType::Error => "bg-red-500 text-white hover:bg-red-600",
            ButtonType::Tertiary => "bg-transparent text-gray-700 hover:bg-gray-100",
        }
    }
}

/// Button 组件的尺寸
#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl ButtonSize {
    pub fn to_class(&self) -> &str {
        match self {
            ButtonSize::Small => "px-3 py-1 text-sm",
            ButtonSize::Medium => "px-4 py-2 text-base",
            ButtonSize::Large => "px-6 py-3 text-lg",
        }
    }
}

/// 按钮组件
#[component]
pub fn Button(
    #[props(default = ButtonType::Default)] button_type: ButtonType,
    #[props(default = ButtonSize::Medium)] size: ButtonSize,
    #[props(default = false)] disabled: bool,
    #[props(default = false)] secondary: bool,
    children: Element,
) -> Element {
    let type_class = button_type.to_class();
    let size_class = size.to_class();
    let disabled_class = if disabled {
        "opacity-50 cursor-not-allowed"
    } else {
        "cursor-pointer"
    };
    let secondary_class = if secondary {
        match button_type {
            ButtonType::Primary => "bg-green-100 text-green-700 hover:bg-green-200",
            ButtonType::Info => "bg-blue-100 text-blue-700 hover:bg-blue-200",
            ButtonType::Success => "bg-emerald-100 text-emerald-700 hover:bg-emerald-200",
            ButtonType::Warning => "bg-orange-100 text-orange-700 hover:bg-orange-200",
            ButtonType::Error => "bg-red-100 text-red-700 hover:bg-red-200",
            _ => type_class,
        }
    } else {
        type_class
    };

    rsx! {
        button {
            class: "rounded-md font-medium transition-colors duration-200 {secondary_class} {size_class} {disabled_class}",
            disabled: disabled,
            {children}
        }
    }
}

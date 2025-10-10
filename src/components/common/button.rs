use crate::components::{Icon, IconType};
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
    Tiny,
    Small,
    Medium,
    Large,
}

/// Button 组件的变体
#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Text,     // 纯文字按钮
    Icon,     // 纯图标按钮
    IconText, // 图标+文字按钮
}

/// Button 组件的形状
#[derive(Clone, PartialEq)]
pub enum ButtonShape {
    Default, // 默认矩形
    Rounded, // 圆角矩形
    Circle,  // 圆形
    Ellipse, // 椭圆形
}

impl ButtonSize {
    pub fn to_class(&self) -> &str {
        match self {
            ButtonSize::Tiny => "px-2 py-0.5 text-xs",
            ButtonSize::Small => "px-3 py-1 text-sm",
            ButtonSize::Medium => "px-4 py-2 text-base",
            ButtonSize::Large => "px-6 py-3 text-lg",
        }
    }

    pub fn icon_class(&self) -> &str {
        match self {
            ButtonSize::Tiny => "w-3 h-3",
            ButtonSize::Small => "w-4 h-4",
            ButtonSize::Medium => "w-5 h-5",
            ButtonSize::Large => "w-6 h-6",
        }
    }

    pub fn icon_only_class(&self) -> &str {
        match self {
            ButtonSize::Tiny => "p-1",
            ButtonSize::Small => "p-1.5",
            ButtonSize::Medium => "p-2",
            ButtonSize::Large => "p-3",
        }
    }
}

impl ButtonShape {
    pub fn to_class(&self) -> &str {
        match self {
            ButtonShape::Default => "rounded-none",
            ButtonShape::Rounded => "rounded-md",
            ButtonShape::Circle => "rounded-full",
            ButtonShape::Ellipse => "rounded-full",
        }
    }

    pub fn size_class(&self, size: &ButtonSize) -> &str {
        match self {
            ButtonShape::Circle => match size {
                ButtonSize::Tiny => "w-6 h-6",
                ButtonSize::Small => "w-8 h-8",
                ButtonSize::Medium => "w-10 h-10",
                ButtonSize::Large => "w-12 h-12",
            },
            ButtonShape::Ellipse => match size {
                ButtonSize::Tiny => "w-8 h-6",
                ButtonSize::Small => "w-12 h-8",
                ButtonSize::Medium => "w-16 h-10",
                ButtonSize::Large => "w-20 h-12",
            },
            _ => "", // Default 和 Rounded 不需要特殊尺寸
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
    #[props(default = ButtonVariant::Text)] variant: ButtonVariant,
    #[props(default = ButtonShape::Rounded)] shape: ButtonShape,
    #[props(default)] icon: Option<IconType>,
    #[props(default)] onclick: Option<EventHandler<()>>,
    #[props(default)] class: Option<String>,
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

    // 根据变体和形状确定样式类
    let shape_class = shape.to_class();
    let shape_size_class = shape.size_class(&size);

    let base_class = match variant {
        ButtonVariant::Text => {
            let padding_class = if shape == ButtonShape::Circle || shape == ButtonShape::Ellipse {
                "" // 圆形和椭圆形使用固定尺寸，不需要 padding
            } else {
                size_class
            };
            format!(
                "{} font-medium transition-colors duration-200 {} {} {} {}",
                shape_class, secondary_class, padding_class, shape_size_class, disabled_class
            )
        }
        ButtonVariant::Icon => {
            let padding_class = if shape == ButtonShape::Circle || shape == ButtonShape::Ellipse {
                "" // 圆形和椭圆形使用固定尺寸，不需要 padding
            } else {
                size.icon_only_class()
            };
            format!(
                "{} font-medium transition-colors duration-200 {} {} {} {}",
                shape_class, secondary_class, padding_class, shape_size_class, disabled_class
            )
        }
        ButtonVariant::IconText => {
            let padding_class = if shape == ButtonShape::Circle || shape == ButtonShape::Ellipse {
                "" // 圆形和椭圆形使用固定尺寸，不需要 padding
            } else {
                size_class
            };
            format!(
                "{} font-medium transition-colors duration-200 {} {} {} {} flex items-center gap-2",
                shape_class, secondary_class, padding_class, shape_size_class, disabled_class
            )
        }
    };

    let final_class = if let Some(custom_class) = class {
        format!("{} {}", base_class, custom_class)
    } else {
        base_class
    };

    rsx! {
        button {
            class: final_class,
            disabled: disabled,
            onclick: move |_| {
                if let Some(handler) = &onclick {
                    handler.call(());
                }
            },
            match variant {
                ButtonVariant::Text => rsx! { {children} },
                ButtonVariant::Icon => rsx! {
                    if let Some(icon_type) = icon {
                        Icon {
                            icon: icon_type,
                            class: size.icon_class().to_string(),
                        }
                    }
                },
                ButtonVariant::IconText => rsx! {
                    if let Some(icon_type) = icon {
                        Icon {
                            icon: icon_type,
                            class: size.icon_class().to_string(),
                        }
                    }
                    {children}
                },
            }
        }
    }
}

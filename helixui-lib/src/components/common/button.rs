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
    PureText,  // 纯文字按钮（无背景色和边框）
    PureIcon,  // 纯图标按钮（无背景色和边框）
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
            ButtonType::PureText => "bg-transparent text-gray-700 hover:text-green-600",
            ButtonType::PureIcon => "bg-transparent text-gray-500 hover:text-green-600",
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
            ButtonSize::Tiny => "w-3.5 h-3.5", // 从 3x3 改为 3.5x3.5，与 7x7 按钮更好匹配
            ButtonSize::Small => "w-4 h-4",
            ButtonSize::Medium => "w-5 h-5",
            ButtonSize::Large => "w-6 h-6",
        }
    }

    pub fn icon_only_class(&self) -> &str {
        match self {
            ButtonSize::Tiny => "p-1.5", // 从 p-1 改为 p-1.5，为 7x7 按钮提供更好的图标居中
            ButtonSize::Small => "p-2",  // 从 p-1.5 改为 p-2，为 8x8 按钮提供更好的图标居中
            ButtonSize::Medium => "p-2.5", // 从 p-2 改为 p-2.5，为 10x10 按钮提供更好的图标居中
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
                ButtonSize::Tiny => "w-7 h-7", // 从 6x6 改为 7x7，提供更好的图标空间
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
    #[props(default = false)] dashed: bool,
    #[props(default = ButtonVariant::Text)] variant: ButtonVariant,
    #[props(default = ButtonShape::Rounded)] shape: ButtonShape,
    #[props(default)] icon: Option<IconType>,
    #[props(default)] onclick: Option<EventHandler<()>>,
    #[props(default)] class: Option<String>,
    #[props(default)] hover_color: Option<String>,
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
            ButtonType::PureText => "bg-transparent text-gray-500 hover:text-green-600",
            ButtonType::PureIcon => "bg-transparent text-gray-400 hover:text-green-600",
            _ => type_class,
        }
    } else {
        type_class
    };

    // 处理虚线边框
    let dashed_class = if dashed {
        "border-2 border-dashed"
    } else {
        ""
    };

    // 根据变体和形状确定样式类
    let shape_class = shape.to_class();
    let shape_size_class = shape.size_class(&size);

    let base_class = match variant {
        ButtonVariant::Text => {
            let padding_class = if button_type == ButtonType::PureText {
                "p-0" // 纯文字按钮无 padding
            } else if shape == ButtonShape::Circle || shape == ButtonShape::Ellipse {
                "" // 圆形和椭圆形使用固定尺寸，不需要 padding
            } else {
                size_class
            };
            format!(
                "{} font-medium transition-colors duration-200 {} {} {} {} {}",
                shape_class, secondary_class, dashed_class, padding_class, shape_size_class, disabled_class
            )
        }
        ButtonVariant::Icon => {
            let padding_class = if button_type == ButtonType::PureIcon {
                "p-0" // 纯图标按钮无 padding
            } else if shape == ButtonShape::Circle || shape == ButtonShape::Ellipse {
                "" // 圆形和椭圆形使用固定尺寸，不需要 padding
            } else {
                size.icon_only_class()
            };
            format!(
                "{} font-medium transition-colors duration-200 {} {} {} {} {}",
                shape_class, secondary_class, dashed_class, padding_class, shape_size_class, disabled_class
            )
        }
        ButtonVariant::IconText => {
            let padding_class = if shape == ButtonShape::Circle || shape == ButtonShape::Ellipse {
                "" // 圆形和椭圆形使用固定尺寸，不需要 padding
            } else {
                size_class
            };
            format!(
                "{} font-medium transition-colors duration-200 {} {} {} {} {} flex items-center gap-2",
                shape_class, secondary_class, dashed_class, padding_class, shape_size_class, disabled_class
            )
        }
    };

    let final_class = if let Some(custom_class) = class {
        format!("{} {}", base_class, custom_class)
    } else {
        base_class
    };

    // 处理自定义 hover 颜色
    let final_class_with_hover = if let Some(hover_color) = hover_color {
        // 移除默认的 hover 颜色类，添加自定义的
        let hover_class = match button_type {
            ButtonType::PureText | ButtonType::PureIcon => {
                format!("hover:text-[{}]", hover_color)
            },
            _ => {
                // 对于其他按钮类型，可能需要更复杂的处理
                // 这里先简单处理为文字颜色
                format!("hover:text-[{}]", hover_color)
            }
        };
        format!("{} {}", final_class, hover_class)
    } else {
        final_class
    };

    rsx! {
        button {
            class: final_class_with_hover,
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

/// 按钮组属性
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupProps {
    /// 按钮列表
    pub buttons: Vec<ButtonGroupItemProps>,
    
    /// 按钮组尺寸
    #[props(default = ButtonSize::Medium)]
    pub size: ButtonSize,
    
    /// 按钮组形状
    #[props(default = ButtonShape::Rounded)]
    pub shape: ButtonShape,
    
    /// 是否紧凑模式（按钮之间无间距）
    #[props(default = false)]
    pub compact: bool,
    
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
}

/// 按钮组中的单个按钮属性
#[derive(Props, Clone, PartialEq)]
pub struct ButtonGroupItemProps {
    /// 按钮类型
    #[props(default = ButtonType::Default)]
    pub button_type: ButtonType,
    
    /// 按钮变体
    #[props(default = ButtonVariant::Text)]
    pub variant: ButtonVariant,
    
    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,
    
    /// 是否次要样式
    #[props(default = false)]
    pub secondary: bool,
    
    /// 是否虚线边框
    #[props(default = false)]
    pub dashed: bool,
    
    /// 图标
    #[props(default)]
    pub icon: Option<IconType>,
    
    /// 点击事件
    #[props(default)]
    pub onclick: Option<EventHandler<()>>,
    
    /// 自定义 hover 颜色
    #[props(default)]
    pub hover_color: Option<String>,
    
    /// 按钮内容
    pub children: Element,
}

impl Default for ButtonGroupItemProps {
    fn default() -> Self {
        Self {
            button_type: ButtonType::Default,
            variant: ButtonVariant::Text,
            disabled: false,
            secondary: false,
            dashed: false,
            icon: None,
            onclick: None,
            hover_color: None,
            children: rsx! {},
        }
    }
}

/// 按钮组组件
#[component]
pub fn ButtonGroup(props: ButtonGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");
    let gap_class = if props.compact { "" } else { "gap-1" };
    
    let base_class = format!("inline-flex {}", gap_class);
    let final_class = if custom_class.is_empty() {
        base_class
    } else {
        format!("{} {}", base_class, custom_class)
    };
    
    let buttons_len = props.buttons.len();
    let size = props.size.clone();
    let shape = props.shape.clone();
    
    rsx! {
        div {
            class: final_class,
            for (index, button) in props.buttons.iter().enumerate() {
                Button {
                    button_type: button.button_type.clone(),
                    size: size.clone(),
                    disabled: button.disabled,
                    secondary: button.secondary,
                    dashed: button.dashed,
                    variant: button.variant.clone(),
                    shape: if props.compact {
                        // 紧凑模式下，第一个按钮左边圆角，最后一个按钮右边圆角
                        match index {
                            0 => ButtonShape::Rounded, // 第一个按钮
                            _ if index == buttons_len - 1 => ButtonShape::Rounded, // 最后一个按钮
                            _ => ButtonShape::Default, // 中间按钮
                        }
                    } else {
                        shape.clone()
                    },
                    icon: button.icon,
                    onclick: button.onclick.clone(),
                    hover_color: button.hover_color.clone(),
                    class: if props.compact {
                        // 紧凑模式下的特殊样式
                        match index {
                            0 => Some("rounded-l-md rounded-r-none".to_string()),
                            _ if index == buttons_len - 1 => Some("rounded-r-md rounded-l-none".to_string()),
                            _ => Some("rounded-none".to_string()),
                        }
                    } else {
                        None
                    },
                    {button.children.clone()}
                }
            }
        }
    }
}
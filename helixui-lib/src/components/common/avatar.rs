use super::icon::{Icon, IconSize, IconType};
use dioxus::prelude::*;

/// 头像尺寸
#[derive(Debug, Clone, PartialEq)]
pub enum AvatarSize {
    Small,
    Medium,
    Large,
    Custom(f32),
}

impl AvatarSize {
    pub fn to_class(&self) -> &'static str {
        match self {
            AvatarSize::Small => "w-8 h-8 text-xs",
            AvatarSize::Medium => "w-10 h-10 text-sm",
            AvatarSize::Large => "w-12 h-12 text-base",
            AvatarSize::Custom(_) => "",
        }
    }

    pub fn to_style(&self) -> String {
        match self {
            AvatarSize::Custom(size) => format!("width: {}px; height: {}px;", size, size),
            _ => String::new(),
        }
    }
}

impl Default for AvatarSize {
    fn default() -> Self {
        AvatarSize::Medium
    }
}

/// 头像形状
#[derive(Debug, Clone, PartialEq)]
pub enum AvatarShape {
    Circle,
    Square,
}

impl AvatarShape {
    pub fn to_class(&self) -> &'static str {
        match self {
            AvatarShape::Circle => "rounded-full",
            AvatarShape::Square => "rounded-md",
        }
    }
}

impl Default for AvatarShape {
    fn default() -> Self {
        AvatarShape::Circle
    }
}

/// 头像属性
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// 头像尺寸
    #[props(default)]
    pub size: AvatarSize,

    /// 头像形状
    #[props(default)]
    pub shape: AvatarShape,

    /// 头像颜色
    #[props(default)]
    pub color: Option<String>,

    /// 头像背景色
    #[props(default)]
    pub background_color: Option<String>,

    /// 头像文字
    #[props(default)]
    pub text: Option<String>,

    /// 头像图片源
    #[props(default)]
    pub src: Option<String>,

    /// 图片加载失败时的备用文字
    #[props(default)]
    pub fallback_text: Option<String>,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,

    /// 自定义样式
    #[props(default)]
    pub style: Option<String>,

    /// 子元素
    pub children: Option<Element>,
}

/// 头像组件
#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let size_class = props.size.to_class();
    let shape_class = props.shape.to_class();
    let size_style = props.size.to_style();

    let custom_class = props.class.as_deref().unwrap_or("");
    let custom_style = props.style.as_deref().unwrap_or("");

    let background_style = if let Some(bg_color) = &props.background_color {
        format!("background-color: {};", bg_color)
    } else {
        String::new()
    };

    let color_style = if let Some(color) = &props.color {
        format!("color: {};", color)
    } else {
        String::new()
    };

    let final_style = format!(
        "{} {} {} {}",
        size_style, background_style, color_style, custom_style
    );

    let base_class = format!(
        "inline-flex items-center justify-center font-medium select-none {} {} {}",
        size_class, shape_class, custom_class
    );

    // 图片尺寸
    let icon_size = match props.size {
        AvatarSize::Small => IconSize::Small,
        AvatarSize::Medium => IconSize::Medium,
        AvatarSize::Large => IconSize::Large,
        AvatarSize::Custom(_) => IconSize::Medium,
    };

    // 图片加载失败状态
    let mut image_error = use_signal(|| false);

    rsx! {
        div {
            class: base_class,
            style: final_style,

            if let Some(children) = props.children {
                {children}
            } else if let Some(src) = &props.src {
                // 图片头像
                if !image_error() {
                    img {
                        src: src.clone(),
                        alt: props.fallback_text.as_deref().unwrap_or(""),
                        class: "w-full h-full object-cover",
                        style: shape_class,
                        onerror: move |_| {
                            image_error.set(true);
                        }
                    }
                } else {
                    // 图片加载失败时显示裂开图标
                    Icon {
                        icon: IconType::ImageBroken,
                        size: icon_size,
                        class: "text-gray-400".to_string(),
                    }
                }
            } else if let Some(text) = &props.text {
                // 文字头像
                span {
                    class: "text-center",
                    {text.clone()}
                }
            } else {
                // 默认头像
                Icon {
                    icon: IconType::User,
                    size: icon_size,
                    class: "text-gray-400".to_string(),
                }
            }
        }
    }
}

/// 头像组属性
#[derive(Props, Clone, PartialEq)]
pub struct AvatarGroupProps {
    /// 头像列表
    pub avatars: Vec<AvatarProps>,

    /// 最大显示数量
    #[props(default = 3)]
    pub max: usize,

    /// 头像间距
    #[props(default = -8)]
    pub spacing: i32,

    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
}

/// 头像组组件
#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    let custom_class = props.class.as_deref().unwrap_or("");
    let spacing_style = format!("margin-left: {}px;", props.spacing);

    let display_avatars = if props.avatars.len() > props.max {
        let mut result = props.avatars[..props.max].to_vec();

        // 添加更多数量的提示
        let more_count = props.avatars.len() - props.max;
        result.push(AvatarProps {
            text: Some(format!("+{}", more_count)),
            size: props.avatars[0].size.clone(),
            shape: props.avatars[0].shape.clone(),
            background_color: Some("#f0f0f0".to_string()),
            color: Some("#666666".to_string()),
            ..Default::default()
        });

        result
    } else {
        props.avatars
    };

    rsx! {
        div {
            class: format!("flex items-center {}", custom_class),
            for (index, avatar) in display_avatars.into_iter().enumerate() {
                div {
                    class: if index == 0 { "" } else { "relative" },
                    style: if index == 0 { "" } else { spacing_style.as_str() },
                    Avatar {
                        size: avatar.size,
                        shape: avatar.shape,
                        color: avatar.color,
                        background_color: avatar.background_color,
                        text: avatar.text,
                        src: avatar.src,
                        fallback_text: avatar.fallback_text,
                        class: avatar.class,
                        style: avatar.style,
                        children: avatar.children,
                    }
                }
            }
        }
    }
}

// 为 AvatarProps 实现 Default
impl Default for AvatarProps {
    fn default() -> Self {
        Self {
            size: AvatarSize::default(),
            shape: AvatarShape::default(),
            color: None,
            background_color: None,
            text: None,
            src: None,
            fallback_text: None,
            class: None,
            style: None,
            children: None,
        }
    }
}

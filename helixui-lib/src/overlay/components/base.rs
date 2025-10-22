use crate::overlay::core::{
    AnimationConfig, AnimationManager, AnimationType, EasingType, PlatformAdapter, ThemeManager,
    ThemeMode,
};
use crate::overlay::utils::merge_classes;
use crate::{Button, ButtonShape, ButtonSize, ButtonType};
use dioxus::prelude::*;

/// Overlay 位置枚举
#[derive(Debug, Clone, PartialEq)]
pub enum OverlayPosition {
    Fixed { x: f64, y: f64 },
    Center,
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Custom(String), // CSS 类名
}

// 移除了具体渲染逻辑，BaseOverlay 只提供通用能力

/// 基础 Overlay 属性
#[derive(Props, Clone, PartialEq)]
pub struct BaseOverlayProps {
    pub visible: bool,
    pub z_index: u32,
    pub position: OverlayPosition,
    pub animation: Option<AnimationConfig>,
    pub theme_mode: Option<ThemeMode>,

    // 交互属性
    pub mask_closable: bool,
    pub closable: bool,
    pub draggable: bool,
    pub resizable: bool,

    // 回调
    pub on_close: Option<fn()>,
    pub on_show: Option<fn()>,
    pub on_hide: Option<fn()>,

    // 内容 - 由具体组件提供
    pub children: Element,
}

/// 基础 Overlay 组件
#[component]
pub fn BaseOverlay(props: BaseOverlayProps) -> Element {
    let platform_adapter = use_signal(|| PlatformAdapter::new());
    let _animation_manager = use_signal(|| AnimationManager::new(platform_adapter.read().clone()));
    let theme_manager = use_signal(|| ThemeManager::new());

    let mut is_dragging = use_signal(|| false);
    let mut drag_offset = use_signal(|| (0.0, 0.0));
    let _position = use_signal(|| (0.0, 0.0));

    // 获取样式类
    let base_class = theme_manager.read().get_base_class();
    let shadow_class = theme_manager.read().get_shadow_class();
    let mask_class = theme_manager.read().get_mask_class();

    let animation_config = props
        .animation
        .unwrap_or_else(|| AnimationConfig::default());

    // 使用 core 中的动画类型
    let animation_class = if props.visible {
        format!(
            "animate-{} duration-{} {}",
            animation_type_to_class(&animation_config.enter),
            animation_config.duration,
            easing_type_to_class(&animation_config.easing)
        )
    } else {
        format!(
            "animate-{} duration-{} {}",
            animation_type_to_class(&animation_config.exit),
            animation_config.duration,
            easing_type_to_class(&animation_config.easing)
        )
    };

    let merged_class = merge_classes(&[&base_class, &shadow_class, &animation_class]);
    let position_style = get_position_style(&props.position);

    rsx! {
        if props.visible {
            // 遮罩层
            if props.mask_closable {
                div {
                    class: format!("fixed inset-0 z-{} {}", props.z_index - 1, mask_class),
                    onclick: move |_| {
                        if let Some(callback) = props.on_close {
                            callback();
                        }
                    },
                }
            }

            // 内容层
            div {
                class: format!("fixed z-{} {}", props.z_index, merged_class),
                style: position_style,

                // 拖拽处理
                if props.draggable {
                    div {
                        class: "cursor-move select-none",
                        onmousedown: move |e: MouseEvent| {
                            is_dragging.set(true);
                            let offset_x = e.element_coordinates().x as f64;
                            let offset_y = e.element_coordinates().y as f64;
                            drag_offset.set((offset_x, offset_y));
                        },
                        "拖拽区域"
                    }
                }

                // 关闭按钮
                if props.closable {
                    Button {
                        button_type: ButtonType::Tertiary,
                        size: ButtonSize::Small,
                        shape: ButtonShape::Circle,
                        class: Some("absolute top-2 right-2 text-gray-400 hover:text-gray-600".to_string()),
                        onclick: move |_| {
                            if let Some(callback) = props.on_close {
                                callback();
                            }
                        },
                        "×"
                    }
                }

                // 渲染具体内容 - 由具体组件提供
                {props.children}
            }
        }
    }
}

/// 获取位置样式
fn get_position_style(position: &OverlayPosition) -> String {
    match position {
        OverlayPosition::Fixed { x, y } => {
            format!("left: {}px; top: {}px;", x, y)
        }
        OverlayPosition::Center => {
            "left: 50%; top: 50%; transform: translate(-50%, -50%);".to_string()
        }
        OverlayPosition::TopLeft => "top: 1rem; left: 1rem;".to_string(),
        OverlayPosition::TopCenter => {
            "top: 1rem; left: 50%; transform: translateX(-50%);".to_string()
        }
        OverlayPosition::TopRight => "top: 1rem; right: 1rem;".to_string(),
        OverlayPosition::BottomLeft => "bottom: 1rem; left: 1rem;".to_string(),
        OverlayPosition::BottomCenter => {
            "bottom: 1rem; left: 50%; transform: translateX(-50%);".to_string()
        }
        OverlayPosition::BottomRight => "bottom: 1rem; right: 1rem;".to_string(),
        OverlayPosition::Custom(class) => class.clone(),
    }
}

/// 将动画类型转换为 CSS 类名
fn animation_type_to_class(animation_type: &AnimationType) -> String {
    match animation_type {
        AnimationType::None => "none".to_string(),
        AnimationType::FadeIn => "fade-in".to_string(),
        AnimationType::FadeOut => "fade-out".to_string(),
        AnimationType::SlideIn => "slide-in".to_string(),
        AnimationType::SlideOut => "slide-out".to_string(),
        AnimationType::ScaleIn => "scale-in".to_string(),
        AnimationType::ScaleOut => "scale-out".to_string(),
        AnimationType::BounceIn => "bounce-in".to_string(),
        AnimationType::BounceOut => "bounce-out".to_string(),
        AnimationType::Custom(class) => class.clone(),
    }
}

/// 将缓动类型转换为 CSS 类名
fn easing_type_to_class(easing_type: &EasingType) -> String {
    match easing_type {
        EasingType::Linear => "linear".to_string(),
        EasingType::EaseIn => "ease-in".to_string(),
        EasingType::EaseOut => "ease-out".to_string(),
        EasingType::EaseInOut => "ease-in-out".to_string(),
        EasingType::EaseInQuad => "ease-in-quad".to_string(),
        EasingType::EaseOutQuad => "ease-out-quad".to_string(),
        EasingType::EaseInOutQuad => "ease-in-out-quad".to_string(),
        EasingType::EaseInCubic => "ease-in-cubic".to_string(),
        EasingType::EaseOutCubic => "ease-out-cubic".to_string(),
        EasingType::EaseInOutCubic => "ease-in-out-cubic".to_string(),
        EasingType::Custom(class) => class.clone(),
    }
}

// 添加 MessagePosition 到 OverlayPosition 的转换
impl From<crate::overlay::MessagePosition> for OverlayPosition {
    fn from(pos: crate::overlay::MessagePosition) -> Self {
        match pos {
            crate::overlay::MessagePosition::TopLeft => OverlayPosition::TopLeft,
            crate::overlay::MessagePosition::TopCenter => OverlayPosition::TopCenter,
            crate::overlay::MessagePosition::TopRight => OverlayPosition::TopRight,
            crate::overlay::MessagePosition::BottomLeft => OverlayPosition::BottomLeft,
            crate::overlay::MessagePosition::BottomCenter => OverlayPosition::BottomCenter,
            crate::overlay::MessagePosition::BottomRight => OverlayPosition::BottomRight,
            crate::overlay::MessagePosition::Center => OverlayPosition::Center,
        }
    }
}

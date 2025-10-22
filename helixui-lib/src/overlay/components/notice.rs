use crate::overlay::components::base::{BaseOverlay, BaseOverlayProps, OverlayPosition};
use crate::overlay::core::animation_manager::FillMode;
use crate::overlay::core::{AnimationConfig, AnimationType, EasingType, ThemeMode};
use crate::overlay::MessagePosition;
use crate::{Button, ButtonShape, ButtonSize, ButtonType};
use dioxus::prelude::*;

/// 轻提示 Overlay 基类
#[derive(Props, Clone, PartialEq)]
pub struct NoticeOverlayProps {
    pub visible: bool,
    pub on_close: Option<fn()>,
    pub duration: u32,
    pub closable: bool,
    pub position: NoticePosition,
    pub children: Element,
}

/// 轻提示位置（映射到 MessagePosition）
#[derive(Debug, Clone, PartialEq)]
pub enum NoticePosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Center,
}

impl From<NoticePosition> for MessagePosition {
    fn from(pos: NoticePosition) -> Self {
        match pos {
            NoticePosition::TopLeft => MessagePosition::TopLeft,
            NoticePosition::TopCenter => MessagePosition::TopCenter,
            NoticePosition::TopRight => MessagePosition::TopRight,
            NoticePosition::BottomLeft => MessagePosition::BottomLeft,
            NoticePosition::BottomCenter => MessagePosition::BottomCenter,
            NoticePosition::BottomRight => MessagePosition::BottomRight,
            NoticePosition::Center => MessagePosition::Center,
        }
    }
}

impl From<NoticePosition> for OverlayPosition {
    fn from(pos: NoticePosition) -> Self {
        match pos {
            NoticePosition::TopLeft => OverlayPosition::TopLeft,
            NoticePosition::TopCenter => OverlayPosition::TopCenter,
            NoticePosition::TopRight => OverlayPosition::TopRight,
            NoticePosition::BottomLeft => OverlayPosition::BottomLeft,
            NoticePosition::BottomCenter => OverlayPosition::BottomCenter,
            NoticePosition::BottomRight => OverlayPosition::BottomRight,
            NoticePosition::Center => OverlayPosition::Center,
        }
    }
}

/// 轻提示 Overlay 组件（使用新的 BaseOverlay 架构）
#[component]
pub fn NoticeOverlay(props: NoticeOverlayProps) -> Element {
    // 自动关闭定时器
    use_effect(move || {
        if props.duration > 0 {
            let duration = props.duration;
            let on_close = props.on_close.clone();

            spawn(async move {
                use std::time::Duration;
                tokio::time::sleep(Duration::from_millis(duration as u64)).await;

                if let Some(callback) = on_close {
                    callback();
                }
            });
        }
    });

    // 创建 BaseOverlay 属性
    let base_props = BaseOverlayProps {
        visible: props.visible,
        z_index: 2000,
        position: props.position.into(),
        animation: Some(AnimationConfig {
            duration: 200,
            delay: 0,
            easing: EasingType::EaseOut,
            enter: AnimationType::FadeIn,
            exit: AnimationType::FadeOut,
            fill_mode: FillMode::Forwards,
            iteration_count: 1,
        }),
        theme_mode: Some(ThemeMode::Auto),
        mask_closable: false,
        closable: true,
        draggable: false,
        resizable: false,
        on_close: props.on_close,
        on_show: None,
        on_hide: None,
        children: rsx! {
            // Notice 的具体渲染逻辑
            div {
                class: "p-4 rounded-lg shadow-lg border max-w-sm pointer-events-auto bg-white",
                role: "alert",
                "aria-live": "polite",

                div { class: "flex items-center",
                    // 内容
                    div { class: "flex-1 text-sm font-medium",
                        {props.children}
                    }

                    // 关闭按钮
                    if props.closable {
                        div { class: "ml-3 flex-shrink-0",
                            Button {
                                button_type: ButtonType::Tertiary,
                                size: ButtonSize::Small,
                                shape: ButtonShape::Circle,
                                class: Some("text-gray-400 hover:text-gray-600".to_string()),
                                onclick: move |_| {
                                    if let Some(callback) = props.on_close {
                                        callback();
                                    }
                                },
                                "×"
                            }
                        }
                    }
                }
            }
        },
    };

    rsx! {
        BaseOverlay { ..base_props }
    }
}

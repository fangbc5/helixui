use crate::overlay::core::{AnimationManager, PlatformAdapter, ThemeManager};
use crate::overlay::utils::{ErrorHandler, OverlayError, Result};
use crate::{Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant};
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

/// 轻提示位置
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

/// 轻提示 Overlay 组件
#[component]
pub fn NoticeOverlay(props: NoticeOverlayProps) -> Element {
    let platform_adapter = use_signal(|| PlatformAdapter::new());
    let animation_manager = use_signal(|| AnimationManager::new(platform_adapter.read().clone()));
    let theme_manager = use_signal(|| ThemeManager::new());
    let error_handler = use_signal(|| ErrorHandler::new());

    let visible = use_signal(|| props.visible);

    // 自动关闭定时器
    use_effect(move || {
        if props.duration > 0 {
            let duration = props.duration;
            let on_close = props.on_close.clone();

            spawn(async move {
                use std::time::Duration;
                futures_timer::Delay::new(Duration::from_millis(duration as u64)).await;

                if let Some(callback) = on_close {
                    callback();
                }
            });
        }
    });

    // 获取样式类
    let base_class = theme_manager.read().get_base_class();
    let shadow_class = theme_manager.read().get_shadow_class();
    let position_class = get_position_class(&props.position);
    let animation_class = if *visible.read() {
        animation_manager.read().create_enter_animation()
    } else {
        animation_manager.read().create_exit_animation()
    };

    let merged_class = merge_classes(&[
        &base_class,
        &shadow_class,
        &position_class,
        &animation_class,
    ]);

    rsx! {
        if *visible.read() {
            div {
                class: merged_class,
                role: "alert",
                "aria-live": "polite",

                // 内容
                div {
                    class: "flex items-center",
                    {props.children}
                }

                // 关闭按钮
                if props.closable {
                    Button {
                        button_type: ButtonType::PureText,
                        size: ButtonSize::Small,
                        variant: ButtonVariant::Text,
                        shape: ButtonShape::Circle,
                        class: Some("ml-2 text-gray-400 hover:text-gray-600".to_string()),
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
}

/// 获取位置样式类
fn get_position_class(position: &NoticePosition) -> String {
    match position {
        NoticePosition::TopLeft => "fixed top-4 left-4",
        NoticePosition::TopCenter => "fixed top-4 left-1/2 transform -translate-x-1/2",
        NoticePosition::TopRight => "fixed top-4 right-4",
        NoticePosition::BottomLeft => "fixed bottom-4 left-4",
        NoticePosition::BottomCenter => "fixed bottom-4 left-1/2 transform -translate-x-1/2",
        NoticePosition::BottomRight => "fixed bottom-4 right-4",
        NoticePosition::Center => {
            "fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
        }
    }
}

/// 合并类名
fn merge_classes(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|&&class| !class.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

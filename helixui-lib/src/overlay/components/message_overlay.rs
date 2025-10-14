use crate::overlay::core::{
    get_global_state_manager, AnimationManager, GlobalStateManager, OverlayType, PlatformAdapter,
    ThemeManager,
};
use crate::overlay::utils::{ErrorHandler, OverlayError, Result};
use crate::{Button, ButtonShape, ButtonSize, ButtonType};
use dioxus::prelude::*;

/// 消息类型
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Success,
    Warning,
    Error,
    Info,
    Loading,
}

/// 消息位置
#[derive(Debug, Clone, PartialEq)]
pub enum MessagePosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Center,
}

/// 消息数据
#[derive(Debug, Clone, PartialEq)]
pub struct MessageData {
    pub id: String,
    pub content: String,
    pub message_type: MessageType,
    pub position: MessagePosition,
    pub duration: u32,
    pub closable: bool,
    pub show_icon: bool,
}

/// 消息 Overlay 组件
#[component]
pub fn MessageOverlay(data: MessageData, on_close: Option<fn(String)>) -> Element {
    let platform_adapter = use_signal(|| PlatformAdapter::new());
    let animation_manager = use_signal(|| AnimationManager::new(platform_adapter.read().clone()));
    let theme_manager = use_signal(|| ThemeManager::new());
    let error_handler = use_signal(|| ErrorHandler::new());

    let visible = use_signal(|| true);
    let animation_class = use_signal(|| String::new());

    // 自动关闭定时器
    use_effect(move || {
        if data.duration > 0 {
            let duration = data.duration;
            let id = data.id.clone();
            let on_close = on_close.clone();

            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(duration).await;

                if let Some(callback) = on_close {
                    callback(id);
                }
            });
        }
    });

    // 获取样式类
    let base_class = theme_manager.read().get_base_class();
    let message_class = theme_manager
        .read()
        .get_message_class(&format!("{:?}", data.message_type));
    let position_class = get_position_class(&data.position);
    let animation_class = animation_manager.read().create_enter_animation();

    let merged_class = merge_classes(&[
        &base_class,
        &message_class,
        &position_class,
        &animation_class,
    ]);

    rsx! {
        div {
            class: merged_class,
            role: "alert",
            "aria-live": "polite",

            // 图标
            if data.show_icon {
                div {
                    class: "flex-shrink-0",
                    {get_icon(&data.message_type)}
                }
            }

            // 内容
            div {
                class: "flex-1",
                {data.content}
            }

            // 关闭按钮
            if data.closable {
                Button {
                    button_type: ButtonType::PureText,
                    size: ButtonSize::Small,
                    shape: ButtonShape::Circle,
                    class: Some("flex-shrink-0 ml-2 text-gray-400 hover:text-gray-600".to_string()),
                    onclick: move |_| {
                        if let Some(callback) = on_close {
                            callback(data.id.clone());
                        }
                    },
                    "×"
                }
            }
        }
    }
}

/// 获取位置样式类
fn get_position_class(position: &MessagePosition) -> String {
    match position {
        MessagePosition::TopLeft => "fixed top-4 left-4".to_string(),
        MessagePosition::TopCenter => "fixed top-4 left-1/2 transform -translate-x-1/2".to_string(),
        MessagePosition::TopRight => "fixed top-4 right-4".to_string(),
        MessagePosition::BottomLeft => "fixed bottom-4 left-4".to_string(),
        MessagePosition::BottomCenter => {
            "fixed bottom-4 left-1/2 transform -translate-x-1/2".to_string()
        }
        MessagePosition::BottomRight => "fixed bottom-4 right-4".to_string(),
        MessagePosition::Center => {
            "fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2".to_string()
        }
    }
}

/// 获取图标
fn get_icon(message_type: &MessageType) -> Element {
    match message_type {
        MessageType::Success => rsx! {
            div {
                class: "w-5 h-5 text-green-500",
                "✓"
            }
        },
        MessageType::Warning => rsx! {
            div {
                class: "w-5 h-5 text-yellow-500",
                "⚠"
            }
        },
        MessageType::Error => rsx! {
            div {
                class: "w-5 h-5 text-red-500",
                "✕"
            }
        },
        MessageType::Info => rsx! {
            div {
                class: "w-5 h-5 text-blue-500",
                "ℹ"
            }
        },
        MessageType::Loading => rsx! {
            div {
                class: "w-5 h-5 text-blue-500 animate-spin",
                "⟳"
            }
        },
    }
}

/// 合并类名
fn merge_classes(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|&&class| !class.is_empty())
        .map(|s| *s)
        .collect::<Vec<_>>()
        .join(" ")
}

/// 显示消息
pub fn show_message(content: String, message_type: MessageType) {
    let data = MessageData {
        id: generate_id("message"),
        content,
        message_type,
        position: MessagePosition::TopRight,
        duration: 2000,
        closable: true,
        show_icon: true,
    };

    if let Ok(mut manager) = get_global_state_manager().lock() {
        manager.add_overlay(
            data.id.clone(),
            OverlayType::Message,
            serde_json::to_string(&data).unwrap_or_default(),
        );
    }
}

/// 显示带持续时间的消息
pub fn show_message_with_duration(content: String, message_type: MessageType, duration: u32) {
    let data = MessageData {
        id: generate_id("message"),
        content,
        message_type,
        position: MessagePosition::TopRight,
        duration,
        closable: true,
        show_icon: true,
    };

    if let Ok(mut manager) = get_global_state_manager().lock() {
        manager.add_overlay(
            data.id.clone(),
            OverlayType::Message,
            serde_json::to_string(&data).unwrap_or_default(),
        );
    }
}

/// 显示带位置的消息
pub fn show_message_with_position(
    content: String,
    message_type: MessageType,
    position: MessagePosition,
) {
    let data = MessageData {
        id: generate_id("message"),
        content,
        message_type,
        position,
        duration: 2000,
        closable: true,
        show_icon: true,
    };

    if let Ok(mut manager) = get_global_state_manager().lock() {
        manager.add_overlay(
            data.id.clone(),
            OverlayType::Message,
            serde_json::to_string(&data).unwrap_or_default(),
        );
    }
}

/// 生成唯一 ID
fn generate_id(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    format!("{}_{}", prefix, timestamp)
}

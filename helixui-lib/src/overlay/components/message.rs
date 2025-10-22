use crate::overlay::components::base::{BaseOverlay, BaseOverlayProps, OverlayPosition};
use crate::overlay::components::types::MessageData;
use crate::overlay::core::animation_manager::FillMode;
use crate::overlay::core::{get_global_state_manager, OverlayType};
use crate::overlay::core::{AnimationConfig, AnimationType, EasingType, ThemeMode};
use crate::overlay::utils::generate_id;
use crate::overlay::{MessagePosition, MessageType};
use crate::{Button, ButtonShape, ButtonSize, ButtonType};
use dioxus::prelude::*;

// MessageData 已移至 types.rs

/// 消息 Overlay 组件（使用新的 BaseOverlay 架构）
#[component]
pub fn MessageOverlay(data: MessageData, on_close: Option<fn(String)>) -> Element {
    let data_id = data.id.clone();

    // 自动关闭定时器
    use_effect(move || {
        if data.duration > 0 {
            let duration_ms = data.duration as u64;
            let id = data.id.clone();
            let on_close = on_close.clone();

            spawn(async move {
                tokio::time::sleep(std::time::Duration::from_millis(duration_ms)).await;
                if let Some(callback) = on_close {
                    callback(id);
                }
            });
        }
    });

    // 创建 BaseOverlay 属性
    let base_props = BaseOverlayProps {
        visible: true,
        z_index: 1000,
        position: OverlayPosition::TopRight,
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
        on_close: None,
        on_show: None,
        on_hide: None,
        children: rsx! {
            // Message 的具体渲染逻辑
            div {
                class: "p-4 rounded-lg shadow-lg border max-w-sm pointer-events-auto bg-white",
                role: "alert",
                "aria-live": "polite",

                div { class: "flex items-center",
                    // 图标
                    if data.show_icon {
                        div { class: "flex-shrink-0 mr-3",
                            {get_message_icon(&data.message_type)}
                        }
                    }

                    // 内容
                    div { class: "flex-1 text-sm font-medium",
                        {data.content}
                    }

                    // 关闭按钮
                    if data.closable {
                        div { class: "ml-3 flex-shrink-0",
                            Button {
                                button_type: ButtonType::Tertiary,
                                size: ButtonSize::Small,
                                shape: ButtonShape::Circle,
                                class: Some("text-gray-400 hover:text-gray-600".to_string()),
                                onclick: move |_| {
                                    if let Some(callback) = on_close {
                                        callback(data_id.clone());
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

/// 获取消息图标
fn get_message_icon(message_type: &MessageType) -> Element {
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

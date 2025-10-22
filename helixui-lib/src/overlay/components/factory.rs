//! Overlay 工厂
//!
//! 提供创建各种 Overlay 组件的工厂方法

use crate::overlay::components::base::{BaseOverlayProps, OverlayPosition};
use crate::overlay::core::{AnimationConfig, AnimationType, EasingType, ThemeMode};
use crate::overlay::core::animation_manager::FillMode;
use crate::overlay::components::types::{DialogData, MessageData};
use crate::overlay::utils::generate_id;
use crate::overlay::{MessagePosition, MessageType};
use dioxus::prelude::*;

/// Overlay 工厂
pub struct OverlayFactory;

impl OverlayFactory {
    /// 创建消息 Overlay 属性
    pub fn create_message_props(
        content: String,
        message_type: MessageType,
        position: MessagePosition,
        duration: u32,
        closable: bool,
        show_icon: bool,
    ) -> BaseOverlayProps {
        let data = MessageData {
            id: generate_id("message"),
            content,
            message_type,
            position: position.clone(),
            duration,
            closable,
            show_icon,
        };

        BaseOverlayProps {
            visible: true,
            z_index: 1000,
            position: position.into(),
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
                // 这里应该渲染 Message 的具体内容
                div { "Message content will be rendered here" }
            },
        }
    }

    /// 创建对话框 Overlay 属性
    pub fn create_dialog_props(
        title: Option<String>,
        content: String,
        confirm_text: Option<String>,
        cancel_text: Option<String>,
        on_confirm: Option<fn()>,
        on_cancel: Option<fn()>,
    ) -> BaseOverlayProps {
        let data = DialogData {
            id: generate_id("dialog"),
            title,
            content,
            confirm_text,
            cancel_text,
            on_confirm: None, // DialogData 中不需要存储回调函数
            on_cancel: None,
        };

        BaseOverlayProps {
            visible: true,
            z_index: 4000,
            position: OverlayPosition::Center,
            animation: Some(AnimationConfig {
                duration: 300,
                delay: 0,
                easing: EasingType::EaseOut,
                enter: AnimationType::ScaleIn,
                exit: AnimationType::ScaleOut,
                fill_mode: FillMode::Forwards,
                iteration_count: 1,
            }),
            theme_mode: Some(ThemeMode::Auto),
            mask_closable: true,
            closable: true,
            draggable: false,
            resizable: false,
            on_close: None,
            on_show: None,
            on_hide: None,
            children: rsx! {
                // 这里应该渲染 Dialog 的具体内容
                div { "Dialog content will be rendered here" }
            },
        }
    }

    /// 创建通知 Overlay 属性
    pub fn create_notice_props(
        content: String,
        position: MessagePosition,
        duration: u32,
        closable: bool,
    ) -> BaseOverlayProps {
        let data = MessageData {
            id: generate_id("notice"),
            content,
            message_type: MessageType::Info,
            position: position.clone(),
            duration,
            closable,
            show_icon: false,
        };

        BaseOverlayProps {
            visible: true,
            z_index: 2000,
            position: position.into(),
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
                // 这里应该渲染 Notice 的具体内容
                div { "Notice content will be rendered here" }
            },
        }
    }
}
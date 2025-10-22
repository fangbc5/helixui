use crate::overlay::{MessagePosition, MessageType};

/// Message 数据
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MessageData {
    pub id: String,
    pub content: String,
    pub message_type: MessageType,
    pub position: MessagePosition,
    pub duration: u32,
    pub closable: bool,
    pub show_icon: bool,
}

/// Message 内容渲染器（已移至 base_overlay.rs）
#[derive(Clone)]
pub struct MessageContentRenderer;

/// Dialog 数据
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DialogData {
    pub id: String,
    pub title: Option<String>,
    pub content: String,
    pub confirm_text: Option<String>,
    pub cancel_text: Option<String>,
    pub on_confirm: Option<String>, // 回调标识
    pub on_cancel: Option<String>,  // 回调标识
}

/// Dialog 内容渲染器（已移至 base_overlay.rs）
#[derive(Clone)]
pub struct DialogContentRenderer;

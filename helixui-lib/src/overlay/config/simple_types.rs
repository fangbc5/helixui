//! 通用的简化类型：消息类型与位置

use serde::{Deserialize, Serialize};

/// 简化的消息类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    Success,
    Warning,
    Error,
    Info,
    Loading,
}

/// 简化的消息位置
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessagePosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Center,
}

use dioxus::prelude::*;
use super::notice_base::{NoticeBase, NoticeType, NoticePosition};

/// Message 类型（基于 NoticeType）
pub type MessageType = NoticeType;

/// Message 位置（基于 NoticePosition）
pub type MessagePosition = NoticePosition;

/// Message 属性
#[derive(Props, Clone, PartialEq)]
pub struct MessageProps {
    /// 是否显示消息
    #[props(default = false)]
    pub visible: bool,
    
    /// 消息内容
    pub content: String,
    
    /// 消息类型
    #[props(default = MessageType::Info)]
    pub message_type: MessageType,
    
    /// 消息位置
    #[props(default = MessagePosition::TopRight)]
    pub position: MessagePosition,
    
    /// 持续时间（毫秒），0 表示不自动关闭
    #[props(default = 2000)]
    pub duration: u32,
    
    /// 是否可关闭
    #[props(default = true)]
    pub closable: bool,
    
    /// 关闭事件
    pub on_close: Option<EventHandler<()>>,
    
    /// 自定义样式类
    pub class: Option<String>,
}

#[component]
pub fn Message(props: MessageProps) -> Element {
    rsx! {
        NoticeBase {
            visible: props.visible,
            position: props.position.to_string(),
            duration: props.duration,
            closable: props.closable,
            notice_type: props.message_type.to_string(),
            content: Some(props.content),
            on_close: props.on_close,
            class: props.class,
            
            // Message 组件不需要额外的子组件
            div {}
        }
    }
}

/// Message 管理器（简化版本）
#[derive(Clone)]
pub struct MessageManager {
    messages: Signal<Vec<MessageData>>,
}

#[derive(Clone)]
pub struct MessageData {
    pub id: u32,
    pub content: String,
    pub message_type: MessageType,
    pub position: MessagePosition,
    pub duration: u32,
}

impl MessageManager {
    pub fn new() -> Self {
        Self {
            messages: use_signal(|| Vec::new()),
        }
    }

    pub fn show(&mut self, content: String, message_type: MessageType, position: MessagePosition, duration: u32) {
        let id = self.messages.read().len() as u32;
        let message = MessageData {
            id,
            content,
            message_type,
            position,
            duration,
        };
        
        let mut messages = self.messages.write();
        messages.push(message);
    }

    pub fn close(&mut self, id: u32) {
        let mut messages = self.messages.write();
        messages.retain(|msg| msg.id != id);
    }

    pub fn close_all(&mut self) {
        self.messages.write().clear();
    }
}

/// 全局 Message 容器（简化版本）
#[component]
pub fn GlobalMessageContainer() -> Element {
    rsx! {
        div { class: "message-container" }
    }
}
/// 便捷函数
pub fn show_message(_content: String, _message_type: MessageType) {
    // 在实际应用中，这里会显示消息
}

pub fn show_message_with_duration(_content: String, _message_type: MessageType, _duration: u32) {
    // 在实际应用中，这里会显示带持续时间的消息
}

pub fn show_message_with_position(_content: String, _message_type: MessageType, _position: MessagePosition) {
    // 在实际应用中，这里会显示指定位置的消息
}

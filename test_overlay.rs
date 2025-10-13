//! 测试新的 Overlay 架构

use helixui::overlay::{
    SimpleMessage, SimpleMessageData, SimpleMessageType, SimpleMessagePosition,
    GlobalMessageContainer, show_message, show_message_with_duration, 
    show_message_with_position, close_message, close_all_messages,
};

fn main() {
    println!("测试新的 Overlay 架构");
    
    // 测试显示消息
    show_message("这是一个成功消息".to_string(), SimpleMessageType::Success);
    show_message("这是一个警告消息".to_string(), SimpleMessageType::Warning);
    show_message("这是一个错误消息".to_string(), SimpleMessageType::Error);
    show_message("这是一个信息消息".to_string(), SimpleMessageType::Info);
    show_message("这是一个加载消息".to_string(), SimpleMessageType::Loading);
    
    // 测试带持续时间的消息
    show_message_with_duration("5秒后消失的消息".to_string(), SimpleMessageType::Info, 5000);
    
    // 测试带位置的消息
    show_message_with_position("左上角消息".to_string(), SimpleMessageType::Success, SimpleMessagePosition::TopLeft);
    show_message_with_position("右下角消息".to_string(), SimpleMessageType::Warning, SimpleMessagePosition::BottomRight);
    
    println!("所有消息已发送，请检查 UI 中的显示效果");
}

use dioxus::prelude::*;
use super::popup_base::{PopupBase, ResponsiveSize};

/// Dialog 尺寸
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogSize {
    /// 小型对话框
    Small,
    /// 中型对话框（默认）
    Medium,
    /// 大型对话框
    Large,
    /// 全屏对话框
    Fullscreen,
}

impl DialogSize {
    pub fn to_responsive_size(&self) -> ResponsiveSize {
        match self {
            DialogSize::Small => ResponsiveSize::new()
                .with_responsive(
                    Some("max-w-xs"),  // 移动端
                    Some("max-w-sm"),  // 平板
                    Some("max-w-md"),  // 桌面
                    Some("max-w-md"),  // 大屏
                    Some("max-w-md"),  // 超大屏
                    Some("max-w-md")   // 超超大屏
                ),
            DialogSize::Medium => ResponsiveSize::new()
                .with_responsive(
                    Some("max-w-sm"),  // 移动端
                    Some("max-w-md"),  // 平板
                    Some("max-w-lg"),  // 桌面
                    Some("max-w-lg"),  // 大屏
                    Some("max-w-lg"),  // 超大屏
                    Some("max-w-lg")   // 超超大屏
                ),
            DialogSize::Large => ResponsiveSize::new()
                .with_responsive(
                    Some("max-w-md"),  // 移动端
                    Some("max-w-lg"),  // 平板
                    Some("max-w-2xl"), // 桌面
                    Some("max-w-2xl"), // 大屏
                    Some("max-w-2xl"), // 超大屏
                    Some("max-w-2xl")  // 超超大屏
                ),
            DialogSize::Fullscreen => ResponsiveSize::new()
                .with_responsive(
                    Some("max-w-full h-full"),  // 移动端
                    Some("max-w-full h-full"),  // 平板
                    Some("max-w-full h-full"),  // 桌面
                    Some("max-w-full h-full"),  // 大屏
                    Some("max-w-full h-full"),  // 超大屏
                    Some("max-w-full h-full")   // 超超大屏
                ),
        }
    }
}

/// Dialog 位置
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogPosition {
    /// 居中（默认）
    Center,
    /// 顶部
    Top,
    /// 底部
    Bottom,
}

impl DialogPosition {
    pub fn to_string(&self) -> String {
        match self {
            DialogPosition::Center => "center".to_string(),
            DialogPosition::Top => "top".to_string(),
            DialogPosition::Bottom => "bottom".to_string(),
        }
    }
}

/// Dialog 类型
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DialogType {
    /// 默认对话框
    Default,
    /// 确认对话框
    Confirm,
    /// 信息提示
    Info,
    /// 成功提示
    Success,
    /// 警告提示
    Warning,
    /// 错误提示
    Error,
}

impl DialogType {
    pub fn icon(&self) -> Option<&str> {
        match self {
            DialogType::Default => None,
            DialogType::Confirm => Some("M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"),
            DialogType::Info => Some("M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z"),
            DialogType::Success => Some("M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"),
            DialogType::Warning => Some("M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"),
            DialogType::Error => Some("M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"),
        }
    }

    pub fn icon_color(&self) -> &str {
        match self {
            DialogType::Default => "text-gray-600 dark:text-gray-300",
            DialogType::Confirm => "text-blue-600 dark:text-blue-400",
            DialogType::Info => "text-blue-600 dark:text-blue-400",
            DialogType::Success => "text-green-600 dark:text-green-400",
            DialogType::Warning => "text-orange-600 dark:text-orange-400",
            DialogType::Error => "text-red-600 dark:text-red-400",
        }
    }
}

/// Dialog 属性
#[derive(Props, Clone, PartialEq)]
pub struct DialogProps {
    /// 是否显示对话框
    #[props(default = false)]
    pub visible: bool,
    
    /// 对话框标题
    #[props(default)]
    pub title: Option<String>,
    
    /// 对话框内容
    pub children: Element,
    
    /// 对话框尺寸
    #[props(default = DialogSize::Medium)]
    pub size: DialogSize,
    
    /// 对话框位置
    #[props(default = DialogPosition::Center)]
    pub position: DialogPosition,
    
    /// 对话框类型
    #[props(default = DialogType::Default)]
    pub dialog_type: DialogType,
    
    /// 是否显示遮罩层
    #[props(default = true)]
    pub show_mask: bool,
    
    /// 遮罩层点击是否关闭
    #[props(default = true)]
    pub mask_closable: bool,
    
    /// 是否显示关闭按钮
    #[props(default = true)]
    pub closable: bool,
    
    /// 是否可拖拽
    #[props(default = false)]
    pub draggable: bool,
    
    /// 变换原点
    #[props(default = "center".to_string())]
    pub transform_origin: String,
    
    /// 关闭事件
    pub on_close: Option<EventHandler<()>>,
    
    /// 自定义样式类
    pub class: Option<String>,
}

#[component]
pub fn Dialog(props: DialogProps) -> Element {
    let _handle_close = move || {
        if let Some(on_close) = &props.on_close {
            on_close.call(());
        }
    };

    rsx! {
        PopupBase {
            visible: props.visible,
            position: props.position.to_string(),
            size: Some(props.size.to_responsive_size()),
            draggable: props.draggable,
            transform_origin: props.transform_origin,
            show_mask: props.show_mask,
            mask_closable: props.mask_closable,
            closable: props.closable,
            title: props.title.clone(),
            on_close: props.on_close.clone(),
            class: props.class.clone(),
            
            div { class: "space-y-4",
                // 类型图标
                if let Some(icon_path) = props.dialog_type.icon() {
                    div { class: "flex justify-center",
                        div { 
                            class: format!("w-12 h-12 rounded-full flex items-center justify-center {}", props.dialog_type.icon_color()),
                            svg {
                                class: "w-6 h-6",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: icon_path
                                }
                            }
                        }
                    }
                }
                
                // 内容
                {props.children}
            }
        }
    }
}
/// 便捷函数
pub fn show_confirm_dialog(_title: &str, _content: &str) {
    // 在实际应用中，这里会显示确认对话框
}

pub fn show_info_dialog(_title: &str, _content: &str) {
    // 在实际应用中，这里会显示信息对话框
}

pub fn show_success_dialog(_title: &str, _content: &str) {
    // 在实际应用中，这里会显示成功对话框
}

pub fn show_warning_dialog(_title: &str, _content: &str) {
    // 在实际应用中，这里会显示警告对话框
}

pub fn show_error_dialog(_title: &str, _content: &str) {
    // 在实际应用中，这里会显示错误对话框
}

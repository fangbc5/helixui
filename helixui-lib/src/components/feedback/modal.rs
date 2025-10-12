use dioxus::prelude::*;

/// 模态框尺寸
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ModalSize {
    /// 小型模态框
    Small,
    /// 中型模态框（默认）
    Medium,
    /// 大型模态框
    Large,
    /// 全屏模态框
    Fullscreen,
}

impl ModalSize {
    pub fn to_class(&self) -> &str {
        match self {
            ModalSize::Small => "max-w-md",
            ModalSize::Medium => "max-w-lg",
            ModalSize::Large => "max-w-2xl",
            ModalSize::Fullscreen => "max-w-full h-full",
        }
    }
}

/// 模态框位置
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ModalPosition {
    /// 居中（默认）
    Center,
    /// 顶部
    Top,
    /// 底部
    Bottom,
}

impl ModalPosition {
    pub fn to_class(&self) -> &str {
        match self {
            ModalPosition::Center => "items-center",
            ModalPosition::Top => "items-start pt-16",
            ModalPosition::Bottom => "items-end pb-16",
        }
    }
}

/// 模态框类型
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ModalType {
    /// 默认模态框
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

impl ModalType {
    pub fn to_class(&self) -> &str {
        match self {
            ModalType::Default => "bg-white dark:bg-gray-800",
            ModalType::Confirm => "bg-white dark:bg-gray-800",
            ModalType::Info => "bg-white dark:bg-gray-800",
            ModalType::Success => "bg-white dark:bg-gray-800",
            ModalType::Warning => "bg-white dark:bg-gray-800",
            ModalType::Error => "bg-white dark:bg-gray-800",
        }
    }

    pub fn icon(&self) -> Option<&str> {
        match self {
            ModalType::Default => None,
            ModalType::Confirm => Some("M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"),
            ModalType::Info => Some("M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z"),
            ModalType::Success => Some("M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z"),
            ModalType::Warning => Some("M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"),
            ModalType::Error => Some("M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"),
        }
    }

    pub fn icon_color(&self) -> &str {
        match self {
            ModalType::Default => "text-gray-600 dark:text-gray-300",
            ModalType::Confirm => "text-blue-600 dark:text-blue-400",
            ModalType::Info => "text-blue-600 dark:text-blue-400",
            ModalType::Success => "text-green-600 dark:text-green-400",
            ModalType::Warning => "text-orange-600 dark:text-orange-400",
            ModalType::Error => "text-red-600 dark:text-red-400",
        }
    }
}

/// 模态框属性
#[derive(Props, Clone, PartialEq)]
pub struct ModalProps {
    /// 是否显示模态框
    #[props(default = false)]
    pub visible: bool,
    
    /// 模态框标题
    #[props(default)]
    pub title: Option<String>,
    
    /// 模态框内容
    pub children: Element,
    
    /// 模态框尺寸
    #[props(default = ModalSize::Medium)]
    pub size: ModalSize,
    
    /// 模态框位置
    #[props(default = ModalPosition::Center)]
    pub position: ModalPosition,
    
    /// 模态框类型
    #[props(default = ModalType::Default)]
    pub modal_type: ModalType,
    
    /// 是否显示遮罩层
    #[props(default = true)]
    pub mask_closable: bool,
    
    /// 是否显示关闭按钮
    #[props(default = true)]
    pub closable: bool,
    
    /// 是否显示确认按钮
    #[props(default = false)]
    pub show_confirm: bool,
    
    /// 是否显示取消按钮
    #[props(default = false)]
    pub show_cancel: bool,
    
    /// 确认按钮文本
    #[props(default = "确认".to_string())]
    pub confirm_text: String,
    
    /// 取消按钮文本
    #[props(default = "取消".to_string())]
    pub cancel_text: String,
    
    /// 确认按钮点击事件
    #[props(default)]
    pub on_confirm: Option<EventHandler<()>>,
    
    /// 取消按钮点击事件
    #[props(default)]
    pub on_cancel: Option<EventHandler<()>>,
    
    /// 关闭事件
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
    
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 变换原点
    #[props(default = "center".to_string())]
    pub transform_origin: String,
    
    /// 是否显示遮罩层
    #[props(default = true)]
    pub show_mask: bool,
    
    /// 是否可拖拽
    #[props(default = false)]
    pub draggable: bool,
}

/// 模态框组件
#[component]
pub fn Modal(props: ModalProps) -> Element {
    // 直接使用 props.visible，不使用内部状态管理
    let is_visible = props.visible;

    let handle_close = move |_: MouseEvent| {
        if let Some(handler) = &props.on_close {
            handler.call(());
        }
    };

    let handle_mask_click = move |_: MouseEvent| {
        if props.mask_closable {
            if let Some(handler) = &props.on_close {
                handler.call(());
            }
        }
    };

    let handle_confirm = move |_: MouseEvent| {
        if let Some(handler) = &props.on_confirm {
            handler.call(());
        }
    };

    let handle_cancel = move |_: MouseEvent| {
        if let Some(handler) = &props.on_cancel {
            handler.call(());
        }
        if let Some(handler) = &props.on_close {
            handler.call(());
        }
    };

    // 键盘事件处理
    let handle_keydown = move |event: KeyboardEvent| {
        if event.key() == Key::Escape && props.mask_closable {
            if let Some(handler) = &props.on_close {
                handler.call(());
            }
        }
    };

    if !is_visible {
        return rsx! { div {} };
    }

    let modal_class = format!(
        "fixed inset-0 z-[9999] flex {} justify-center p-4 {} transition-opacity duration-300",
        props.position.to_class(),
        if props.show_mask { "bg-black bg-opacity-50" } else { "" }
    );

    let content_class = format!(
        "relative w-full {} bg-white dark:bg-gray-800 rounded-lg shadow-xl transform transition-all duration-300 {} {}",
        props.size.to_class(),
        props.class.unwrap_or_default(),
        if props.draggable { "cursor-move" } else { "" }
    );

    rsx! {
        div {
            class: modal_class,
            onclick: handle_mask_click,
            onkeydown: handle_keydown,
            tabindex: "-1",
            
            div {
                class: content_class,
                style: format!("transform-origin: {}", props.transform_origin),
                onclick: move |e| e.stop_propagation(),
                
                // 头部
                if props.title.is_some() || props.closable {
                    div {
                        class: "flex items-center justify-between p-6 border-b border-gray-200 dark:border-gray-700",
                        
                        // 标题和图标
                        div {
                            class: "flex items-center gap-3",
                            
                            // 类型图标
                            if let Some(icon_path) = props.modal_type.icon() {
                                svg {
                                    class: format!("w-6 h-6 {}", props.modal_type.icon_color()),
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: icon_path,
                                    }
                                }
                            }
                            
                            // 标题
                            if let Some(title) = &props.title {
                                h3 {
                                    class: "text-lg font-semibold text-gray-900 dark:text-white",
                                    {title.clone()}
                                }
                            }
                        }
                        
                        // 关闭按钮
                        if props.closable {
                            button {
                                class: "p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors",
                                onclick: handle_close,
                                svg {
                                    class: "w-5 h-5",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M6 18L18 6M6 6l12 12",
                                    }
                                }
                            }
                        }
                    }
                }
                
                // 内容
                div {
                    class: "p-6",
                    {props.children}
                }
                
                // 底部按钮
                if props.show_confirm || props.show_cancel {
                    div {
                        class: "flex justify-end gap-3 p-6 border-t border-gray-200 dark:border-gray-700",
                        
                        if props.show_cancel {
                            button {
                                class: "px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 dark:bg-gray-800 dark:text-gray-300 dark:border-gray-600 dark:hover:bg-gray-700 transition-colors",
                                onclick: handle_cancel,
                                {props.cancel_text}
                            }
                        }
                        
                        if props.show_confirm {
                            button {
                                class: "px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600 transition-colors",
                                onclick: handle_confirm,
                                {props.confirm_text}
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 模态框管理器
#[derive(Clone)]
pub struct ModalManager {
    pub visible: Signal<bool>,
}

impl ModalManager {
    pub fn new() -> Self {
        Self {
            visible: use_signal(|| false),
        }
    }

    pub fn show(&mut self) {
        self.visible.set(true);
    }

    pub fn hide(&mut self) {
        self.visible.set(false);
    }
}

impl Default for ModalManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 命令式模态框 API
pub struct ImperativeModal {
    id: u32,
}

impl ImperativeModal {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    /// 显示基础模态框
    pub fn show_basic(_title: String, _content: String) -> u32 {
        // 这里需要与全局模态框容器集成
        // 暂时返回一个 ID
        1
    }

    /// 显示确认对话框
    pub fn show_confirm(_title: String, _content: String) -> u32 {
        // 这里需要与全局模态框容器集成
        // 暂时返回一个 ID
        2
    }

    /// 关闭模态框
    pub fn close(_id: u32) {
        // 这里需要与全局模态框容器集成
    }
}

/// 全局模态框容器
#[component]
pub fn GlobalModalContainer() -> Element {
    rsx! {
        div {
            // 这里将渲染所有通过命令式 API 创建的模态框
        }
    }
}

use dioxus::prelude::*;
use super::overlay::{PlatformAdapter, Breakpoint};

/// NoticeBase 组件 - 跨平台轻提示类浮层的基础组件
#[derive(Props, Clone, PartialEq)]
pub struct NoticeBaseProps {
    /// 是否显示
    pub visible: bool,
    /// 位置
    #[props(default = "top-right".to_string())]
    pub position: String,
    /// 持续时间（毫秒），0 表示不自动关闭
    #[props(default = 2000)]
    pub duration: u32,
    /// 是否可关闭
    #[props(default = true)]
    pub closable: bool,
    /// 类型
    #[props(default = "info".to_string())]
    pub notice_type: String,
    /// 标题
    pub title: Option<String>,
    /// 内容
    pub content: Option<String>,
    /// 是否支持手势关闭（移动端）
    #[props(default = true)]
    pub gesture_close: bool,
    /// 是否支持震动反馈（移动端）
    #[props(default = false)]
    pub haptic_feedback: bool,
    /// 关闭事件
    pub on_close: Option<EventHandler<()>>,
    /// 自定义样式类
    pub class: Option<String>,
    /// 子组件
    pub children: Element,
}

#[component]
pub fn NoticeBase(props: NoticeBaseProps) -> Element {
    let platform_adapter = PlatformAdapter::new();
    let platform = platform_adapter.get_platform();
    
    // 模拟屏幕宽度检测
    let screen_width = use_signal(|| 1024u32);
    let breakpoint = Breakpoint::from_width(*screen_width.read());
    
    let (position_class, size_class) = get_notice_classes(&platform, &breakpoint, &props);
    
    let type_class = get_type_class(&props.notice_type);
    
    let base_z_index = platform_adapter.get_base_z_index();
    let content_class = format!(
        "fixed {} {} p-4 rounded-lg border shadow-lg {} {} {}",
        position_class,
        size_class,
        type_class,
        if props.haptic_feedback && breakpoint.is_mobile() { "animate-pulse" } else { "" },
        props.class.unwrap_or_default()
    );

    let handle_close = move || {
        if let Some(on_close) = &props.on_close {
            on_close.call(());
        }
    };

    // 手势关闭处理（移动端）
    let handle_gesture_close = move |_event: TouchEvent| {
        if props.gesture_close && breakpoint.is_mobile() {
            // 简单的滑动关闭逻辑
            handle_close();
        }
    };

    // 自动关闭逻辑
    use_effect(move || {
        if props.visible && props.duration > 0 {
            let duration = props.duration;
            let on_close = props.on_close.clone();
            
            // 使用spawn创建定时器
            spawn(async move {
                gloo_timers::future::TimeoutFuture::new(duration).await;
                if let Some(on_close) = on_close {
                    on_close.call(());
                }
            });
        }
    });

    // 震动反馈（移动端）
    use_effect(move || {
        if props.visible && props.haptic_feedback && breakpoint.is_mobile() {
            // 在实际应用中，这里会调用设备的震动 API
            // navigator.vibrate([100]);
        }
    });

    rsx! {
        if props.visible {
            div {
                class: content_class,
                style: format!("z-index: {}", base_z_index),
                onclick: move |e: MouseEvent| {
                    e.stop_propagation();
                },
                ontouchstart: handle_gesture_close,
                
                div {
                    class: "flex items-start",
                    
                    // 图标
                    div { class: "flex-shrink-0 mr-3 text-lg",
                        match props.notice_type.as_str() {
                            "success" => rsx! { "✓" },
                            "warning" => rsx! { "⚠" },
                            "error" => rsx! { "✕" },
                            "loading" => rsx! { 
                                div { 
                                    class: "animate-spin",
                                    "⟳"
                                }
                            },
                            _ => rsx! { "ℹ" },
                        }
                    }
                    
                    // 内容
                    div { class: "flex-1",
                        if let Some(title) = &props.title {
                            div { class: "font-semibold mb-1", "{title}" }
                        }
                        
                        if let Some(content) = &props.content {
                            div { class: "text-sm", "{content}" }
                        }
                        
                        {props.children}
                    }
                    
                    // 关闭按钮
                    if props.closable {
                        button {
                            class: "flex-shrink-0 ml-3 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                            onclick: move |_| handle_close(),
                            "×"
                        }
                    }
                }
            }
        }
    }
}

/// 获取通知的位置和尺寸类名
fn get_notice_classes(_platform: &super::overlay::Platform, breakpoint: &Breakpoint, props: &NoticeBaseProps) -> (String, String) {
    let position_class = match props.position.as_str() {
        "top-left" => "top-4 left-4",
        "top-center" => "top-4 left-1/2 transform -translate-x-1/2",
        "top-right" => "top-4 right-4",
        "bottom-left" => "bottom-4 left-4",
        "bottom-center" => "bottom-4 left-1/2 transform -translate-x-1/2",
        "bottom-right" => "bottom-4 right-4",
        _ => "top-4 right-4",
    };

    let size_class = match breakpoint {
        Breakpoint::Xs => "w-72 max-w-sm",
        Breakpoint::Sm => "w-80 max-w-sm",
        Breakpoint::Md => "w-96 max-w-md",
        Breakpoint::Lg => "w-96 max-w-md",
        Breakpoint::Xl => "w-96 max-w-md",
        Breakpoint::Xxl => "w-96 max-w-md",
    };

    (position_class.to_string(), size_class.to_string())
}

/// 获取类型样式类名
fn get_type_class(notice_type: &str) -> String {
    match notice_type {
        "success" => "bg-green-50 border-green-200 text-green-800 dark:bg-green-900 dark:border-green-700 dark:text-green-200".to_string(),
        "warning" => "bg-yellow-50 border-yellow-200 text-yellow-800 dark:bg-yellow-900 dark:border-yellow-700 dark:text-yellow-200".to_string(),
        "error" => "bg-red-50 border-red-200 text-red-800 dark:bg-red-900 dark:border-red-700 dark:text-red-200".to_string(),
        "loading" => "bg-blue-50 border-blue-200 text-blue-800 dark:bg-blue-900 dark:border-blue-700 dark:text-blue-200".to_string(),
        _ => "bg-blue-50 border-blue-200 text-blue-800 dark:bg-blue-900 dark:border-blue-700 dark:text-blue-200".to_string(),
    }
}

/// NoticeBase 类型枚举（向后兼容）
#[derive(Debug, Clone, PartialEq)]
pub enum NoticeType {
    Info,
    Success,
    Warning,
    Error,
    Loading,
}

impl NoticeType {
    pub fn to_string(&self) -> String {
        match self {
            NoticeType::Info => "info".to_string(),
            NoticeType::Success => "success".to_string(),
            NoticeType::Warning => "warning".to_string(),
            NoticeType::Error => "error".to_string(),
            NoticeType::Loading => "loading".to_string(),
        }
    }
}

/// NoticeBase 位置枚举（向后兼容）
#[derive(Debug, Clone, PartialEq)]
pub enum NoticePosition {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Top,
    Bottom,
}

impl NoticePosition {
    pub fn to_string(&self) -> String {
        match self {
            NoticePosition::TopLeft => "top-left".to_string(),
            NoticePosition::TopCenter => "top-center".to_string(),
            NoticePosition::TopRight => "top-right".to_string(),
            NoticePosition::BottomLeft => "bottom-left".to_string(),
            NoticePosition::BottomCenter => "bottom-center".to_string(),
            NoticePosition::BottomRight => "bottom-right".to_string(),
            NoticePosition::Top => "top-center".to_string(),
            NoticePosition::Bottom => "bottom-center".to_string(),
        }
    }
}

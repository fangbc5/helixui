use dioxus::prelude::*;
use super::overlay::{Overlay, PlatformAdapter, Breakpoint};

/// 响应式尺寸配置
#[derive(Debug, Clone, PartialEq)]
pub struct ResponsiveSize {
    pub xs: Option<String>,
    pub sm: Option<String>,
    pub md: Option<String>,
    pub lg: Option<String>,
    pub xl: Option<String>,
    pub xxl: Option<String>,
}

impl ResponsiveSize {
    pub fn new() -> Self {
        Self {
            xs: None,
            sm: None,
            md: None,
            lg: None,
            xl: None,
            xxl: None,
        }
    }

    pub fn with_size(mut self, size: &str) -> Self {
        self.xs = Some(size.to_string());
        self.sm = Some(size.to_string());
        self.md = Some(size.to_string());
        self.lg = Some(size.to_string());
        self.xl = Some(size.to_string());
        self.xxl = Some(size.to_string());
        self
    }

    pub fn with_responsive(mut self, xs: Option<&str>, sm: Option<&str>, md: Option<&str>, lg: Option<&str>, xl: Option<&str>, xxl: Option<&str>) -> Self {
        self.xs = xs.map(|s| s.to_string());
        self.sm = sm.map(|s| s.to_string());
        self.md = md.map(|s| s.to_string());
        self.lg = lg.map(|s| s.to_string());
        self.xl = xl.map(|s| s.to_string());
        self.xxl = xxl.map(|s| s.to_string());
        self
    }

    pub fn get_size_for_breakpoint(&self, breakpoint: &Breakpoint) -> Option<String> {
        match breakpoint {
            Breakpoint::Xs => self.xs.clone(),
            Breakpoint::Sm => self.sm.clone(),
            Breakpoint::Md => self.md.clone(),
            Breakpoint::Lg => self.lg.clone(),
            Breakpoint::Xl => self.xl.clone(),
            Breakpoint::Xxl => self.xxl.clone(),
        }
    }
}

/// PopupBase 组件 - 跨平台可交互类浮层的基础组件
#[derive(Props, Clone, PartialEq)]
pub struct PopupBaseProps {
    /// 是否显示
    pub visible: bool,
    /// 位置
    #[props(default = "center".to_string())]
    pub position: String,
    /// 响应式尺寸配置
    pub size: Option<ResponsiveSize>,
    /// 是否可拖拽
    #[props(default = false)]
    pub draggable: bool,
    /// 变换原点
    #[props(default = "center".to_string())]
    pub transform_origin: String,
    /// 是否显示遮罩层
    #[props(default = true)]
    pub show_mask: bool,
    /// 遮罩层点击是否关闭
    #[props(default = true)]
    pub mask_closable: bool,
    /// 是否显示关闭按钮
    #[props(default = true)]
    pub closable: bool,
    /// 标题
    pub title: Option<String>,
    /// 是否全屏（移动端）
    #[props(default = false)]
    pub fullscreen_mobile: bool,
    /// 是否支持手势关闭（移动端）
    #[props(default = true)]
    pub gesture_close: bool,
    /// 关闭事件
    pub on_close: Option<EventHandler<()>>,
    /// 自定义样式类
    pub class: Option<String>,
    /// 子组件
    pub children: Element,
}

#[component]
pub fn PopupBase(props: PopupBaseProps) -> Element {
    let platform_adapter = PlatformAdapter::new();
    let platform = platform_adapter.get_platform();
    
    // 模拟屏幕宽度检测（在实际应用中会从窗口大小获取）
    let screen_width = use_signal(|| 1024u32);
    let breakpoint = Breakpoint::from_width(*screen_width.read());
    
    // 根据平台和断点确定位置和尺寸
    let (position_class, size_class) = get_responsive_classes(&platform, &breakpoint, &props);
    
    let content_class = format!(
        "relative bg-white dark:bg-gray-800 rounded-lg shadow-lg {} {} {}",
        size_class,
        if props.draggable { "cursor-move" } else { "" },
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
            // 简单的向下滑动关闭逻辑
            // 在实际应用中需要更复杂的手势识别
            handle_close();
        }
    };

    rsx! {
        Overlay {
            visible: props.visible,
            show_mask: props.show_mask,
            mask_closable: props.mask_closable,
            on_close: props.on_close.clone(),
            content_class: Some(format!("flex {} {}", position_class, content_class)),
            
            div {
                class: "w-full",
                style: format!("transform-origin: {}", props.transform_origin),
                ontouchstart: handle_gesture_close,
                
                // 标题栏
                if props.title.is_some() || props.closable {
                    div { class: "flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700",
                        if let Some(title) = &props.title {
                            h3 { class: "text-lg font-semibold text-gray-900 dark:text-white", "{title}" }
                        }
                        
                        if props.closable {
                            button {
                                class: "text-gray-400 hover:text-gray-600 dark:hover:text-gray-300",
                                onclick: move |_| handle_close(),
                                "×"
                            }
                        }
                    }
                }
                
                // 内容区域
                div { class: "p-4",
                    {props.children}
                }
            }
        }
    }
}

/// 根据平台和断点获取响应式类名
fn get_responsive_classes(_platform: &super::overlay::Platform, breakpoint: &Breakpoint, props: &PopupBaseProps) -> (String, String) {
    let position_class = match props.position.as_str() {
        "top" => "items-start pt-16",
        "bottom" => "items-end pb-16",
        "left" => "justify-start pl-16",
        "right" => "justify-end pr-16",
        _ => "items-center justify-center",
    };

    let size_class = if let Some(size_config) = &props.size {
        size_config.get_size_for_breakpoint(breakpoint)
            .unwrap_or_else(|| get_default_size_class(breakpoint))
    } else {
        get_default_size_class(breakpoint)
    };

    (position_class.to_string(), size_class)
}

/// 获取默认尺寸类名
fn get_default_size_class(breakpoint: &Breakpoint) -> String {
    match breakpoint {
        Breakpoint::Xs => "max-w-xs mx-4".to_string(),
        Breakpoint::Sm => "max-w-sm mx-4".to_string(),
        Breakpoint::Md => "max-w-md".to_string(),
        Breakpoint::Lg => "max-w-lg".to_string(),
        Breakpoint::Xl => "max-w-xl".to_string(),
        Breakpoint::Xxl => "max-w-2xl".to_string(),
    }
}

/// PopupBase 尺寸枚举（向后兼容）
#[derive(Debug, Clone, PartialEq)]
pub enum PopupSize {
    Small,
    Medium,
    Large,
    Full,
    Responsive(ResponsiveSize),
}

impl PopupSize {
    pub fn to_responsive_size(&self) -> ResponsiveSize {
        match self {
            PopupSize::Small => ResponsiveSize::new().with_size("max-w-sm"),
            PopupSize::Medium => ResponsiveSize::new().with_size("max-w-md"),
            PopupSize::Large => ResponsiveSize::new().with_size("max-w-lg"),
            PopupSize::Full => ResponsiveSize::new().with_size("max-w-full"),
            PopupSize::Responsive(size) => size.clone(),
        }
    }
}

/// PopupBase 位置枚举（向后兼容）
#[derive(Debug, Clone, PartialEq)]
pub enum PopupPosition {
    Top,
    Center,
    Bottom,
    Left,
    Right,
}

impl PopupPosition {
    pub fn to_string(&self) -> String {
        match self {
            PopupPosition::Top => "top".to_string(),
            PopupPosition::Center => "center".to_string(),
            PopupPosition::Bottom => "bottom".to_string(),
            PopupPosition::Left => "left".to_string(),
            PopupPosition::Right => "right".to_string(),
        }
    }
}

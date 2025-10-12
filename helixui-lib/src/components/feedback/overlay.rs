use dioxus::prelude::*;

/// 平台类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    Web,
    Desktop,
    Mobile,
    Unknown,
}

/// 平台检测和适配
#[derive(Clone)]
pub struct PlatformAdapter {
    platform: Platform,
}

impl PlatformAdapter {
    pub fn new() -> Self {
        // 在实际应用中，这里会根据编译目标或运行时检测来确定平台
        #[cfg(target_arch = "wasm32")]
        let platform = Platform::Web;
        
        #[cfg(not(target_arch = "wasm32"))]
        let platform = Platform::Desktop;
        
        Self { platform }
    }

    pub fn get_platform(&self) -> Platform {
        self.platform.clone()
    }

    /// 获取平台特定的 z-index 基础值
    pub fn get_base_z_index(&self) -> u32 {
        match self.platform {
            Platform::Web => 1000,
            Platform::Desktop => 10000,
            Platform::Mobile => 2000,
            Platform::Unknown => 1000,
        }
    }

    /// 获取平台特定的动画持续时间
    pub fn get_animation_duration(&self) -> u32 {
        match self.platform {
            Platform::Web => 200,
            Platform::Desktop => 150,
            Platform::Mobile => 250,
            Platform::Unknown => 200,
        }
    }

    /// 获取平台特定的遮罩层样式
    pub fn get_mask_class(&self) -> String {
        match self.platform {
            Platform::Web => "bg-black bg-opacity-50".to_string(),
            Platform::Desktop => "bg-black bg-opacity-30".to_string(),
            Platform::Mobile => "bg-black bg-opacity-60".to_string(),
            Platform::Unknown => "bg-black bg-opacity-50".to_string(),
        }
    }
}

/// 响应式断点
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Breakpoint {
    Xs,    // < 640px
    Sm,    // 640px - 768px
    Md,    // 768px - 1024px
    Lg,    // 1024px - 1280px
    Xl,    // 1280px - 1536px
    Xxl,   // >= 1536px
}

impl Breakpoint {
    pub fn from_width(width: u32) -> Self {
        match width {
            0..=639 => Breakpoint::Xs,
            640..=767 => Breakpoint::Sm,
            768..=1023 => Breakpoint::Md,
            1024..=1279 => Breakpoint::Lg,
            1280..=1535 => Breakpoint::Xl,
            _ => Breakpoint::Xxl,
        }
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Breakpoint::Xs | Breakpoint::Sm)
    }

    pub fn is_tablet(&self) -> bool {
        matches!(self, Breakpoint::Md)
    }

    pub fn is_desktop(&self) -> bool {
        matches!(self, Breakpoint::Lg | Breakpoint::Xl | Breakpoint::Xxl)
    }
}

/// 通用 Overlay 基础组件 - 跨平台兼容
#[derive(Props, Clone, PartialEq)]
pub struct OverlayProps {
    /// 是否显示
    pub visible: bool,
    /// z-index 层级（相对于平台基础值）
    #[props(default = 0)]
    pub z_index_offset: u32,
    /// 是否显示遮罩层
    #[props(default = true)]
    pub show_mask: bool,
    /// 遮罩层点击是否关闭
    #[props(default = true)]
    pub mask_closable: bool,
    /// 遮罩层样式类（可选，使用平台默认值）
    pub mask_class: Option<String>,
    /// 内容容器样式类
    pub content_class: Option<String>,
    /// 是否支持键盘事件
    #[props(default = true)]
    pub keyboard_enabled: bool,
    /// 是否支持触摸事件
    #[props(default = true)]
    pub touch_enabled: bool,
    /// 关闭事件
    pub on_close: Option<EventHandler<()>>,
    /// 遮罩层点击事件
    pub on_mask_click: Option<EventHandler<()>>,
    /// 键盘事件
    pub on_keydown: Option<EventHandler<KeyboardEvent>>,
    /// 子组件
    pub children: Element,
}

#[component]
pub fn Overlay(props: OverlayProps) -> Element {
    let platform_adapter = PlatformAdapter::new();
    let base_z_index = platform_adapter.get_base_z_index();
    let mask_class = props.mask_class.unwrap_or_else(|| platform_adapter.get_mask_class());
    
    let handle_mask_click = move |_event: MouseEvent| {
        if props.mask_closable {
            if let Some(on_close) = &props.on_close {
                on_close.call(());
            }
            if let Some(on_mask_click) = &props.on_mask_click {
                on_mask_click.call(());
            }
        }
    };

    let handle_keydown = move |event: KeyboardEvent| {
        if props.keyboard_enabled {
            if event.key() == Key::Escape {
                if let Some(on_close) = &props.on_close {
                    on_close.call(());
                }
            }
            if let Some(on_keydown) = &props.on_keydown {
                on_keydown.call(event);
            }
        }
    };

    let handle_touch_start = move |event: TouchEvent| {
        if props.touch_enabled {
            // 触摸事件处理逻辑
            event.stop_propagation();
        }
    };

    if !props.visible {
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "fixed inset-0 flex items-center justify-center",
            style: format!("z-index: {}", base_z_index + props.z_index_offset),
            onclick: handle_mask_click,
            onkeydown: handle_keydown,
            ontouchstart: handle_touch_start,
            tabindex: "-1",
            
            // 遮罩层
            if props.show_mask {
                div {
                    class: format!("absolute inset-0 {}", mask_class),
                }
            }
            
            // 内容容器
            div {
                class: props.content_class.unwrap_or_default(),
                onclick: move |e: MouseEvent| {
                    e.stop_propagation();
                },
                ontouchstart: move |e: TouchEvent| {
                    e.stop_propagation();
                },
                
                {props.children}
            }
        }
    }
}

/// Overlay 管理器 - 跨平台 z-index 管理
#[derive(Clone)]
pub struct OverlayManager {
    platform_adapter: PlatformAdapter,
    current_z_index: Signal<u32>,
}

impl OverlayManager {
    pub fn new() -> Self {
        let platform_adapter = PlatformAdapter::new();
        let base_z_index = platform_adapter.get_base_z_index();
        Self {
            platform_adapter,
            current_z_index: use_signal(|| base_z_index),
        }
    }

    /// 获取下一个 z-index
    pub fn next_z_index(&mut self) -> u32 {
        let current = *self.current_z_index.read();
        let next = current + 10; // 每次增加 10，为其他元素留出空间
        self.current_z_index.set(next);
        next
    }

    /// 重置 z-index
    pub fn reset_z_index(&mut self) {
        let base_z_index = self.platform_adapter.get_base_z_index();
        self.current_z_index.set(base_z_index);
    }

    /// 获取平台适配器
    pub fn get_platform_adapter(&self) -> &PlatformAdapter {
        &self.platform_adapter
    }
}

// 全局 Overlay 管理器（暂时注释掉，避免编译错误）
// pub static OVERLAY_MANAGER: Signal<OverlayManager> = Signal::new(OverlayManager::new());

use dioxus::prelude::*;

/// Overlay 错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum OverlayError {
    ZIndexOverflow,
    PlatformNotSupported,
    AnimationFailed,
    StateCorrupted,
    InvalidConfiguration,
    RenderFailed,
    EventHandlerError,
    PluginError(String),
}

impl std::fmt::Display for OverlayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OverlayError::ZIndexOverflow => write!(f, "Z-index overflow: Maximum z-index exceeded"),
            OverlayError::PlatformNotSupported => write!(f, "Platform not supported"),
            OverlayError::AnimationFailed => write!(f, "Animation failed to execute"),
            OverlayError::StateCorrupted => write!(f, "Overlay state is corrupted"),
            OverlayError::InvalidConfiguration => write!(f, "Invalid configuration provided"),
            OverlayError::RenderFailed => write!(f, "Failed to render overlay"),
            OverlayError::EventHandlerError => write!(f, "Event handler error"),
            OverlayError::PluginError(msg) => write!(f, "Plugin error: {}", msg),
        }
    }
}

impl std::error::Error for OverlayError {}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, OverlayError>;

/// 错误处理器
pub struct ErrorHandler {
    pub on_error: Option<Box<dyn Fn(OverlayError) -> Element + Send + Sync>>,
    pub fallback: Element,
    pub log_errors: bool,
}

impl ErrorHandler {
    /// 创建新的错误处理器
    pub fn new() -> Self {
        Self {
            on_error: None,
            fallback: rsx! { div { "An error occurred" } },
            log_errors: true,
        }
    }

    /// 设置错误处理函数
    pub fn with_error_handler<F>(mut self, handler: F) -> Self
    where
        F: Fn(OverlayError) -> Element + Send + Sync + 'static,
    {
        self.on_error = Some(Box::new(handler));
        self
    }

    /// 设置回退元素
    pub fn with_fallback(mut self, fallback: Element) -> Self {
        self.fallback = fallback;
        self
    }

    /// 设置是否记录错误
    pub fn with_logging(mut self, log_errors: bool) -> Self {
        self.log_errors = log_errors;
        self
    }

    /// 处理错误
    pub fn handle_error(&self, error: OverlayError) -> Element {
        if self.log_errors {
            eprintln!("Overlay Error: {}", error);
        }

        if let Some(handler) = &self.on_error {
            handler(error)
        } else {
            self.fallback.clone()
        }
    }

    /// 处理结果
    pub fn handle_result<T>(&self, result: Result<T>) -> Option<T> {
        match result {
            Ok(value) => Some(value),
            Err(error) => {
                self.handle_error(error);
                None
            }
        }
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// 错误边界组件
#[component]
pub fn ErrorBoundary(children: Element) -> Element {
    let error = use_signal(|| None::<OverlayError>);
    let error_handler = use_signal(|| ErrorHandler::new());

    // 在实际应用中，这里会捕获渲染错误
    // 目前只是简单的占位符
    let current_error = error.read().clone();
    if let Some(err) = current_error {
        error_handler.read().handle_error(err)
    } else {
        children
    }
}

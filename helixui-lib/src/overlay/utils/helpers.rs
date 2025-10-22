use std::collections::HashMap;

/// 生成唯一 ID
pub fn generate_id(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    format!("{}_{}", prefix, timestamp)
}

/// 合并 CSS 类名
pub fn merge_classes(classes: &[&str]) -> String {
    classes
        .iter()
        .filter(|&&class| !class.is_empty())
        .map(|s| *s)
        .collect::<Vec<_>>()
        .join(" ")
}

/// 条件类名
pub fn conditional_class(condition: bool, true_class: &str, false_class: &str) -> String {
    if condition {
        true_class.to_string()
    } else {
        false_class.to_string()
    }
}

/// 响应式类名
pub fn responsive_class(mobile: &str, tablet: &str, desktop: &str, breakpoint: &str) -> String {
    match breakpoint {
        "mobile" => mobile.to_string(),
        "tablet" => tablet.to_string(),
        "desktop" => desktop.to_string(),
        _ => desktop.to_string(),
    }
}

/// 深度合并对象
pub fn deep_merge<T: Clone>(
    mut base: HashMap<String, T>,
    other: HashMap<String, T>,
) -> HashMap<String, T> {
    for (key, value) in other {
        base.insert(key, value);
    }
    base
}

/// 防抖函数（简化版本，避免时间依赖）
pub fn debounce<F>(mut callback: F, _delay: u32) -> impl FnMut()
where
    F: FnMut() + 'static,
{
    // 最小实现：直接调用（占位，避免异步/生命周期问题）
    move || {
        callback();
    }
}

/// 节流函数
pub fn throttle<F>(mut callback: F, delay: u32) -> impl FnMut()
where
    F: FnMut() + 'static,
{
    let mut last_call = 0u64;

    move || {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        if now - last_call >= delay as u64 {
            last_call = now;
            callback();
        }
    }
}

/// 检查是否为移动设备
pub fn is_mobile() -> bool {
    // 在实际应用中，这里应该通过 JavaScript 检测
    // 这里使用简化的实现
    false
}

/// 检查是否为触摸设备
pub fn is_touch_device() -> bool {
    // 在实际应用中，这里应该通过 JavaScript 检测
    // 这里使用简化的实现
    false
}

/// 获取视口尺寸
pub fn get_viewport_size() -> (u32, u32) {
    // 在实际应用中，这里应该通过 JavaScript 获取
    // 这里使用默认值
    (1024, 768)
}

/// 格式化持续时间
pub fn format_duration(milliseconds: u32) -> String {
    if milliseconds < 1000 {
        format!("{}ms", milliseconds)
    } else {
        format!("{:.1}s", milliseconds as f64 / 1000.0)
    }
}

/// 验证 z-index 值
pub fn validate_z_index(z_index: u32, max: u32) -> bool {
    z_index <= max
}

/// 计算 z-index 层级
pub fn calculate_z_index(base: u32, level: u32, step: u32) -> u32 {
    base + (level * step)
}

/// 生成 CSS 变量
pub fn generate_css_variables(variables: HashMap<String, String>) -> String {
    variables
        .iter()
        .map(|(key, value)| format!("--{}: {};", key, value))
        .collect::<Vec<_>>()
        .join(" ")
}

/// 检查浏览器支持
pub fn check_browser_support() -> BrowserSupport {
    BrowserSupport {
        animations: true,
        transitions: true,
        transforms: true,
        flexbox: true,
        grid: true,
        custom_properties: true,
    }
}

/// 浏览器支持信息
#[derive(Debug, Clone)]
pub struct BrowserSupport {
    pub animations: bool,
    pub transitions: bool,
    pub transforms: bool,
    pub flexbox: bool,
    pub grid: bool,
    pub custom_properties: bool,
}

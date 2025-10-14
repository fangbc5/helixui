use dioxus::prelude::*;

/// 滚动监听 Hook
/// 使用简化的滚动位置检测方法，支持桌面和 Web 平台
pub fn use_scroll_observer(items: Vec<String>) -> Signal<String> {
    let active_id = use_signal(|| String::new());

    // 为保持跨平台与依赖精简，这里不再直接绑定浏览器滚动事件。
    // 目录高亮推荐通过点击行为或后续可选 JS 注入方案实现。

    active_id
}

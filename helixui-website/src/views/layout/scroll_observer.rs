use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// 滚动监听 Hook
/// 使用简化的滚动位置检测方法
pub fn use_scroll_observer(items: Vec<String>) -> Signal<String> {
    let active_id = use_signal(|| String::new());

    use_effect(move || {
        let items = items.clone();
        let mut active_id = active_id.clone();

        // 使用 spawn 来异步设置滚动监听
        let _ = spawn(async move {
            // 减少延迟，提高响应性
            gloo_timers::future::TimeoutFuture::new(20).await;

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    // 创建滚动监听回调
                    let callback = Closure::wrap(Box::new(move || {
                        let mut best_id = String::new();
                        let mut best_distance = f64::MAX;

                        for item_id in &items {
                            if let Some(element) = document.get_element_by_id(item_id) {
                                let rect = element.get_bounding_client_rect();
                                let element_top = rect.top();
                                let element_bottom = rect.bottom();

                                // 计算元素到视口顶部的距离
                                let distance = element_top.abs();

                                // 如果元素在视口内且距离更近，则选择它
                                if element_top <= 120.0
                                    && element_bottom > 0.0
                                    && distance < best_distance
                                {
                                    best_distance = distance;
                                    best_id = item_id.clone();
                                }
                            }
                        }

                        if !best_id.is_empty() {
                            active_id.set(best_id);
                        }
                    }) as Box<dyn FnMut()>);

                    // 添加滚动事件监听器
                    if let Ok(_) = window.add_event_listener_with_callback(
                        "scroll",
                        callback.as_ref().unchecked_ref(),
                    ) {
                        // 存储回调以便后续清理
                        std::mem::forget(callback);
                    }
                }
            }
        });
    });

    active_id
}

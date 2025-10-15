use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContentRect {
    pub width: f32,
    pub height: f32,
}

pub fn use_resize_observer(_cx: &ScopeState, _node: Signal<Element>) -> Signal<ContentRect> {
    let rect = use_signal(|| ContentRect {
        width: 0.0,
        height: 0.0,
    });
    let _ = _node; // 占位：后续基于 web/desktop 平台分别实现
    rect
}

use dioxus::{core::ScopeState, prelude::*};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DragAxis {
    Horizontal,
    Vertical,
    Both,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DragState {
    pub dx: f32,
    pub dy: f32,
}

#[derive(Props, PartialEq, Clone)]
pub struct UseDragProps {
    #[props(default = DragAxis::Both)]
    pub axis: DragAxis,
    #[props(optional)]
    pub on_start: Option<EventHandler<()>>,
    #[props(optional)]
    pub on_move: Option<EventHandler<DragState>>,
    #[props(optional)]
    pub on_end: Option<EventHandler<()>>,
}

pub struct DragBind;

pub fn use_drag(_cx: &ScopeState, props: UseDragProps) -> (Signal<bool>, DragBind) {
    let dragging = use_signal(|| false);
    let _ = props; // 占位，后续补充 web 事件绑定实现
    (dragging, DragBind)
}

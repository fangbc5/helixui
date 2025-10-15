use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BreakpointState {
    pub current: &'static str, // sm/md/lg/xl/xxl
}

impl Default for BreakpointState {
    fn default() -> Self {
        Self { current: "md" }
    }
}

#[derive(Clone)]
pub struct BreakpointContext(pub Signal<BreakpointState>);

#[derive(Clone, Debug, PartialEq)]
pub struct GutterContext {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
pub struct GutterProvider(pub Signal<GutterContext>);

#[derive(Clone, Debug, PartialEq)]
pub struct LayoutState {
    pub has_sider: bool,
    pub is_rtl: bool,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            has_sider: false,
            is_rtl: false,
        }
    }
}

#[derive(Clone)]
pub struct LayoutContext(pub Signal<LayoutState>);

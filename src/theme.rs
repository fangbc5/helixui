use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Theme::Light => "浅色",
            Theme::Dark => "深色",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Theme::Light => "☀️",
            Theme::Dark => "🌙",
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Light
    }
}

/// 全局主题状态
pub static THEME: GlobalSignal<Theme> = Signal::global(Theme::default);

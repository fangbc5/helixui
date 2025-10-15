use super::hooks::use_breakpoint::use_breakpoint;
use super::theme::use_theme;
use super::tokens::SpacingToken;
use super::utils::{calc_gap, ResponsiveSize};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpaceDirection {
    Horizontal,
    Vertical,
}

impl std::fmt::Display for SpaceDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpaceDirection::Horizontal => write!(f, "row"),
            SpaceDirection::Vertical => write!(f, "column"),
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SpaceProps {
    #[props(default = SpaceDirection::Horizontal)]
    pub direction: SpaceDirection,
    #[props(optional)]
    pub size: Option<i32>, // px 或者使用 spacing_token
    #[props(optional)]
    pub spacing_token: Option<SpacingToken>,
    #[props(optional)]
    pub responsive_size: Option<ResponsiveSize>, // 响应式尺寸
    #[props(default = false)]
    pub wrap: bool,
    #[props(optional)]
    pub split: Option<String>, // 分隔符文本
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    children: Element,
}

// ResponsiveSize 现在在 utils 中定义

#[allow(non_snake_case)]
pub fn Space(props: SpaceProps) -> Element {
    let theme = use_theme();
    let current_breakpoint = use_breakpoint();

    // 使用工具函数计算间距
    let gap = calc_gap(
        props.size,
        props.spacing_token,
        props.responsive_size.as_ref(),
        &theme,
        &current_breakpoint,
    );

    // 优化的样式计算
    let flex_direction = props.direction.to_string();
    let gap_style = match props.direction {
        SpaceDirection::Horizontal => format!("column-gap:{}px;", gap),
        SpaceDirection::Vertical => format!("row-gap:{}px;", gap),
    };

    let wrap = if props.wrap { "wrap" } else { "nowrap" };
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();

    // 如果有分隔符，使用特殊渲染逻辑
    if let Some(split_text) = &props.split {
        rsx! {
            div {
                class: format!("hx-space hx-space-with-split {}", class),
                style: format!("display:flex;flex-direction:{};flex-wrap:{};align-items:center;{}", flex_direction, wrap, style),
                // 使用 CSS 变量来传递分隔符文本
                "style": format!("--split-text: '{}'; display:flex;flex-direction:{};flex-wrap:{};align-items:center;{}", split_text, flex_direction, wrap, style),
                {props.children}
            }
        }
    } else {
        rsx! {
            div {
                class: format!("hx-space {}", class),
                style: format!("display:flex;flex-direction:{};flex-wrap:{};{};{}", flex_direction, wrap, gap_style, style),
                {props.children}
            }
        }
    }
}

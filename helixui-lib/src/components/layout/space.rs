use super::flex::{Flex, FlexDirection};
use super::hooks::use_breakpoint::use_breakpoint;
use super::theme::use_theme;
use super::tokens::{Breakpoint, SpacingToken, ThemeTokens};
use super::utils::{calc_gap, gap_to_tailwind_class, ResponsiveSize};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpaceDirection {
    Horizontal,
    Vertical,
}

// 移除未使用的 Display trait 实现

/// Space 尺寸类型
#[derive(Clone, PartialEq, Debug)]
pub enum SpaceSize {
    /// 单一数值（同时应用于水平和垂直）
    Single(i32),
    /// 分别指定水平和垂直间距 (horizontal, vertical)
    Pair(i32, i32),
}

impl From<i32> for SpaceSize {
    fn from(value: i32) -> Self {
        SpaceSize::Single(value)
    }
}

impl From<(i32, i32)> for SpaceSize {
    fn from(value: (i32, i32)) -> Self {
        SpaceSize::Pair(value.0, value.1)
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SpaceProps {
    #[props(default = SpaceDirection::Horizontal)]
    pub direction: SpaceDirection,
    #[props(optional)]
    pub size: Option<SpaceSize>, // 支持单一数值或 (h, v) 对
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

/// 解析间距值，返回 (水平间距, 垂直间距)
fn resolve_space_gap(
    size: &Option<SpaceSize>,
    spacing_token: Option<SpacingToken>,
    responsive_size: Option<&ResponsiveSize>,
    theme_tokens: &ThemeTokens,
    breakpoint: &Breakpoint,
) -> (i32, i32) {
    match size {
        Some(SpaceSize::Single(v)) => (*v, *v),
        Some(SpaceSize::Pair(h, v)) => (*h, *v),
        None => {
            // 优化：避免不必要的函数调用
            let gap = calc_gap(
                None,
                spacing_token,
                responsive_size,
                theme_tokens,
                breakpoint,
            );
            (gap, gap)
        }
    }
}

#[allow(non_snake_case)]
pub fn Space(props: SpaceProps) -> Element {
    let theme = use_theme();
    let current_breakpoint = Breakpoint::from_str(&use_breakpoint());

    let (horizontal_gap, vertical_gap) = resolve_space_gap(
        &props.size,
        props.spacing_token,
        props.responsive_size.as_ref(),
        &theme.tokens,
        &current_breakpoint,
    );

    let flex_direction = match props.direction {
        SpaceDirection::Horizontal => FlexDirection::Row,
        SpaceDirection::Vertical => FlexDirection::Column,
    };

    // 优化：使用更简洁的类名构建
    let gap_value = match props.direction {
        SpaceDirection::Horizontal => horizontal_gap,
        SpaceDirection::Vertical => vertical_gap,
    };
    let gap_class = gap_to_tailwind_class(gap_value);

    // 类名集合 - 使用更简洁的方式
    let mut classes = vec!["hx-space"];
    if let Some(custom_class) = &props.class {
        classes.push(custom_class);
    }

    // 只有在没有分隔符时才添加 gap 类
    if props.split.is_none() && !gap_class.starts_with("gap-[") {
        classes.push(&gap_class);
    }

    // 内联样式，仅用于非标准 gap
    let mut style = props.style.unwrap_or_default();
    if props.split.is_none() && gap_class.starts_with("gap-[") {
        match props.direction {
            SpaceDirection::Horizontal => {
                style.push_str(&format!("column-gap:{}px;", horizontal_gap));
            }
            SpaceDirection::Vertical => {
                style.push_str(&format!("row-gap:{}px;", vertical_gap));
            }
        }
    }

    // 分隔符实现：使用纯 Tailwind 类
    if let Some(split_text) = &props.split {
        // 为分隔符添加特殊类名
        classes.push("hx-space-with-split");

        // 根据分隔符文本选择对应的 Tailwind 类
        let split_class = match split_text.as_str() {
            "|" => "before:content-['|'] before:mx-2 before:text-gray-400",
            "/" => "before:content-['/'] before:mx-2 before:text-gray-400",
            "-" => "before:content-['-'] before:mx-2 before:text-gray-400",
            "•" => "before:content-['•'] before:mx-2 before:text-gray-400",
            _ => "before:content-['|'] before:mx-2 before:text-gray-400", // 默认使用 |
        };

        rsx! {
            div {
                class: classes.join(" "),
                style: if style.is_empty() { None } else { Some(style) },
                role: "group",
                "aria-label": "Space container with splitter",
                Flex {
                    direction: flex_direction,
                    wrap: props.wrap,
                    class: format!("hx-space-split {}", split_class),
                    {props.children}
                }
            }
        }
    } else {
        rsx! {
            div {
                class: classes.join(" "),
                style: if style.is_empty() { None } else { Some(style) },
                role: "group",
                "aria-label": "Space container",
                Flex {
                    direction: flex_direction,
                    wrap: props.wrap,
                    {props.children}
                }
            }
        }
    }
}

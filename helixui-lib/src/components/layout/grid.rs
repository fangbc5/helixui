use super::tokens::Breakpoint;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct GridProps {
    #[props(default = 24)]
    pub cols: u16,
    #[props(optional)]
    pub x_gap: Option<i32>,
    #[props(optional)]
    pub y_gap: Option<i32>,
    #[props(optional)]
    pub responsive_cols: Option<std::collections::HashMap<Breakpoint, u16>>, // 断点->列数
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    children: Element,
}

#[allow(non_snake_case)]
pub fn Grid(props: GridProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let style = props.style.clone().unwrap_or_default();

    let x_gap = props.x_gap.unwrap_or(0);
    let y_gap = props.y_gap.unwrap_or(0);

    // 基于 CSS Grid 的简化实现（列宽 = 1fr * span）
    let style_inline = format!(
        "display:grid;grid-template-columns:repeat({}, minmax(0, 1fr));column-gap:{}px;row-gap:{}px;{}",
        props.cols, x_gap, y_gap, style
    );

    rsx! {
        div { class: format!("hx-grid {}", class), style: style_inline,
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct GridItemProps {
    #[props(optional)]
    pub span: Option<u16>,
    #[props(optional)]
    pub offset: Option<u16>,
    #[props(optional)]
    pub responsive: Option<std::collections::HashMap<Breakpoint, (u16, u16)>>, // 断点->(span, offset)
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    children: Element,
}

#[allow(non_snake_case)]
pub fn GridItem(props: GridItemProps) -> Element {
    let class = props.class.clone().unwrap_or_default();
    let style = props.style.clone().unwrap_or_default();

    let span = props.span.unwrap_or(1);
    let offset = props.offset.unwrap_or(0);

    // 简化：以行内样式输出 span/offset，对齐 Naive 的语义
    let mut style_inline = format!("grid-column: {} / span {};", offset + 1, span);
    if !style.is_empty() {
        style_inline.push_str(&style);
    }

    rsx! {
        div { class: format!("hx-grid-item {}", class), style: style_inline,
            {props.children}
        }
    }
}

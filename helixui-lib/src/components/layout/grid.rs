use super::tokens::Breakpoint;
use super::utils::{
    cols_to_tailwind_class, gap_to_tailwind_class, generate_grid_column_style,
    generate_grid_row_style, generate_responsive_classes,
};
use dioxus::prelude::*;
use std::collections::BTreeMap;

/// Grid 容器属性
#[derive(Props, PartialEq, Clone)]
pub struct GridProps {
    #[props(default = 24)]
    pub cols: u16,
    #[props(optional)]
    pub x_gap: Option<i32>,
    #[props(optional)]
    pub y_gap: Option<i32>,
    #[props(optional)]
    pub responsive_cols: Option<BTreeMap<Breakpoint, u16>>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    children: Element,
}

/// Grid 容器组件
#[allow(non_snake_case)]
pub fn Grid(props: GridProps) -> Element {
    let x_gap = props.x_gap.unwrap_or(0);
    let y_gap = props.y_gap.unwrap_or(0);

    let mut classes = vec![
        "hx-grid".to_string(),
        "grid".to_string(),
        generate_responsive_classes(
            &cols_to_tailwind_class(props.cols),
            &props.responsive_cols,
            |c| cols_to_tailwind_class(*c),
        ),
        if x_gap == y_gap {
            gap_to_tailwind_class(x_gap)
        } else {
            let mut gap_classes = vec![];
            if x_gap > 0 {
                gap_classes.push(format!("gap-x-[{}px]", x_gap));
            }
            if y_gap > 0 {
                gap_classes.push(format!("gap-y-[{}px]", y_gap));
            }
            gap_classes.join(" ")
        },
    ];

    if let Some(custom) = &props.class {
        classes.push(custom.clone());
    }

    let mut styles = vec![];
    if !matches!(props.cols, 1..=24) {
        styles.push(format!(
            "grid-template-columns:repeat({}, minmax(0, 1fr))",
            props.cols
        ));
    }
    if let Some(custom_style) = &props.style {
        styles.push(custom_style.clone());
    }

    rsx! {
        div {
            class: classes.join(" "),
            style: if styles.is_empty() { None } else { Some(styles.join(";")) },
            {props.children}
        }
    }
}

/// GridItem 属性
#[derive(Props, PartialEq, Clone)]
pub struct GridItemProps {
    #[props(optional)]
    pub span: Option<u16>,
    #[props(optional)]
    pub offset: Option<u16>,
    #[props(optional)]
    pub row: Option<u16>,
    #[props(optional)]
    pub row_span: Option<u16>,
    #[props(optional)]
    pub col: Option<u16>,
    #[props(optional)]
    pub responsive: Option<BTreeMap<Breakpoint, (u16, u16, Option<u16>, Option<u16>, Option<u16>)>>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    children: Element,
}

/// GridItem 组件
#[allow(non_snake_case)]
pub fn GridItem(props: GridItemProps) -> Element {
    let mut styles = vec![
        generate_grid_column_style(props.col, props.span, props.offset),
        generate_grid_row_style(props.row, props.row_span),
    ];

    if let Some(resp) = &props.responsive {
        for (bp, (span, offset, row, row_span, col)) in resp {
            let s = vec![
                generate_grid_column_style(*col, Some(*span), Some(*offset)),
                generate_grid_row_style(*row, *row_span),
            ];
            styles.push(format!("{}{}", bp.to_tailwind_prefix(), s.join("")))
        }
    }

    if let Some(custom) = &props.style {
        styles.push(custom.clone());
    }

    let mut classes = vec!["hx-grid-item".to_string()];
    if let Some(custom) = &props.class {
        classes.push(custom.clone());
    }

    rsx! {
        div {
            class: classes.join(" "),
            style: if styles.is_empty() { None } else { Some(styles.join(";")) },
            {props.children}
        }
    }
}

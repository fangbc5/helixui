use super::utils::gap_to_tailwind_class;
use dioxus::prelude::*;

/// Flex 布局方向
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
}

/// 主轴对齐方式
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// 交叉轴对齐方式
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

#[derive(Props, PartialEq, Clone)]
pub struct FlexProps {
    #[props(default = FlexDirection::Row)]
    pub direction: FlexDirection,
    #[props(default = false)]
    pub wrap: bool,
    #[props(optional)]
    pub justify: Option<Justify>,
    #[props(optional)]
    pub align: Option<AlignItems>,
    #[props(optional)]
    pub gap: Option<i32>,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    children: Element,
}

#[allow(non_snake_case)]
pub fn Flex(props: FlexProps) -> Element {
    let mut classes = vec!["flex".to_string()];

    // 布局方向
    classes.push(
        match props.direction {
            FlexDirection::Row => "flex-row",
            FlexDirection::Column => "flex-col",
        }
        .to_string(),
    );

    // 换行
    classes.push(
        if props.wrap {
            "flex-wrap"
        } else {
            "flex-nowrap"
        }
        .to_string(),
    );

    // 主轴对齐
    if let Some(j) = props.justify {
        let jc = match j {
            Justify::Start => "justify-start",
            Justify::Center => "justify-center",
            Justify::End => "justify-end",
            Justify::SpaceBetween => "justify-between",
            Justify::SpaceAround => "justify-around",
            Justify::SpaceEvenly => "justify-evenly",
        };
        classes.push(jc.to_string());
    }

    // 交叉轴对齐
    if let Some(a) = props.align {
        let ac = match a {
            AlignItems::Start => "items-start",
            AlignItems::Center => "items-center",
            AlignItems::End => "items-end",
            AlignItems::Stretch => "items-stretch",
            AlignItems::Baseline => "items-baseline",
        };
        classes.push(ac.to_string());
    }

    // 间距
    let mut style = props.style.unwrap_or_default();
    if let Some(g) = props.gap {
        let gap_class = gap_to_tailwind_class(g);
        if gap_class.starts_with("gap-[") {
            // 自定义间距，使用内联样式
            style = format!("gap:{}px;{}", g, style);
        } else {
            // 标准间距，使用类名
            classes.push(gap_class);
        }
    }

    // 自定义类名
    if let Some(c) = props.class {
        classes.push(c);
    }

    rsx! {
        div {
            class: classes.join(" "),
            style: if style.is_empty() { None } else { Some(style) },
            {props.children}
        }
    }
}

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

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
    let dir = match props.direction {
        FlexDirection::Row => "row",
        FlexDirection::Column => "column",
    };
    let wrap = if props.wrap { "wrap" } else { "nowrap" };
    let class = props.class.unwrap_or_default();
    let mut style = props.style.unwrap_or_default();
    if let Some(j) = props.justify {
        let v = match j {
            Justify::Start => "flex-start",
            Justify::Center => "center",
            Justify::End => "flex-end",
            Justify::SpaceBetween => "space-between",
            Justify::SpaceAround => "space-around",
            Justify::SpaceEvenly => "space-evenly",
        };
        style = format!("justify-content:{};{}", v, style);
    }
    if let Some(a) = props.align {
        let v = match a {
            AlignItems::Start => "flex-start",
            AlignItems::Center => "center",
            AlignItems::End => "flex-end",
            AlignItems::Stretch => "stretch",
            AlignItems::Baseline => "baseline",
        };
        style = format!("align-items:{};{}", v, style);
    }
    if let Some(g) = props.gap {
        style = format!("gap:{}px;{}", g, style);
    }
    rsx! {
        div { class: format!("hx-flex {}", class), style: format!("display:flex;flex-direction:{};flex-wrap:{};{}", dir, wrap, style),
            {props.children}
        }
    }
}

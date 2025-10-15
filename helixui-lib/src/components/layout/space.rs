use super::theme::use_theme;
use super::tokens::SpacingToken;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpaceDirection {
    Horizontal,
    Vertical,
}

#[derive(Props, PartialEq, Clone)]
pub struct SpaceProps {
    #[props(default = SpaceDirection::Horizontal)]
    pub direction: SpaceDirection,
    #[props(optional)]
    pub size: Option<i32>, // px 或者使用 spacing_token
    #[props(optional)]
    pub spacing_token: Option<SpacingToken>,
    #[props(default = false)]
    pub wrap: bool,
    #[props(optional)]
    pub class: Option<String>,
    #[props(optional)]
    pub style: Option<String>,
    children: Element,
}

#[allow(non_snake_case)]
pub fn Space(props: SpaceProps) -> Element {
    let theme = use_theme();
    let gap = props
        .size
        .unwrap_or_else(|| match props.spacing_token.unwrap_or(SpacingToken::Sm) {
            SpacingToken::Xs => theme.spacing_px("xs") as i32,
            SpacingToken::Sm => theme.spacing_px("sm") as i32,
            SpacingToken::Md => theme.spacing_px("md") as i32,
            SpacingToken::Lg => theme.spacing_px("lg") as i32,
            SpacingToken::Xl => theme.spacing_px("xl") as i32,
            SpacingToken::Xxl => theme.spacing_px("xxl") as i32,
        });
    let (row_gap, col_gap) = match props.direction {
        SpaceDirection::Horizontal => (0, gap),
        SpaceDirection::Vertical => (gap, 0),
    };
    let wrap = if props.wrap { "wrap" } else { "nowrap" };
    let class = props.class.unwrap_or_default();
    let style = props.style.unwrap_or_default();
    rsx! {
        div { class: format!("hx-space {}", class), style: format!("display:flex;flex-wrap:{};row-gap:{}px;column-gap:{}px;{}", wrap, row_gap, col_gap, style),
            {props.children}
        }
    }
}

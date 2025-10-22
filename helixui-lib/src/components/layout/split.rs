use dioxus::prelude::*;

/// 分割面板方向
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    Row,    // 水平分割
    Column, // 垂直分割
}

/// Split 组件属性
#[derive(Props, PartialEq, Clone)]
pub struct SplitProps {
    #[props(default = SplitDirection::Row)]
    pub direction: SplitDirection,

    #[props(default = vec![300.0, 300.0])]
    pub initial_sizes: Vec<f64>,

    #[props(default = vec![100.0, 100.0])]
    pub min_sizes: Vec<f64>,

    #[props(default = false)]
    pub disabled: bool,

    #[props(optional)]
    pub class: Option<String>,

    #[props(optional)]
    pub style: Option<String>,

    panel1: Element,
    panel2: Element,
}

#[allow(non_snake_case)]
pub fn Split(props: SplitProps) -> Element {
    let is_horizontal = matches!(props.direction, SplitDirection::Row);

    // 状态
    let mut sizes = use_signal(|| props.initial_sizes.clone());
    let mut dragging = use_signal(|| false);
    let mut start_pos = use_signal(|| 0.0f64);
    let mut start_sizes = use_signal(|| props.initial_sizes.clone());

    // 类名
    let mut classes = vec![
        "flex w-full h-full overflow-hidden select-none",
        if is_horizontal {
            "flex-row"
        } else {
            "flex-col"
        },
        "hx-split",
    ];

    if let Some(custom) = &props.class {
        classes.push(custom);
    }

    // 构建
    rsx! {
        div {
            class: classes.join(" "),
            style: props.style.clone(),
            onmousemove: move |evt: MouseEvent| {
                if props.disabled || !*dragging.read() {
                    return;
                }
                let current_pos = if is_horizontal {
                    evt.page_coordinates().x as f64
                } else {
                    evt.page_coordinates().y as f64
                };
                let delta = current_pos - *start_pos.read();
                let base = start_sizes.read();
                let total = base[0] + base[1];

                let mut s1 = base[0] + delta;
                let mut s2 = base[1] - delta;

                let min1 = props.min_sizes.get(0).unwrap_or(&100.0);
                let min2 = props.min_sizes.get(1).unwrap_or(&100.0);

                if s1 < *min1 {
                    s1 = *min1;
                    s2 = total - s1;
                }
                if s2 < *min2 {
                    s2 = *min2;
                    s1 = total - s2;
                }
                sizes.set(vec![s1, s2]);
            },
            onmouseup: move |_evt: MouseEvent| {
                dragging.set(false);
            },
            onmouseleave: move |_evt: MouseEvent| {
                dragging.set(false);
            },

            // 左 / 上 面板
            div {
                class: "flex-[0_0_auto] overflow-auto",
                style: format!("flex-basis:{:.1}px;", sizes()[0]),
                {props.panel1}
            }

            // 分割线
            div {
                class: format!(
                    "{} {} {}",
                    "flex-shrink-0 z-10 bg-gray-300",
                    if is_horizontal { "w-1" } else { "h-1" },
                    if props.disabled {
                        "opacity-50"
                    } else {
                        if is_horizontal {
                            "hover:bg-gray-400 cursor-col-resize"
                        } else {
                            "hover:bg-gray-400 cursor-row-resize"
                        }
                    },
                ),
                onmousedown: move |evt| {
                    if props.disabled { return; }
                    dragging.set(true);
                    start_pos.set(if is_horizontal { evt.page_coordinates().x as f64 } else { evt.page_coordinates().y as f64 });
                    start_sizes.set(sizes());
                    evt.prevent_default();
                },
            }

            // 右 / 下 面板
            div {
                class: "flex-[1_1_auto] overflow-auto",
                style: format!("flex-basis:{:.1}px;", sizes()[1]),
                {props.panel2}
            }
        }
    }
}

use dioxus::prelude::*;

/// 分割面板方向
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// 水平分割（左右布局）
    Row,
    /// 垂直分割（上下布局）
    Column,
}

/// Split 组件属性 - 可拖拽的分割面板
#[derive(Props, PartialEq, Clone)]
pub struct SplitProps {
    /// 分割方向
    #[props(default = SplitDirection::Row)]
    pub direction: SplitDirection,
    /// 初始尺寸（像素），默认为 [300, 300]
    #[props(default = vec![300.0, 300.0])]
    pub initial_sizes: Vec<f64>,
    /// 最小尺寸（像素），默认为 [10, 10]
    #[props(default = vec![10.0, 10.0])]
    pub min_sizes: Vec<f64>,
    /// 是否禁用拖拽
    #[props(default = false)]
    pub disabled: bool,
    /// 自定义类名
    #[props(optional)]
    pub class: Option<String>,
    /// 自定义样式
    #[props(optional)]
    pub style: Option<String>,
    /// 左/上面板
    panel1: Element,
    /// 右/下面板
    panel2: Element,
}

/// Split 组件 - 可拖拽的分割面板
#[allow(non_snake_case)]
pub fn Split(props: SplitProps) -> Element {
    let is_horizontal = matches!(props.direction, SplitDirection::Row);

    // 状态：面板比例与拖拽信息
    let sizes = use_signal(|| props.initial_sizes.clone());
    let mut dragging = use_signal(|| false);
    let mut start_pos = use_signal(|| 0.0f64);
    let mut start_sizes = use_signal(|| props.initial_sizes.clone());

    // 构建类名
    let mut classes = vec![
        "flex",
        if is_horizontal {
            "flex-row"
        } else {
            "flex-col"
        },
        "hx-split",
    ];

    if props.disabled {
        classes.push("hx-split-disabled");
    }

    if let Some(custom_class) = &props.class {
        classes.push(custom_class);
    }

    // 事件处理器
    let on_mouse_move = {
        let mut sizes = sizes.clone();
        let dragging = dragging.clone();
        let start_pos = start_pos.clone();
        let start_sizes = start_sizes.clone();
        let min_sizes = props.min_sizes.clone();
        let disabled = props.disabled;
        move |evt: MouseEvent| {
            if disabled || !*dragging.read() {
                return;
            }

            let current_pos = if is_horizontal {
                evt.page_coordinates().x as f64
            } else {
                evt.page_coordinates().y as f64
            };

            let delta = current_pos - *start_pos.read();
            let base_sizes = start_sizes.read().clone();

            if base_sizes.len() < 2 {
                return;
            }

            let total_size = base_sizes[0] + base_sizes[1];
            let min1 = *min_sizes.get(0).unwrap_or(&10.0);
            let min2 = *min_sizes.get(1).unwrap_or(&10.0);

            let mut new_s1 = base_sizes[0] + delta;
            let mut new_s2 = base_sizes[1] - delta;

            // 应用最小尺寸限制
            if new_s1 < min1 {
                new_s1 = min1;
                new_s2 = total_size - new_s1;
            }
            if new_s2 < min2 {
                new_s2 = min2;
                new_s1 = total_size - new_s2;
            }

            sizes.set(vec![new_s1, new_s2]);
        }
    };

    let on_mouse_up = {
        let mut dragging = dragging.clone();
        move |_| {
            dragging.set(false);
        }
    };

    rsx! {
        div {
            class: classes.join(" "),
            style: props.style.clone(),
            onmousemove: on_mouse_move,
            onmouseup: on_mouse_up,
            role: "group",
            "aria-label": "Split panel container",

            // 面板 1
            div {
                class: "flex-[0_0_auto] overflow-hidden",
                style: format!("flex-basis:{:.0}px;", sizes().get(0).copied().unwrap_or(300.0)),
                role: "region",
                "aria-label": "Panel 1",
                {props.panel1}
            }

            // 分割线（禁用时仍展示，但不可拖拽）
            div {
                class: if props.disabled {
                    if is_horizontal {
                        "flex-shrink-0 z-10 w-1 bg-gray-300 cursor-not-allowed opacity-50 select-none"
                    } else {
                        "flex-shrink-0 z-10 h-1 bg-gray-300 cursor-not-allowed opacity-50 select-none"
                    }
                } else {
                    if is_horizontal {
                        "flex-shrink-0 z-10 w-1 bg-gray-300 hover:bg-gray-400 cursor-col-resize select-none"
                    } else {
                        "flex-shrink-0 z-10 h-1 bg-gray-300 hover:bg-gray-400 cursor-row-resize select-none"
                    }
                },
                role: "separator",
                "aria-orientation": if is_horizontal { "vertical" } else { "horizontal" },
                onmousedown: move |evt| {
                    if props.disabled { return; }
                    dragging.set(true);
                    start_pos.set(if is_horizontal { evt.page_coordinates().x as f64 } else { evt.page_coordinates().y as f64 });
                    start_sizes.set(sizes());
                    evt.prevent_default();
                }
            }

            // 面板 2
            div {
                class: "flex-[0_0_auto] overflow-hidden",
                style: format!("flex-basis:{:.0}px;", sizes().get(1).copied().unwrap_or(300.0)),
                role: "region",
                "aria-label": "Panel 2",
                {props.panel2}
            }
        }
    }
}

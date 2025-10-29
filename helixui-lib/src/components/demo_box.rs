use dioxus::prelude::*;
use dioxus_time::{use_timeout, TimeoutHandle};
use std::time::Duration;

/// 演示框组件 - 用于展示组件示例和代码
#[component]
pub fn DemoBox(
    /// 示例标题
    #[props(default = String::new())]
    title: String,
    /// 示例描述
    #[props(default = String::new())]
    description: String,
    /// 代码示例
    code: String,
    /// 示例内容
    children: Element,
) -> Element {
    let mut show_code = use_signal(|| false);
    let mut copied = use_signal(|| false);
    let mut current_timeout: Signal<Option<TimeoutHandle>> = use_signal(|| None);

    // 2 秒超时后重置 copied 状态
    let reset_timeout = {
        let mut copied = copied.clone();
        let mut current_timeout = current_timeout.clone();
        use_timeout(Duration::from_millis(2000), move |()| {
            copied.set(false);
            current_timeout.set(None);
        })
    };

    rsx! {
        div {
            class: "mb-8 border border-gray-200 dark:border-gray-700 rounded-lg transition-colors",

            // 标题和描述
            if !title.is_empty() || !description.is_empty() {
                div {
                    class: "px-6 pt-6 pb-2",
                    if !title.is_empty() {
                        h3 {
                            class: "text-lg font-semibold text-gray-900 dark:text-white mb-2",
                            "{title}"
                        }
                    }
                    if !description.is_empty() {
                        p {
                            class: "text-sm text-gray-600 dark:text-gray-400",
                            "{description}"
                        }
                    }
                }
            }

            // 示例展示区域
            div {
                class: "p-6 bg-white dark:bg-gray-800 transition-colors",
                {children}
            }

            // 分隔线
            div {
                class: "border-t border-gray-200 dark:border-gray-700"
            }

            // 代码切换按钮
            div {
                class: "px-6 py-3 bg-gray-50 dark:bg-gray-900 flex items-center justify-between transition-colors",

                button {
                    class: "flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors",
                    onclick: move |_| {
                        show_code.set(!show_code());
                    },

                    // 代码图标
                    svg {
                        class: "w-4 h-4",
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            d: "M16 18L22 12L16 6M8 6L2 12L8 18"
                        }
                    }

                    span {
                        if *show_code.read() {
                            "隐藏代码"
                        } else {
                            "显示代码"
                        }
                    }

                    // 展开/收起图标
                    svg {
                        class: if *show_code.read() { "w-4 h-4 transform rotate-180 transition-transform" } else { "w-4 h-4 transition-transform" },
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "currentColor",
                        path {
                            d: "M7.41 8.59L12 13.17l4.59-4.58L18 10l-6 6-6-6 1.41-1.41z"
                        }
                    }
                }

                // 复制按钮
                button {
                    class: "flex items-center gap-1 text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors",
                    onclick: move |_| {
                        let _code_to_copy = code.clone();
                        copied.set(true);
                        // 如有正在等待的超时，先取消
                        if let Some(handle) = *current_timeout.read() {
                            handle.cancel();
                        }
                        // 触发新的超时
                        let handle = reset_timeout.action(());
                        current_timeout.set(Some(handle));
                    },

                    if *copied.read() {
                        // 已复制图标
                        svg {
                            class: "w-4 h-4 text-green-500",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path {
                                d: "M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"
                            }
                        }
                        span { class: "text-green-500", "已复制" }
                    } else {
                        // 复制图标
                        svg {
                            class: "w-4 h-4",
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path {
                                d: "M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"
                            }
                        }
                    }
                }
            }


            // 代码展示区域（可折叠）
            if *show_code.read() {
                div {
                    class: "border-t border-gray-200 dark:border-gray-700",
                    pre {
                        class: "p-6 bg-gray-900 dark:bg-gray-950 text-gray-100 text-sm overflow-x-auto transition-colors",
                        code {
                            class: "language-rust",
                            "{code}"
                        }
                    }
                }
            }
        }
    }
}

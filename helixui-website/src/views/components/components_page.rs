use crate::i18n;
use dioxus::prelude::*;

/// 组件索引页面 - 展示所有可用的组件
#[component]
pub fn ComponentsPage() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gradient-to-b from-green-50 to-white dark:from-gray-900 dark:to-gray-950 transition-colors",

            // 标题区域
            div {
                class: "container mx-auto px-4 py-16",

                h1 {
                    class: "text-4xl font-bold text-gray-900 dark:text-white mb-4",
                    "{i18n::t(\"nav.components\")}"
                }

                p {
                    class: "text-lg text-gray-600 dark:text-gray-300 mb-12",
                    "Helix UI 提供了一系列精心设计的组件，帮助您快速构建美观的用户界面。"
                }

                // 组件分类网格
                div {
                    class: "space-y-12",

                    // 通用组件
                    ComponentCategory {
                        title: "通用组件",
                        description: "常用的基础 UI 组件",
                        components: vec![
                            ComponentCard {
                                name: "Button".to_string(),
                                name_zh: "按钮".to_string(),
                                description: "按钮用来触发一些操作。".to_string(),
                                route: crate::Route::ButtonPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Icon".to_string(),
                                name_zh: "图标".to_string(),
                                description: "语义化的矢量图形。".to_string(),
                                route: crate::Route::IconPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Avatar".to_string(),
                                name_zh: "头像".to_string(),
                                description: "用来展示用户头像或其他图片。".to_string(),
                                route: crate::Route::AvatarPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Card".to_string(),
                                name_zh: "卡片".to_string(),
                                description: "容器组件，用于组织内容。".to_string(),
                                route: crate::Route::CardPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Carousel".to_string(),
                                name_zh: "轮播图".to_string(),
                                description: "轮播图组件，用于展示多个内容卡片。".to_string(),
                                route: crate::Route::CarouselPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Collapsible".to_string(),
                                name_zh: "折叠面板".to_string(),
                                description: "可以展开和折叠的内容面板组件。".to_string(),
                                route: crate::Route::CollapsiblePage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Divider".to_string(),
                                name_zh: "分割线".to_string(),
                                description: "用于分隔内容的分割线组件。".to_string(),
                                route: crate::Route::DividerPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Dropdown".to_string(),
                                name_zh: "下拉菜单".to_string(),
                                description: "用于创建下拉菜单，支持键盘导航。".to_string(),
                                route: crate::Route::DropdownPage {},
                                status: ComponentStatus::Available,
                            },
                        ],
                    }

                    // 布局组件
                    ComponentCategory {
                        title: "布局组件",
                        description: "布局与排版组件",
                        components: vec![
                            ComponentCard {
                                name: "Flex".to_string(),
                                name_zh: "弹性布局".to_string(),
                                description: "基于 CSS Flex 的一维布局容器。".to_string(),
                                route: crate::Route::FlexPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Grid".to_string(),
                                name_zh: "栅格".to_string(),
                                description: "基于 24 列的响应式栅格系统。".to_string(),
                                route: crate::Route::GridPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Layout".to_string(),
                                name_zh: "布局".to_string(),
                                description: "页面布局组件，提供 Header、Sider、Content、Footer 等布局容器。".to_string(),
                                route: crate::Route::LayoutPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Space".to_string(),
                                name_zh: "间距".to_string(),
                                description: "控制一组子元素间的间距、对齐与换行。".to_string(),
                                route: crate::Route::SpacePage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Split".to_string(),
                                name_zh: "分隔符".to_string(),
                                description: "用于在子元素之间添加分隔符的组件。".to_string(),
                                route: crate::Route::SplitPage {},
                                status: ComponentStatus::Available,
                            },
                        ],
                    }

                    // 数据输入
                    ComponentCategory {
                        title: "数据输入",
                        description: "表单和数据输入相关组件",
                        components: vec![
                            ComponentCard {
                                name: "Input".to_string(),
                                name_zh: "输入框".to_string(),
                                description: "用于文本输入。".to_string(),
                                route: crate::Route::InputPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "TextArea".to_string(),
                                name_zh: "文本域".to_string(),
                                description: "用于多行文本输入。".to_string(),
                                route: crate::Route::TextAreaPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Checkbox".to_string(),
                                name_zh: "复选框".to_string(),
                                description: "用于多项选择。".to_string(),
                                route: crate::Route::CheckboxPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Radio".to_string(),
                                name_zh: "单选框".to_string(),
                                description: "用于单项选择。".to_string(),
                                route: crate::Route::RadioPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Switch".to_string(),
                                name_zh: "开关".to_string(),
                                description: "用于开启或关闭操作。".to_string(),
                                route: crate::Route::SwitchPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Select".to_string(),
                                name_zh: "选择器".to_string(),
                                description: "用于从多个选项中选择。".to_string(),
                                route: crate::Route::ComponentsPage {},
                                status: ComponentStatus::ComingSoon,
                            },
                        ],
                    }

                    // 数据展示
                    ComponentCategory {
                        title: "数据展示",
                        description: "用于展示数据的组件",
                        components: vec![
                            ComponentCard {
                                name: "Table".to_string(),
                                name_zh: "表格".to_string(),
                                description: "用于展示结构化数据。".to_string(),
                                route: crate::Route::TablePage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "ScrollArea".to_string(),
                                name_zh: "滚动域".to_string(),
                                description: "创建可滚动的区域，支持多种滚动模式和方向。".to_string(),
                                route: crate::Route::ScrollAreaPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "List".to_string(),
                                name_zh: "列表".to_string(),
                                description: "用于展示列表数据。".to_string(),
                                route: crate::Route::ComponentsPage {},
                                status: ComponentStatus::ComingSoon,
                            },
                            ComponentCard {
                                name: "Tag".to_string(),
                                name_zh: "标签".to_string(),
                                description: "用于标记和分类。".to_string(),
                                route: crate::Route::ComponentsPage {},
                                status: ComponentStatus::ComingSoon,
                            },
                            ComponentCard {
                                name: "Progress".to_string(),
                                name_zh: "进度条".to_string(),
                                description: "用于显示操作进度。".to_string(),
                                route: crate::Route::ComponentsPage {},
                                status: ComponentStatus::ComingSoon,
                            },
                            ComponentCard {
                                name: "Skeleton".to_string(),
                                name_zh: "骨架屏".to_string(),
                                description: "用于在内容加载时显示占位符。".to_string(),
                                route: crate::Route::ComponentsPage {},
                                status: ComponentStatus::ComingSoon,
                            },
                            ComponentCard {
                                name: "Empty".to_string(),
                                name_zh: "空状态".to_string(),
                                description: "用于展示空数据状态。".to_string(),
                                route: crate::Route::ComponentsPage {},
                                status: ComponentStatus::ComingSoon,
                            },
                        ],
                    }

                    // 反馈组件
                    ComponentCategory {
                        title: "反馈组件",
                        description: "向用户反馈信息",
                        components: vec![
                            ComponentCard {
                                name: "Alert".to_string(),
                                name_zh: "警告框".to_string(),
                                description: "用于显示重要的警告信息。".to_string(),
                                route: crate::Route::AlertPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Badge".to_string(),
                                name_zh: "标记".to_string(),
                                description: "用于显示状态标记或数量信息。".to_string(),
                                route: crate::Route::BadgePage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Dialog".to_string(),
                                name_zh: "对话框".to_string(),
                                description: "用于显示模态对话框和重要信息。".to_string(),
                                route: crate::Route::DialogPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Toast".to_string(),
                                name_zh: "提示框".to_string(),
                                description: "轻量级的全局提示反馈。".to_string(),
                                route: crate::Route::ToastPage {},
                                status: ComponentStatus::Available,
                            },
                            ComponentCard {
                                name: "Tooltip".to_string(),
                                name_zh: "工具提示".to_string(),
                                description: "鼠标悬停时显示的提示信息。".to_string(),
                                route: crate::Route::TooltipPage {},
                                status: ComponentStatus::Available,
                            },
                        ],
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct ComponentCard {
    name: String,
    name_zh: String,
    description: String,
    route: crate::Route,
    status: ComponentStatus,
}

#[derive(Clone, PartialEq)]
enum ComponentStatus {
    Available,
    ComingSoon,
}

/// 组件分类组件
#[component]
fn ComponentCategory(
    title: String,
    description: String,
    components: Vec<ComponentCard>,
) -> Element {
    rsx! {
        div {
            class: "component-category",

            h2 {
                class: "text-2xl font-semibold text-gray-900 dark:text-white mb-2",
                "{title}"
            }

            p {
                class: "text-gray-600 dark:text-gray-300 mb-6",
                "{description}"
            }

            div {
                class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",

                for card in components {
                    ComponentCardItem { card: card.clone() }
                }
            }
        }
    }
}

/// 组件卡片
#[component]
fn ComponentCardItem(card: ComponentCard) -> Element {
    let is_available = card.status == ComponentStatus::Available;

    rsx! {
        if is_available {
            Link {
                to: card.route.clone(),
                class: "block p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 hover:border-green-500 dark:hover:border-green-500 hover:shadow-md transition-all",
                div {
                    class: "flex items-start justify-between mb-3",
                    div {
                        h3 {
                            class: "text-lg font-semibold text-gray-900 dark:text-white",
                            "{card.name_zh} {card.name}"
                        }
                    }
                    span {
                        class: "px-2 py-1 text-xs font-medium text-green-700 dark:text-green-400 bg-green-100 dark:bg-green-900/30 rounded",
                        "可用"
                    }
                }

                p {
                    class: "text-sm text-gray-600 dark:text-gray-300",
                    "{card.description}"
                }
            }
        } else {
            div {
                class: "block p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 opacity-60 cursor-not-allowed",

                div {
                    class: "flex items-start justify-between mb-3",
                    div {
                        h3 {
                            class: "text-lg font-semibold text-gray-900 dark:text-white",
                            "{card.name_zh} {card.name}"
                        }
                    }
                    span {
                        class: "px-2 py-1 text-xs font-medium text-gray-500 dark:text-gray-400 bg-gray-100 dark:bg-gray-700 rounded",
                        "即将推出"
                    }
                }

                p {
                    class: "text-sm text-gray-600 dark:text-gray-300",
                    "{card.description}"
                }
            }
        }
    }
}

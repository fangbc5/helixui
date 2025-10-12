use crate::views::layout::{ComponentsSidebar, DocPage, TocItem};
use dioxus::prelude::*;
use helixui::components::{Avatar, AvatarGroup, AvatarProps, AvatarShape, AvatarSize, DemoBox, Icon, IconType};

/// Avatar 组件演示页面
#[component]
pub fn AvatarPage() -> Element {
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "基础用法".to_string(),
            level: 1,
        },
        TocItem {
            id: "size".to_string(),
            title: "尺寸".to_string(),
            level: 1,
        },
        TocItem {
            id: "shape".to_string(),
            title: "形状".to_string(),
            level: 1,
        },
        TocItem {
            id: "color".to_string(),
            title: "颜色".to_string(),
            level: 1,
        },
        TocItem {
            id: "image".to_string(),
            title: "图片头像".to_string(),
            level: 1,
        },
        TocItem {
            id: "icon".to_string(),
            title: "图标头像".to_string(),
            level: 1,
        },
        TocItem {
            id: "group".to_string(),
            title: "头像组".to_string(),
            level: 1,
        },
        TocItem {
            id: "custom".to_string(),
            title: "自定义".to_string(),
            level: 1,
        },
        TocItem {
            id: "api".to_string(),
            title: "API".to_string(),
            level: 1,
        },
    ];


    rsx! {
        DocPage {
            sidebar: rsx! {
                ComponentsSidebar {}
            },
            toc_items,

            // 标题
            div { class: "mb-8",
                h1 { class: "text-4xl font-bold text-gray-900 dark:text-white mb-2",
                    "头像 Avatar"
                }
                p { class: "text-gray-600 dark:text-gray-300", "头像用来展示用户信息。" }
            }
        
            // 基础用法
            section { id: "basic", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "基础用法"
                }
                p { class: "text-gray-600 dark:text-gray-300 mb-4", "最简单的头像用法。" },
                DemoBox {
                    title: "基础头像".to_string(),
                    description: "最简单的头像用法。".to_string(),
                    code: "use helixui::components::{Avatar, AvatarSize, AvatarShape};

// 基础头像示例
rsx! {
    div { class: \"flex gap-4\",
        Avatar {
            text: Some(\"A\".to_string()),
            background_color: Some(\"#1890ff\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
        Avatar {
            text: Some(\"张三\".to_string()),
            background_color: Some(\"#52c41a\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
        Avatar {
            text: Some(\"李四\".to_string()),
            background_color: Some(\"#faad14\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
    }
}".to_string(),
                    children: rsx! {
                        div { class: "flex gap-4",
                            Avatar {
                                text: Some("A".to_string()),
                                background_color: Some("#1890ff".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                            Avatar {
                                text: Some("张三".to_string()),
                                background_color: Some("#52c41a".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                            Avatar {
                                text: Some("李四".to_string()),
                                background_color: Some("#faad14".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                        }
                    }
                }
            }

            // 尺寸示例
            section { id: "size", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "尺寸"
                }
                p { class: "text-gray-600 dark:text-gray-300 mb-4", "头像支持三种尺寸：小、中、大。" },
                DemoBox {
                    title: "不同尺寸".to_string(),
                    description: "小、中、大三种尺寸的头像。".to_string(),
                    code: "use helixui::components::{Avatar, AvatarSize};

// 不同尺寸的头像示例
rsx! {
    div { class: \"flex gap-4 items-center\",
        Avatar {
            text: Some(\"小\".to_string()),
            size: AvatarSize::Small,
            background_color: Some(\"#1890ff\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
        Avatar {
            text: Some(\"中\".to_string()),
            size: AvatarSize::Medium,
            background_color: Some(\"#52c41a\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
        Avatar {
            text: Some(\"大\".to_string()),
            size: AvatarSize::Large,
            background_color: Some(\"#faad14\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
    }
}".to_string(),
                    children: rsx! {
                        div { class: "flex gap-4 items-center",
                            Avatar {
                                text: Some("小".to_string()),
                                size: AvatarSize::Small,
                                background_color: Some("#1890ff".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                            Avatar {
                                text: Some("中".to_string()),
                                size: AvatarSize::Medium,
                                background_color: Some("#52c41a".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                            Avatar {
                                text: Some("大".to_string()),
                                size: AvatarSize::Large,
                                background_color: Some("#faad14".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                        }
                    }
                }
            }

            // 形状示例
            section { id: "shape", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "形状"
                }
                p { class: "text-gray-600 dark:text-gray-300 mb-4", "头像支持圆形和方形两种形状。" },
                DemoBox {
                    title: "不同形状".to_string(),
                    description: "圆形和方形头像。".to_string(),
                    code: "use helixui::components::{Avatar, AvatarShape};

// 不同形状的头像示例
rsx! {
    div { class: \"flex gap-4\",
        Avatar {
            text: Some(\"圆\".to_string()),
            shape: AvatarShape::Circle,
            background_color: Some(\"#1890ff\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
        Avatar {
            text: Some(\"方\".to_string()),
            shape: AvatarShape::Square,
            background_color: Some(\"#52c41a\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
    }
}".to_string(),
                    children: rsx! {
                        div { class: "flex gap-4",
                            Avatar {
                                text: Some("圆".to_string()),
                                shape: AvatarShape::Circle,
                                background_color: Some("#1890ff".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                            Avatar {
                                text: Some("方".to_string()),
                                shape: AvatarShape::Square,
                                background_color: Some("#52c41a".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                        }
                    }
                }
            }

            // 颜色示例
            section { id: "color", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "颜色"
                }
                p { class: "text-gray-600 dark:text-gray-300 mb-4", "可以自定义头像的背景色和文字颜色。" },
                DemoBox {
                    title: "不同颜色".to_string(),
                    description: "自定义背景色和文字颜色的头像。".to_string(),
                    code: "use helixui::components::Avatar;

// 不同颜色的头像示例
rsx! {
    div { class: \"flex gap-4\",
        Avatar {
            text: Some(\"红\".to_string()),
            background_color: Some(\"#ff4d4f\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
        Avatar {
            text: Some(\"绿\".to_string()),
            background_color: Some(\"#52c41a\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
        Avatar {
            text: Some(\"蓝\".to_string()),
            background_color: Some(\"#1890ff\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
        Avatar {
            text: Some(\"黄\".to_string()),
            background_color: Some(\"#faad14\".to_string()),
            color: Some(\"#ffffff\".to_string()),
        }
    }
}".to_string(),
                    children: rsx! {
                        div { class: "flex gap-4",
                            Avatar {
                                text: Some("红".to_string()),
                                background_color: Some("#ff4d4f".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                            Avatar {
                                text: Some("绿".to_string()),
                                background_color: Some("#52c41a".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                            Avatar {
                                text: Some("蓝".to_string()),
                                background_color: Some("#1890ff".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                    Avatar {
                                text: Some("黄".to_string()),
                                background_color: Some("#faad14".to_string()),
                                color: Some("#ffffff".to_string()),
                            }
                        }
                    }
                }
            }

            // 图片头像示例
            section { id: "image", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "图片头像"
                }
                p { class: "text-gray-600 dark:text-gray-300 mb-4", "可以使用图片作为头像，支持图片加载失败时的备用文字。" },
                DemoBox {
                    title: "图片头像".to_string(),
                    description: "使用图片作为头像，支持备用文字。".to_string(),
                    code: "use helixui::components::Avatar;

// 图片头像示例
rsx! {
    div { class: \"flex gap-4\",
        Avatar {
            src: Some(\"https://api.dicebear.com/7.x/avataaars/svg?seed=1\".to_string()),
            fallback_text: Some(\"用户\".to_string()),
        }
        Avatar {
            src: Some(\"https://api.dicebear.com/7.x/avataaars/svg?seed=2\".to_string()),
            fallback_text: Some(\"张三\".to_string()),
        }
        Avatar {
            src: Some(\"https://api.dicebear.com/7.x/avataaars/svg?seed=3\".to_string()),
            fallback_text: Some(\"李四\".to_string()),
        }
        Avatar {
            src: Some(\"https://invalid-url.com/image.jpg\".to_string()),
            fallback_text: Some(\"加载失败\".to_string()),
            background_color: Some(\"#f0f0f0\".to_string()),
            color: Some(\"#666666\".to_string()),
        }
    }
}".to_string(),
                    children: rsx! {
                        div { class: "flex gap-4",
                            Avatar {
                                src: Some("https://api.dicebear.com/7.x/avataaars/svg?seed=1".to_string()),
                                fallback_text: Some("用户".to_string()),
                            }
                            Avatar {
                                src: Some("https://api.dicebear.com/7.x/avataaars/svg?seed=2".to_string()),
                                fallback_text: Some("张三".to_string()),
                            }
                            Avatar {
                                src: Some("https://api.dicebear.com/7.x/avataaars/svg?seed=3".to_string()),
                                fallback_text: Some("李四".to_string()),
                            }
                            Avatar {
                                src: Some("https://invalid-url.com/image.jpg".to_string()),
                                fallback_text: Some("加载失败".to_string()),
                                background_color: Some("#f0f0f0".to_string()),
                                color: Some("#666666".to_string()),
                            }
                        }
                    }
                }
            }

            // 图标头像示例
            section { id: "icon", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "图标头像"
                }
                p { class: "text-gray-600 dark:text-gray-300 mb-4", "可以使用图标作为头像内容。" },
                DemoBox {
                    title: "图标头像".to_string(),
                    description: "使用图标作为头像内容。".to_string(),
                    code: "use helixui::components::{Avatar, Icon, IconType};

// 图标头像示例
rsx! {
    div { class: \"flex gap-4\",
        Avatar {
            background_color: Some(\"#1890ff\".to_string()),
            color: Some(\"#ffffff\".to_string()),
            children: rsx! {
                Icon {
                    icon: IconType::User,
                    size: helixui::components::IconSize::Medium,
                }
            }
        }
        Avatar {
            background_color: Some(\"#52c41a\".to_string()),
            color: Some(\"#ffffff\".to_string()),
            children: rsx! {
                Icon {
                    icon: IconType::Success,
                    size: helixui::components::IconSize::Medium,
                }
            }
        }
        Avatar {
            background_color: Some(\"#faad14\".to_string()),
            color: Some(\"#ffffff\".to_string()),
            children: rsx! {
                Icon {
                    icon: IconType::Warning,
                    size: helixui::components::IconSize::Medium,
                }
            }
        }
        Avatar {
            background_color: Some(\"#ff4d4f\".to_string()),
            color: Some(\"#ffffff\".to_string()),
            children: rsx! {
                Icon {
                    icon: IconType::Settings,
                    size: helixui::components::IconSize::Medium,
                }
            }
        }
    }
}".to_string(),
                    children: rsx! {
                        div { class: "flex gap-4",
                            Avatar {
                                background_color: Some("#1890ff".to_string()),
                                color: Some("#ffffff".to_string()),
                                children: rsx! {
                                    Icon {
                                        icon: IconType::User,
                                        size: helixui::components::IconSize::Medium,
                                    }
                                }
                            }
                            Avatar {
                                background_color: Some("#52c41a".to_string()),
                                color: Some("#ffffff".to_string()),
                                children: rsx! {
                                    Icon {
                                        icon: IconType::Success,
                                        size: helixui::components::IconSize::Medium,
                                    }
                                }
                            }
                            Avatar {
                                background_color: Some("#faad14".to_string()),
                                color: Some("#ffffff".to_string()),
                                children: rsx! {
                                    Icon {
                                        icon: IconType::Warning,
                                        size: helixui::components::IconSize::Medium,
                                    }
                                }
                            }
                            Avatar {
                                background_color: Some("#ff4d4f".to_string()),
                                color: Some("#ffffff".to_string()),
                                children: rsx! {
                                    Icon {
                                        icon: IconType::Settings,
                                        size: helixui::components::IconSize::Medium,
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 头像组示例
            section { id: "group", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "头像组"
                }
                p { class: "text-gray-600 dark:text-gray-300 mb-4", "多个头像可以组合显示，支持设置最大显示数量。" },
                DemoBox {
                    title: "头像组".to_string(),
                    description: "多个头像组合显示。".to_string(),
                    code: "use helixui::components::{AvatarGroup, AvatarProps};

// 头像组示例
rsx! {
    div { class: \"flex gap-8\",
        // 基础头像组
        AvatarGroup {
            avatars: vec![
                AvatarProps {
                    text: Some(\"A\".to_string()),
                    background_color: Some(\"#1890ff\".to_string()),
                    color: Some(\"#ffffff\".to_string()),
                    ..Default::default()
                },
                AvatarProps {
                    text: Some(\"B\".to_string()),
                    background_color: Some(\"#52c41a\".to_string()),
                    color: Some(\"#ffffff\".to_string()),
                    ..Default::default()
                },
                AvatarProps {
                    text: Some(\"C\".to_string()),
                    background_color: Some(\"#faad14\".to_string()),
                    color: Some(\"#ffffff\".to_string()),
                    ..Default::default()
                },
            ],
            max: 3,
        }
        
        // 带更多数量的头像组
        AvatarGroup {
            avatars: vec![
                AvatarProps {
                    text: Some(\"A\".to_string()),
                    background_color: Some(\"#1890ff\".to_string()),
                    color: Some(\"#ffffff\".to_string()),
                    ..Default::default()
                },
                AvatarProps {
                    text: Some(\"B\".to_string()),
                    background_color: Some(\"#52c41a\".to_string()),
                    color: Some(\"#ffffff\".to_string()),
                    ..Default::default()
                },
                AvatarProps {
                    text: Some(\"C\".to_string()),
                    background_color: Some(\"#faad14\".to_string()),
                    color: Some(\"#ffffff\".to_string()),
                    ..Default::default()
                },
                AvatarProps {
                    text: Some(\"D\".to_string()),
                    background_color: Some(\"#ff4d4f\".to_string()),
                    color: Some(\"#ffffff\".to_string()),
                    ..Default::default()
                },
                AvatarProps {
                    text: Some(\"E\".to_string()),
                    background_color: Some(\"#722ed1\".to_string()),
                    color: Some(\"#ffffff\".to_string()),
                    ..Default::default()
                },
            ],
            max: 3,
        }
    }
}".to_string(),
                    children: rsx! {
                        div { class: "flex gap-8",
                            AvatarGroup {
                                avatars: vec![
                                    AvatarProps {
                                        text: Some("A".to_string()),
                                        background_color: Some("#1890ff".to_string()),
                                        color: Some("#ffffff".to_string()),
                                        ..Default::default()
                                    },
                                    AvatarProps {
                                        text: Some("B".to_string()),
                                        background_color: Some("#52c41a".to_string()),
                                        color: Some("#ffffff".to_string()),
                                        ..Default::default()
                                    },
                                    AvatarProps {
                                        text: Some("C".to_string()),
                                        background_color: Some("#faad14".to_string()),
                                        color: Some("#ffffff".to_string()),
                                        ..Default::default()
                                    },
                                ],
                                max: 3,
                            }
                            AvatarGroup {
                                avatars: vec![
                                    AvatarProps {
                                        text: Some("A".to_string()),
                                        background_color: Some("#1890ff".to_string()),
                                        color: Some("#ffffff".to_string()),
                                        ..Default::default()
                                    },
                                    AvatarProps {
                                        text: Some("B".to_string()),
                                        background_color: Some("#52c41a".to_string()),
                                        color: Some("#ffffff".to_string()),
                                        ..Default::default()
                                    },
                                    AvatarProps {
                                        text: Some("C".to_string()),
                                        background_color: Some("#faad14".to_string()),
                                        color: Some("#ffffff".to_string()),
                                        ..Default::default()
                                    },
                                    AvatarProps {
                                        text: Some("D".to_string()),
                                        background_color: Some("#ff4d4f".to_string()),
                                        color: Some("#ffffff".to_string()),
                                        ..Default::default()
                                    },
                                    AvatarProps {
                                        text: Some("E".to_string()),
                                        background_color: Some("#722ed1".to_string()),
                                        color: Some("#ffffff".to_string()),
                                        ..Default::default()
                                    },
                                ],
                                max: 3,
                            }
                        }
                    }
                }
            }

            // 自定义示例
            section { id: "custom", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "自定义"
                }
                p { class: "text-gray-600 dark:text-gray-300 mb-4", "支持自定义尺寸、样式和类名。" },
                DemoBox {
                    title: "自定义头像".to_string(),
                    description: "自定义尺寸和样式的头像。".to_string(),
                    code: "use helixui::components::{Avatar, AvatarSize};

// 自定义头像示例
rsx! {
    div { class: \"flex gap-4\",
        Avatar {
            text: Some(\"自定义\".to_string()),
            size: AvatarSize::Custom(60.0),
            background_color: Some(\"#722ed1\".to_string()),
            color: Some(\"#ffffff\".to_string()),
            class: Some(\"border-2 border-gray-300\".to_string()),
        }
        Avatar {
            text: Some(\"大号\".to_string()),
            size: AvatarSize::Custom(80.0),
            background_color: Some(\"#13c2c2\".to_string()),
            color: Some(\"#ffffff\".to_string()),
            class: Some(\"shadow-lg\".to_string()),
        }
    }
}".to_string(),
                    children: rsx! {
                        div { class: "flex gap-4",
                            Avatar {
                                text: Some("自定义".to_string()),
                                size: AvatarSize::Custom(60.0),
                                background_color: Some("#722ed1".to_string()),
                                color: Some("#ffffff".to_string()),
                                class: Some("border-2 border-gray-300".to_string()),
                            }
                            Avatar {
                                text: Some("大号".to_string()),
                                size: AvatarSize::Custom(80.0),
                                background_color: Some("#13c2c2".to_string()),
                                color: Some("#ffffff".to_string()),
                                class: Some("shadow-lg".to_string()),
                            }
                        }
                    }
                }
            }

            // API 文档
            section { id: "api", class: "mb-12",
                h2 { class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "API"
                }
                div { class: "bg-gray-50 dark:bg-gray-800 rounded-lg p-6",
                    h3 { class: "text-lg font-semibold text-gray-900 dark:text-white mb-3",
                        "Avatar Props"
                    }
                    div { class: "overflow-x-auto",
                        table { class: "w-full text-sm",
                            thead {
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    th { class: "text-left py-2 px-3 font-medium text-gray-900 dark:text-white", "属性" }
                                    th { class: "text-left py-2 px-3 font-medium text-gray-900 dark:text-white", "类型" }
                                    th { class: "text-left py-2 px-3 font-medium text-gray-900 dark:text-white", "默认值" }
                                    th { class: "text-left py-2 px-3 font-medium text-gray-900 dark:text-white", "说明" }
                                }
                            }
                            tbody {
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    td { class: "py-2 px-3 text-gray-900 dark:text-white", "size" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "AvatarSize" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "Medium" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "头像尺寸" }
                                }
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    td { class: "py-2 px-3 text-gray-900 dark:text-white", "shape" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "AvatarShape" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "Circle" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "头像形状" }
                                }
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    td { class: "py-2 px-3 text-gray-900 dark:text-white", "text" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "Option<String>" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "头像文字" }
                                }
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    td { class: "py-2 px-3 text-gray-900 dark:text-white", "src" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "Option<String>" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "头像图片源 URL" }
                                }
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    td { class: "py-2 px-3 text-gray-900 dark:text-white", "fallback_text" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "Option<String>" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "图片加载失败时的备用文字" }
                                }
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    td { class: "py-2 px-3 text-gray-900 dark:text-white", "children" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "Option<Element>" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "自定义子元素（如图标）" }
                                }
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    td { class: "py-2 px-3 text-gray-900 dark:text-white", "background_color" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "Option<String>" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "背景色" }
                                }
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    td { class: "py-2 px-3 text-gray-900 dark:text-white", "color" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "Option<String>" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "文字颜色" }
                                }
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    td { class: "py-2 px-3 text-gray-900 dark:text-white", "class" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "Option<String>" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "自定义类名" }
                                }
                                tr { class: "border-b border-gray-200 dark:border-gray-700",
                                    td { class: "py-2 px-3 text-gray-900 dark:text-white", "style" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "Option<String>" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "None" }
                                    td { class: "py-2 px-3 text-gray-600 dark:text-gray-300", "自定义样式" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

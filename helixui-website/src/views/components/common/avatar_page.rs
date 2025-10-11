use crate::views::layout::{DocPage, TocItem};
use dioxus::prelude::*;
use helixui::components::{
    Avatar, AvatarGroup, AvatarGroupProps, AvatarProps, AvatarShape, AvatarSize, DemoBox, Icon,
    IconType,
};

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

    // 定义颜色常量
    let gray_color = "#f0f0f0";
    let blue_color = "#e6f7ff";
    let blue_text = "#1890ff";
    let red_color = "#ff4d4f";
    let green_color = "#52c41a";
    let primary_blue = "#1890ff";
    let white_text = "#ffffff";

    rsx! {
        DocPage {
            title: "Avatar 头像",
            description: "用来代表用户或事物，支持图片、文字或图标展示。",
            toc_items: toc_items,

            // 基础用法
            section {
                id: "basic",
                class: "mb-12",

                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                    "基础用法"
                }

                p {
                    class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                    "头像的基础用法，支持文字、图片和图标。"
                }

                DemoBox {
                    title: "文字头像".to_string(),
                    description: "使用文字作为头像内容".to_string(),
                    code: r#"use helixui::components::{Avatar, AvatarSize, AvatarShape};

rsx! {
    div {
        class: "flex items-center space-x-4",
        Avatar {
            text: Some("A".to_string()),
            size: AvatarSize::Medium,
        }
        Avatar {
            text: Some("AB".to_string()),
            size: AvatarSize::Medium,
        }
        Avatar {
            text: Some("ABC".to_string()),
            size: AvatarSize::Medium,
        }
    }
}"#.to_string(),

                    div {
                        class: "flex items-center space-x-4",
                        Avatar {
                            text: Some("A".to_string()),
                            size: AvatarSize::Medium,
                        }
                        Avatar {
                            text: Some("AB".to_string()),
                            size: AvatarSize::Medium,
                        }
                        Avatar {
                            text: Some("ABC".to_string()),
                            size: AvatarSize::Medium,
                        }
                    }
                }

                DemoBox {
                    title: "图片头像".to_string(),
                    description: "使用图片作为头像内容".to_string(),
                    code: r#"use helixui::components::{Avatar, AvatarSize, AvatarShape};

rsx! {
    div {
        class: "flex items-center space-x-4",
        Avatar {
            src: Some("https://07akioni.oss-cn-beijing.aliyuncs.com/demo1.JPG".to_string()),
            size: AvatarSize::Medium,
        }
        Avatar {
            src: Some("https://07akioni.oss-cn-beijing.aliyuncs.com/demo2.JPG".to_string()),
            size: AvatarSize::Medium,
        }
    }
}"#.to_string(),

                    div {
                        class: "flex items-center space-x-4",
                        Avatar {
                            src: Some("https://07akioni.oss-cn-beijing.aliyuncs.com/demo1.JPG".to_string()),
                            size: AvatarSize::Medium,
                        }
                        Avatar {
                            src: Some("https://07akioni.oss-cn-beijing.aliyuncs.com/demo2.JPG".to_string()),
                            size: AvatarSize::Medium,
                        }
                    }
                }

                DemoBox {
                    title: "图标头像".to_string(),
                    description: "使用图标作为头像内容".to_string(),
                    code: r#"use helixui::components::{Avatar, AvatarSize, AvatarShape, Icon, IconType};

rsx! {
    div {
        class: "flex items-center space-x-4",
        Avatar {
            size: AvatarSize::Medium,
            background_color: Some("#f0f0f0".to_string()),
            children: Some(rsx! {
                Icon {
                    icon_type: IconType::User,
                    size: 20,
                }
            }),
        }
        Avatar {
            size: AvatarSize::Medium,
            background_color: Some(String::from("#") + "e6f7ff"),
            color: Some("#1890ff".to_string()),
            children: Some(rsx! {
                Icon {
                    icon_type: IconType::Settings,
                    size: 20,
                }
            }),
        }
    }
}"#.to_string(),

                    div {
                        class: "flex items-center space-x-4",
                        Avatar {
                            size: AvatarSize::Medium,
                            background_color: Some(gray_color.to_string()),
                            children: Some(rsx! {
                                Icon {
                                    icon_type: IconType::User,
                                    size: 20,
                                }
                            }),
                        }
                        Avatar {
                            size: AvatarSize::Medium,
                            background_color: Some(blue_color.to_string()),
                            color: Some(blue_text.to_string()),
                            children: Some(rsx! {
                                Icon {
                                    icon_type: IconType::Settings,
                                    size: 20,
                                }
                            }),
                        }
                    }
                }
            }
        }

        // 尺寸
        section {
            id: "size",
            class: "mb-12",

            h2 {
                class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                "尺寸"
            }

            p {
                class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                "头像支持多种尺寸，也可以自定义尺寸。"
            }

            DemoBox {
                title: "预设尺寸".to_string(),
                description: "使用预设的尺寸选项".to_string(),
                code: r#"use helixui::components::{Avatar, AvatarSize, AvatarShape};

rsx! {
    div {
        class: "flex items-center space-x-4",
        Avatar {
            text: Some("S".to_string()),
            size: AvatarSize::Small,
        }
        Avatar {
            text: Some("M".to_string()),
            size: AvatarSize::Medium,
        }
        Avatar {
            text: Some("L".to_string()),
            size: AvatarSize::Large,
        }
    }
}"#.to_string(),

                div {
                    class: "flex items-center space-x-4",
                    Avatar {
                        text: Some("S".to_string()),
                        size: AvatarSize::Small,
                    }
                    Avatar {
                        text: Some("M".to_string()),
                        size: AvatarSize::Medium,
                    }
                    Avatar {
                        text: Some("L".to_string()),
                        size: AvatarSize::Large,
                    }
                }
            }

            DemoBox {
                title: "自定义尺寸".to_string(),
                description: "使用自定义尺寸".to_string(),
                code: r#"use helixui::components::{Avatar, AvatarSize, AvatarShape};

rsx! {
    div {
        class: "flex items-center space-x-4",
        Avatar {
            text: Some("32".to_string()),
            size: AvatarSize::Custom(32.0),
        }
        Avatar {
            text: Some("48".to_string()),
            size: AvatarSize::Custom(48.0),
        }
        Avatar {
            text: Some("64".to_string()),
            size: AvatarSize::Custom(64.0),
        }
    }
}"#.to_string(),

                div {
                    class: "flex items-center space-x-4",
                    Avatar {
                        text: Some("32".to_string()),
                        size: AvatarSize::Custom(32.0),
                    }
                    Avatar {
                        text: Some("48".to_string()),
                        size: AvatarSize::Custom(48.0),
                    }
                    Avatar {
                        text: Some("64".to_string()),
                        size: AvatarSize::Custom(64.0),
                    }
                }
            }
        }

        // 形状
        section {
            id: "shape",
            class: "mb-12",

            h2 {
                class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                "形状"
            }

            p {
                class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                "头像支持圆形和方形两种形状。"
            }

            DemoBox {
                title: "圆形头像".to_string(),
                description: "默认的圆形头像".to_string(),
                code: r#"use helixui::components::{Avatar, AvatarSize, AvatarShape};

rsx! {
    div {
        class: "flex items-center space-x-4",
        Avatar {
            text: Some("A".to_string()),
            size: AvatarSize::Medium,
            shape: AvatarShape::Circle,
        }
        Avatar {
            src: Some("https://07akioni.oss-cn-beijing.aliyuncs.com/demo1.JPG".to_string()),
            size: AvatarSize::Medium,
            shape: AvatarShape::Circle,
        }
    }
}"#.to_string(),

                div {
                    class: "flex items-center space-x-4",
                    Avatar {
                        text: Some("A".to_string()),
                        size: AvatarSize::Medium,
                        shape: AvatarShape::Circle,
                    }
                    Avatar {
                        src: Some("https://07akioni.oss-cn-beijing.aliyuncs.com/demo1.JPG".to_string()),
                        size: AvatarSize::Medium,
                        shape: AvatarShape::Circle,
                    }
                }
            }

            DemoBox {
                title: "方形头像".to_string(),
                description: "方形头像".to_string(),
                code: r#"use helixui::components::{Avatar, AvatarSize, AvatarShape};

rsx! {
    div {
        class: "flex items-center space-x-4",
        Avatar {
            text: Some("A".to_string()),
            size: AvatarSize::Medium,
            shape: AvatarShape::Square,
        }
        Avatar {
            src: Some("https://07akioni.oss-cn-beijing.aliyuncs.com/demo1.JPG".to_string()),
            size: AvatarSize::Medium,
            shape: AvatarShape::Square,
        }
    }
}"#.to_string(),

                div {
                    class: "flex items-center space-x-4",
                    Avatar {
                        text: Some("A".to_string()),
                        size: AvatarSize::Medium,
                        shape: AvatarShape::Square,
                    }
                    Avatar {
                        src: Some("https://07akioni.oss-cn-beijing.aliyuncs.com/demo1.JPG".to_string()),
                        size: AvatarSize::Medium,
                        shape: AvatarShape::Square,
                    }
                }
            }
        }

        // 颜色
        section {
            id: "color",
            class: "mb-12",

            h2 {
                class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                "颜色"
            }

            p {
                class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                "可以自定义头像的颜色和背景色。"
            }

            DemoBox {
                title: "自定义颜色".to_string(),
                description: "设置头像的文字颜色和背景色".to_string(),
                code: r#"use helixui::components::{Avatar, AvatarSize, AvatarShape};

rsx! {
    div {
        class: "flex items-center space-x-4",
        Avatar {
            text: Some("A".to_string()),
            size: AvatarSize::Medium,
            background_color: Some("#ff4d4f".to_string()),
            color: Some(String::from("#") + "ffffff"),
        }
        Avatar {
            text: Some("B".to_string()),
            size: AvatarSize::Medium,
            background_color: Some("#52c41a".to_string()),
            color: Some(String::from("#") + "ffffff"),
        }
        Avatar {
            text: Some("C".to_string()),
            size: AvatarSize::Medium,
            background_color: Some("#1890ff".to_string()),
            color: Some(String::from("#") + "ffffff"),
        }
    }
}"#.to_string(),

                div {
                    class: "flex items-center space-x-4",
                    Avatar {
                        text: Some("A".to_string()),
                        size: AvatarSize::Medium,
                        background_color: Some(red_color.to_string()),
                        color: Some(white_text.to_string()),
                    }
                    Avatar {
                        text: Some("B".to_string()),
                        size: AvatarSize::Medium,
                        background_color: Some(green_color.to_string()),
                        color: Some(white_text.to_string()),
                    }
                    Avatar {
                        text: Some("C".to_string()),
                        size: AvatarSize::Medium,
                        background_color: Some(primary_blue.to_string()),
                        color: Some(white_text.to_string()),
                    }
                }
            }
        }

        // 头像组
        section {
            id: "group",
            class: "mb-12",

            h2 {
                class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                "头像组"
            }

            p {
                class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                "多个头像可以组合显示，支持设置最大显示数量和间距。"
            }

            DemoBox {
                title: "基础头像组".to_string(),
                description: "多个头像的组合显示".to_string(),
                code: r#"use helixui::components::{Avatar, AvatarGroup, AvatarGroupProps, AvatarProps, AvatarSize, AvatarShape};

rsx! {
    AvatarGroup {
        avatars: vec![
            AvatarProps {
                text: Some("A".to_string()),
                size: AvatarSize::Medium,
                ..Default::default()
            },
            AvatarProps {
                text: Some("B".to_string()),
                size: AvatarSize::Medium,
                ..Default::default()
            },
            AvatarProps {
                text: Some("C".to_string()),
                size: AvatarSize::Medium,
                ..Default::default()
            },
        ],
        max: 3,
    }
}"#.to_string(),

                AvatarGroup {
                    avatars: vec![
                        AvatarProps {
                            text: Some("A".to_string()),
                            size: AvatarSize::Medium,
                            ..Default::default()
                        },
                        AvatarProps {
                            text: Some("B".to_string()),
                            size: AvatarSize::Medium,
                            ..Default::default()
                        },
                        AvatarProps {
                            text: Some("C".to_string()),
                            size: AvatarSize::Medium,
                            ..Default::default()
                        },
                    ],
                    max: 3,
                }
            }

            DemoBox {
                title: "限制显示数量".to_string(),
                description: "当头像数量超过最大显示数量时，会显示更多数量的提示".to_string(),
                code: r#"use helixui::components::{Avatar, AvatarGroup, AvatarGroupProps, AvatarProps, AvatarSize, AvatarShape};

rsx! {
    AvatarGroup {
        avatars: vec![
            AvatarProps {
                text: Some("A".to_string()),
                size: AvatarSize::Medium,
                ..Default::default()
            },
            AvatarProps {
                text: Some("B".to_string()),
                size: AvatarSize::Medium,
                ..Default::default()
            },
            AvatarProps {
                text: Some("C".to_string()),
                size: AvatarSize::Medium,
                ..Default::default()
            },
            AvatarProps {
                text: Some("D".to_string()),
                size: AvatarSize::Medium,
                ..Default::default()
            },
            AvatarProps {
                text: Some("E".to_string()),
                size: AvatarSize::Medium,
                ..Default::default()
            },
        ],
        max: 3,
    }
}"#.to_string(),

                AvatarGroup {
                    avatars: vec![
                        AvatarProps {
                            text: Some("A".to_string()),
                            size: AvatarSize::Medium,
                            ..Default::default()
                        },
                        AvatarProps {
                            text: Some("B".to_string()),
                            size: AvatarSize::Medium,
                            ..Default::default()
                        },
                        AvatarProps {
                            text: Some("C".to_string()),
                            size: AvatarSize::Medium,
                            ..Default::default()
                        },
                        AvatarProps {
                            text: Some("D".to_string()),
                            size: AvatarSize::Medium,
                            ..Default::default()
                        },
                        AvatarProps {
                            text: Some("E".to_string()),
                            size: AvatarSize::Medium,
                            ..Default::default()
                        },
                    ],
                    max: 3,
                }
            }
        }

        // 自定义
        section {
            id: "custom",
            class: "mb-12",

            h2 {
                class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                "自定义"
            }

            p {
                class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                "可以完全自定义头像的内容和样式。"
            }

            DemoBox {
                title: "自定义内容".to_string(),
                description: "使用自定义内容作为头像".to_string(),
                code: r#"use helixui::components::{Avatar, AvatarSize, AvatarShape, Icon, IconType};

rsx! {
    div {
        class: "flex items-center space-x-4",
        Avatar {
            size: AvatarSize::Medium,
            background_color: Some("#f0f0f0".to_string()),
            children: Some(rsx! {
                div {
                    class: "flex flex-col items-center justify-center text-".to_string() + "xs",
                    div { "VIP" }
                    div { "1" }
                }
            }),
        }
        Avatar {
            size: AvatarSize::Medium,
            background_color: Some("#1890ff".to_string()),
            color: Some(String::from("#") + "ffffff"),
            children: Some(rsx! {
                div {
                    class: "flex items-center justify-".to_string() + "center",
                    Icon {
                        icon_type: IconType::Star,
                        size: 16,
                    }
                }
            }),
        }
    }
}"#.to_string(),

                div {
                    class: "flex items-center space-x-4",
                    Avatar {
                        size: AvatarSize::Medium,
                        background_color: Some(gray_color.to_string()),
                        children: Some(rsx! {
                            div {
                                class: "flex flex-col items-center justify-center text-".to_string() + "xs",
                                div { "VIP" }
                                div { "1" }
                            }
                        }),
                    }
                    Avatar {
                        size: AvatarSize::Medium,
                        background_color: Some(primary_blue.to_string()),
                        color: Some(white_text.to_string()),
                        children: Some(rsx! {
                            div {
                                class: "flex items-center justify-".to_string() + "center",
                                Icon {
                                    icon_type: IconType::Star,
                                    size: 16,
                                }
                            }
                        }),
                    }
                }
            }
        }

        // API
        section {
            id: "api",
            class: "mb-12",

            h2 {
                class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4 transition-colors",
                "API"
            }

            p {
                class: "text-gray-600 dark:text-gray-300 mb-4 transition-colors",
                "Avatar 组件的属性说明。"
            }

            div {
                class: "overflow-x-auto",
                table {
                    class: "w-full border-collapse border border-gray-200 dark:border-gray-700",
                    thead {
                        tr {
                            class: "bg-gray-50 dark:bg-gray-800",
                            th {
                                class: "px-4 py-3 text-left text-sm font-medium text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "属性"
                            }
                            th {
                                class: "px-4 py-3 text-left text-sm font-medium text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "类型"
                            }
                            th {
                                class: "px-4 py-3 text-left text-sm font-medium text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "默认值"
                            }
                            th {
                                class: "px-4 py-3 text-left text-sm font-medium text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "说明"
                            }
                        }
                    }
                    tbody {
                        tr {
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "size"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "AvatarSize"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "Medium"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "头像尺寸"
                            }
                        }
                        tr {
                            class: "bg-gray-50 dark:bg-gray-800",
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "shape"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "AvatarShape"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "Circle"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "头像形状"
                            }
                        }
                        tr {
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "color"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "Option&lt;String&gt;"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "None"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "头像文字颜色"
                            }
                        }
                        tr {
                            class: "bg-gray-50 dark:bg-gray-800",
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "background_color"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "Option&lt;String&gt;"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "None"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "头像背景颜色"
                            }
                        }
                        tr {
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "text"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "Option&lt;String&gt;"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "None"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "头像文字内容"
                            }
                        }
                        tr {
                            class: "bg-gray-50 dark:bg-gray-800",
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "src"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "Option&lt;String&gt;"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "None"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "头像图片地址"
                            }
                        }
                        tr {
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "children"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "Option&lt;Element&gt;"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "None"
                            }
                            td {
                                class: "px-4 py-3 text-sm text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700",
                                "自定义头像内容"
                            }
                        }
                    }
                }
            }
        }
    }
}

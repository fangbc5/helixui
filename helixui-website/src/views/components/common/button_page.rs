use helixui::components::{
    Button, ButtonGroup, ButtonGroupItemProps, ButtonShape, ButtonSize, ButtonType, ButtonVariant, DemoBox, IconType,
};
use crate::views::layout::TocItem;
use crate::views::layout::{ComponentsSidebar, DocPage};
use dioxus::prelude::*;

/// Button 组件文档页面
#[component]
pub fn ButtonPage() -> Element {
    // 定义目录项
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "演示".to_string(),
            level: 1,
        },
        TocItem {
            id: "secondary".to_string(),
            title: "次要按钮".to_string(),
            level: 1,
        },
        TocItem {
            id: "size".to_string(),
            title: "尺寸".to_string(),
            level: 1,
        },
        TocItem {
            id: "disabled".to_string(),
            title: "禁用".to_string(),
            level: 1,
        },
        TocItem {
            id: "icon".to_string(),
            title: "图标".to_string(),
            level: 1,
        },
        TocItem {
            id: "shape".to_string(),
            title: "形状".to_string(),
            level: 1,
        },
        TocItem {
            id: "dashed".to_string(),
            title: "虚线边框".to_string(),
            level: 1,
        },
        TocItem {
            id: "pure".to_string(),
            title: "纯文字和纯图标".to_string(),
            level: 1,
        },
        TocItem {
            id: "group".to_string(),
            title: "按钮组".to_string(),
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
            sidebar: rsx! { ComponentsSidebar {} },
            toc_items,

            div {
                class: "component-doc",

            // 标题
            div {
                class: "mb-8",
                h1 {
                    class: "text-4xl font-bold text-gray-900 dark:text-white mb-2",
                    "按钮 Button"
                }
                p {
                    class: "text-gray-600 dark:text-gray-300",
                    "按钮用来触发一些操作。"
                }
            }

            // 基础演示
            section {
                id: "basic",
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "演示"
                }

                DemoBox {
                    title: "基础".to_string(),
                    description: "按钮的 type 分别为 default、tertiary、primary、info、success、warning、error、pureText 和 pureIcon。".to_string(),
                    code: r#"use helixui::components::{Button, ButtonType, ButtonVariant, IconType};

rsx! {
    Button { "Default" }
    Button { button_type: ButtonType::Tertiary, "Tertiary" }
    Button { button_type: ButtonType::Primary, "Primary" }
    Button { button_type: ButtonType::Info, "Info" }
    Button { button_type: ButtonType::Success, "Success" }
    Button { button_type: ButtonType::Warning, "Warning" }
    Button { button_type: ButtonType::Error, "Error" }
    Button { button_type: ButtonType::PureText, "PureText" }
    Button { 
        button_type: ButtonType::PureIcon,
        variant: ButtonVariant::Icon,
        icon: Some(IconType::Settings),
    }
}"#.to_string(),

                    div {
                        class: "flex flex-wrap gap-3",
                        Button { "Default" }
                        Button { button_type: ButtonType::Tertiary, "Tertiary" }
                        Button { button_type: ButtonType::Primary, "Primary" }
                        Button { button_type: ButtonType::Info, "Info" }
                        Button { button_type: ButtonType::Success, "Success" }
                        Button { button_type: ButtonType::Warning, "Warning" }
                        Button { button_type: ButtonType::Error, "Error" }
                        Button { button_type: ButtonType::PureText, "PureText" }
                        Button { 
                            button_type: ButtonType::PureIcon,
                            variant: ButtonVariant::Icon,
                            icon: Some(IconType::Settings),
                        }
                    }
                }
            }

            // 次要按钮
            section {
                id: "secondary",
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "次要按钮"
                }

                DemoBox {
                    title: "次要按钮".to_string(),
                    description: "次要按钮使用浅色背景。".to_string(),
                    code: r#"use helixui::components::{Button, ButtonType};

rsx! {
    Button { button_type: ButtonType::Default, "Default" }
    Button { button_type: ButtonType::Tertiary, "Tertiary" }
    Button { button_type: ButtonType::Primary, secondary: true, "Primary" }
    Button { button_type: ButtonType::Info, secondary: true, "Info" }
    Button { button_type: ButtonType::Success, secondary: true, "Success" }
    Button { button_type: ButtonType::Warning, secondary: true, "Warning" }
    Button { button_type: ButtonType::Error, secondary: true, "Error" }
}"#.to_string(),

                    div {
                        class: "flex flex-wrap gap-3",
                        Button { button_type: ButtonType::Default, "Default" }
                        Button { button_type: ButtonType::Tertiary, "Tertiary" }
                        Button { button_type: ButtonType::Primary, secondary: true, "Primary" }
                        Button { button_type: ButtonType::Info, secondary: true, "Info" }
                        Button { button_type: ButtonType::Success, secondary: true, "Success" }
                        Button { button_type: ButtonType::Warning, secondary: true, "Warning" }
                        Button { button_type: ButtonType::Error, secondary: true, "Error" }
                    }
                }
            }

            // 尺寸
            section {
                id: "size",
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "尺寸"
                }

                DemoBox {
                    title: "按钮尺寸".to_string(),
                    description: "按钮有超小、小、中、大四种尺寸。".to_string(),
                    code: r#"use helixui::components::{Button, ButtonSize, ButtonType};

rsx! {
    Button { 
        button_type: ButtonType::Primary, 
        size: ButtonSize::Tiny, 
        "Tiny" 
    }
    Button { 
        button_type: ButtonType::Primary, 
        size: ButtonSize::Small, 
        "Small" 
    }
    Button { 
        button_type: ButtonType::Primary, 
        size: ButtonSize::Medium, 
        "Medium" 
    }
    Button { 
        button_type: ButtonType::Primary, 
        size: ButtonSize::Large, 
        "Large" 
    }
}"#.to_string(),

                    div {
                        class: "flex flex-wrap items-center gap-3",
                        Button { button_type: ButtonType::Primary, size: ButtonSize::Tiny, "Tiny" }
                        Button { button_type: ButtonType::Primary, size: ButtonSize::Small, "Small" }
                        Button { button_type: ButtonType::Primary, size: ButtonSize::Medium, "Medium" }
                        Button { button_type: ButtonType::Primary, size: ButtonSize::Large, "Large" }
                    }
                }
            }

            // 禁用
            section {
                id: "disabled",
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "禁用状态"
                }

                DemoBox {
                    title: "禁用按钮".to_string(),
                    description: "按钮可以被禁用。".to_string(),
                    code: r#"use helixui::components::{Button, ButtonType};

rsx! {
    Button { button_type: ButtonType::Default, disabled: true, "Default" }
    Button { button_type: ButtonType::Primary, disabled: true, "Primary" }
    Button { button_type: ButtonType::Info, disabled: true, "Info" }
    Button { button_type: ButtonType::Success, disabled: true, "Success" }
}"#.to_string(),

                    div {
                        class: "flex flex-wrap gap-3",
                        Button { button_type: ButtonType::Default, disabled: true, "Default" }
                        Button { button_type: ButtonType::Primary, disabled: true, "Primary" }
                        Button { button_type: ButtonType::Info, disabled: true, "Info" }
                        Button { button_type: ButtonType::Success, disabled: true, "Success" }
                    }
                }
                    }

                    // 图标按钮
                    section {
                        id: "icon",
                        class: "mb-12",
                        h2 {
                            class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                            "图标按钮"
                        }

                        DemoBox {
                            title: "纯图标按钮".to_string(),
                            description: "只显示图标的按钮，适合工具栏和紧凑布局。".to_string(),
                            code: r#"use helixui::components::{Button, ButtonType, ButtonVariant, IconType};

        rsx! {
            Button {
                button_type: ButtonType::Default,
                variant: ButtonVariant::Icon,
                icon: Some(IconType::ChevronLeft),
                "←"
            }
            Button {
                button_type: ButtonType::Primary,
                variant: ButtonVariant::Icon,
                icon: Some(IconType::ChevronRight),
                "→"
            }
            Button {
                button_type: ButtonType::Info,
                variant: ButtonVariant::Icon,
                icon: Some(IconType::Settings),
                "⚙"
            }
        }"#.to_string(),

                            div {
                                class: "flex flex-wrap gap-3",
                                Button {
                                    button_type: ButtonType::Default,
                                    variant: ButtonVariant::Icon,
                                    icon: Some(IconType::ChevronLeft),
                                }
                                Button {
                                    button_type: ButtonType::Primary,
                                    variant: ButtonVariant::Icon,
                                    icon: Some(IconType::ChevronRight),
                                }
                                Button {
                                    button_type: ButtonType::Info,
                                    variant: ButtonVariant::Icon,
                                    icon: Some(IconType::Settings),
                                }
                            }
                        }

                        DemoBox {
                            title: "不同尺寸的图标按钮".to_string(),
                            description: "图标按钮支持多种尺寸，图标会自动居中显示。".to_string(),
                            code: r#"use helixui::components::{Button, ButtonSize, ButtonType, ButtonVariant, IconType};

rsx! {
    div { class: "flex items-center gap-3",
        Button {
            button_type: ButtonType::Primary,
            size: ButtonSize::Tiny,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::Settings),
        }
        Button {
            button_type: ButtonType::Primary,
            size: ButtonSize::Small,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::Settings),
        }
        Button {
            button_type: ButtonType::Primary,
            size: ButtonSize::Medium,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::Settings),
        }
        Button {
            button_type: ButtonType::Primary,
            size: ButtonSize::Large,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::Settings),
        }
    }
}"#.to_string(),
                            children: rsx! {
                                div {
                                    class: "flex items-center gap-3",
                                    Button {
                                        button_type: ButtonType::Primary,
                                        size: ButtonSize::Tiny,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::Settings),
                                    }
                                    Button {
                                        button_type: ButtonType::Primary,
                                        size: ButtonSize::Small,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::Settings),
                                    }
                                    Button {
                                        button_type: ButtonType::Primary,
                                        size: ButtonSize::Medium,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::Settings),
                                    }
                                    Button {
                                        button_type: ButtonType::Primary,
                                        size: ButtonSize::Large,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::Settings),
                                    }
                                }
                            }
                        }

                        DemoBox {
                            title: "图标+文字按钮".to_string(),
                            description: "同时显示图标和文字的按钮。".to_string(),
                            code: r#"use helixui::components::{Button, ButtonType, ButtonVariant, IconType};

        rsx! {
            Button {
                button_type: ButtonType::Primary,
                variant: ButtonVariant::IconText,
                icon: Some(IconType::Check),
                "确认"
            }
            Button {
                button_type: ButtonType::Error,
                variant: ButtonVariant::IconText,
                icon: Some(IconType::Close),
                "取消"
            }
        }"#.to_string(),

                            div {
                                class: "flex flex-wrap gap-3",
                                Button {
                                    button_type: ButtonType::Primary,
                                    variant: ButtonVariant::IconText,
                                    icon: Some(IconType::Check),
                                    "确认"
                                }
                                Button {
                                    button_type: ButtonType::Error,
                                    variant: ButtonVariant::IconText,
                                    icon: Some(IconType::Close),
                                    "取消"
                                }
                            }
                        }
                    }

                    // 按钮形状
                    section {
                        id: "shape",
                        class: "mb-12",
                        h2 {
                            class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                            "按钮形状"
                        }

                        DemoBox {
                            title: "不同形状的按钮".to_string(),
                            description: "按钮支持多种形状：默认矩形、圆角矩形、圆形和椭圆形。".to_string(),
                            code: r#"use helixui::components::{Button, ButtonShape, ButtonType, ButtonVariant, IconType};

        rsx! {
            // 默认矩形
            Button {
                button_type: ButtonType::Primary,
                shape: ButtonShape::Default,
                "默认"
            }
            // 圆角矩形
            Button {
                button_type: ButtonType::Primary,
                shape: ButtonShape::Rounded,
                "圆角"
            }
            // 圆形
            Button {
                button_type: ButtonType::Primary,
                variant: ButtonVariant::Icon,
                shape: ButtonShape::Circle,
                icon: Some(IconType::Settings),
            }
            // 椭圆形
            Button {
                button_type: ButtonType::Primary,
                shape: ButtonShape::Ellipse,
                "椭圆形"
            }
        }"#.to_string(),

                            div {
                                class: "flex flex-wrap items-center gap-4",
                                Button {
                                    button_type: ButtonType::Primary,
                                    shape: ButtonShape::Default,
                                    "默认"
                                }
                                Button {
                                    button_type: ButtonType::Primary,
                                    shape: ButtonShape::Rounded,
                                    "圆角"
                                }
                                Button {
                                    button_type: ButtonType::Primary,
                                    variant: ButtonVariant::Icon,
                                    shape: ButtonShape::Circle,
                                    icon: Some(IconType::Settings),
                                }
                                Button {
                                    button_type: ButtonType::Primary,
                                    shape: ButtonShape::Ellipse,
                                    "椭圆形"
                                }
                            }
                        }

                        DemoBox {
                            title: "圆形图标按钮".to_string(),
                            description: "圆形按钮特别适合图标按钮，提供紧凑的视觉体验。".to_string(),
                            code: r#"use helixui::components::{Button, ButtonShape, ButtonType, ButtonVariant, IconType};

        rsx! {
            Button {
                button_type: ButtonType::Default,
                variant: ButtonVariant::Icon,
                shape: ButtonShape::Circle,
                icon: Some(IconType::ChevronLeft),
            }
            Button {
                button_type: ButtonType::Primary,
                variant: ButtonVariant::Icon,
                shape: ButtonShape::Circle,
                icon: Some(IconType::ChevronRight),
            }
            Button {
                button_type: ButtonType::Info,
                variant: ButtonVariant::Icon,
                shape: ButtonShape::Circle,
                icon: Some(IconType::Settings),
            }
        }"#.to_string(),

                            div {
                                class: "flex flex-wrap gap-3",
                                Button {
                                    button_type: ButtonType::Default,
                                    variant: ButtonVariant::Icon,
                                    shape: ButtonShape::Circle,
                                    icon: Some(IconType::ChevronLeft),
                                }
                                Button {
                                    button_type: ButtonType::Primary,
                                    variant: ButtonVariant::Icon,
                                    shape: ButtonShape::Circle,
                                    icon: Some(IconType::ChevronRight),
                                }
                                Button {
                                    button_type: ButtonType::Info,
                                    variant: ButtonVariant::Icon,
                                    shape: ButtonShape::Circle,
                                    icon: Some(IconType::Settings),
                                }
                            }
                        }
                    }

                    // 虚线边框按钮
                    section {
                        id: "dashed",
                        class: "mb-12",
                        h2 {
                            class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                            "虚线边框按钮"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300 mb-4",
                            "虚线边框按钮提供了一种特殊的视觉样式，适用于需要突出显示但不想过于强烈的场景。"
                        }

                        DemoBox {
                            title: "虚线边框按钮".to_string(),
                            description: "使用 dashed 属性创建虚线边框按钮。".to_string(),
                            code: r#"use helixui::components::{Button, ButtonType};

rsx! {
    div { class: "flex gap-4",
        Button {
            button_type: ButtonType::Default,
            dashed: true,
            "默认虚线"
        }
        Button {
            button_type: ButtonType::Primary,
            dashed: true,
            "主要虚线"
        }
        Button {
            button_type: ButtonType::Info,
            dashed: true,
            "信息虚线"
        }
        Button {
            button_type: ButtonType::Success,
            dashed: true,
            "成功虚线"
        }
        Button {
            button_type: ButtonType::Warning,
            dashed: true,
            "警告虚线"
        }
        Button {
            button_type: ButtonType::Error,
            dashed: true,
            "错误虚线"
        }
    }
}"#.to_string(),
                            children: rsx! {
                                div {
                                    class: "flex flex-wrap gap-4",
                                    Button {
                                        button_type: ButtonType::Default,
                                        dashed: true,
                                        "默认虚线"
                                    }
                                    Button {
                                        button_type: ButtonType::Primary,
                                        dashed: true,
                                        "主要虚线"
                                    }
                                    Button {
                                        button_type: ButtonType::Info,
                                        dashed: true,
                                        "信息虚线"
                                    }
                                    Button {
                                        button_type: ButtonType::Success,
                                        dashed: true,
                                        "成功虚线"
                                    }
                                    Button {
                                        button_type: ButtonType::Warning,
                                        dashed: true,
                                        "警告虚线"
                                    }
                                    Button {
                                        button_type: ButtonType::Error,
                                        dashed: true,
                                        "错误虚线"
                                    }
                                }
                            }
                        }
                    }

                    // 纯文字和纯图标按钮
                    section {
                        id: "pure",
                        class: "mb-12",
                        h2 {
                            class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                            "纯文字和纯图标按钮"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300 mb-4",
                            "纯文字和纯图标按钮没有背景色和边框，只有文字或图标，鼠标悬停时只改变文字和图标的颜色，可点击区域限制在文字和图标本身。"
                        }

                        DemoBox {
                            title: "纯文字按钮".to_string(),
                            description: "纯文字按钮没有背景色和边框，只有文字内容，鼠标悬停时文字颜色变深，可点击区域限制在文字本身。".to_string(),
                            code: r#"use helixui::components::{Button, ButtonType};

rsx! {
    div { class: "flex gap-4",
        Button {
            button_type: ButtonType::PureText,
            "纯文字按钮"
        }
        Button {
            button_type: ButtonType::PureText,
            "链接样式"
        }
        Button {
            button_type: ButtonType::PureText,
            "取消操作"
        }
    }
}"#.to_string(),
                            children: rsx! {
                                div {
                                    class: "flex flex-wrap gap-4",
                                    Button {
                                        button_type: ButtonType::PureText,
                                        "纯文字按钮"
                                    }
                                    Button {
                                        button_type: ButtonType::PureText,
                                        "链接样式"
                                    }
                                    Button {
                                        button_type: ButtonType::PureText,
                                        "取消操作"
                                    }
                                }
                            }
                        }

                        DemoBox {
                            title: "纯图标按钮".to_string(),
                            description: "纯图标按钮没有背景色和边框，只有图标，鼠标悬停时图标颜色变深，可点击区域限制在图标本身。".to_string(),
                            code: r#"use helixui::components::{Button, ButtonType, ButtonVariant, IconType};

rsx! {
    div { class: "flex gap-4",
        Button {
            button_type: ButtonType::PureIcon,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::Settings),
        }
        Button {
            button_type: ButtonType::PureIcon,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::User),
        }
        Button {
            button_type: ButtonType::PureIcon,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::Close),
        }
        Button {
            button_type: ButtonType::PureIcon,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::ChevronLeft),
        }
        Button {
            button_type: ButtonType::PureIcon,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::ChevronRight),
        }
    }
}"#.to_string(),
                            children: rsx! {
                                div {
                                    class: "flex flex-wrap gap-4",
                                    Button {
                                        button_type: ButtonType::PureIcon,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::Settings),
                                    }
                                    Button {
                                        button_type: ButtonType::PureIcon,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::User),
                                    }
                                    Button {
                                        button_type: ButtonType::PureIcon,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::Close),
                                    }
                                    Button {
                                        button_type: ButtonType::PureIcon,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::ChevronLeft),
                                    }
                                    Button {
                                        button_type: ButtonType::PureIcon,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::ChevronRight),
                                    }
                                }
                            }
                        }

                        DemoBox {
                            title: "纯按钮组合使用".to_string(),
                            description: "纯文字和纯图标按钮可以组合使用，创建简洁的工具栏，每个按钮的可点击区域都限制在文字和图标本身。".to_string(),
                            code: r#"use helixui::components::{Button, ButtonType, ButtonVariant, IconType};

rsx! {
    div { class: "flex items-center gap-2",
        Button {
            button_type: ButtonType::PureIcon,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::ChevronLeft),
        }
        Button {
            button_type: ButtonType::PureText,
            "上一页"
        }
        Button {
            button_type: ButtonType::PureText,
            "下一页"
        }
        Button {
            button_type: ButtonType::PureIcon,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::ChevronRight),
        }
    }
}"#.to_string(),
                            children: rsx! {
                                div {
                                    class: "flex items-center gap-2",
                                    Button {
                                        button_type: ButtonType::PureIcon,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::ChevronLeft),
                                    }
                                    Button {
                                        button_type: ButtonType::PureText,
                                        "上一页"
                                    }
                                    Button {
                                        button_type: ButtonType::PureText,
                                        "下一页"
                                    }
                                    Button {
                                        button_type: ButtonType::PureIcon,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::ChevronRight),
                                    }
                                }
                            }
                        }

                        DemoBox {
                            title: "自定义 hover 颜色".to_string(),
                            description: "纯文字和纯图标按钮支持自定义 hover 颜色，使用 hover_color 属性指定颜色。".to_string(),
                            code: "use helixui::components::{Button, ButtonType, ButtonVariant, IconType};

rsx! {
    div { class: \"flex gap-4\",
        Button {
            button_type: ButtonType::PureText,
            hover_color: Some(\"#ff6b6b\".to_string()),
            \"红色 hover\"
        }
        Button {
            button_type: ButtonType::PureText,
            hover_color: Some(\"#4ecdc4\".to_string()),
            \"青色 hover\"
        }
        Button {
            button_type: ButtonType::PureIcon,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::Settings),
            hover_color: Some(\"#ff6b6b\".to_string()),
        }
        Button {
            button_type: ButtonType::PureIcon,
            variant: ButtonVariant::Icon,
            icon: Some(IconType::User),
            hover_color: Some(\"#4ecdc4\".to_string()),
        }
    }
}".to_string(),
                            children: rsx! {
                                div {
                                    class: "flex flex-wrap gap-4",
                                    Button {
                                        button_type: ButtonType::PureText,
                                        hover_color: Some("#ff6b6b".to_string()),
                                        "红色 hover"
                                    }
                                    Button {
                                        button_type: ButtonType::PureText,
                                        hover_color: Some("#4ecdc4".to_string()),
                                        "青色 hover"
                                    }
                                    Button {
                                        button_type: ButtonType::PureIcon,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::Settings),
                                        hover_color: Some("#ff6b6b".to_string()),
                                    }
                                    Button {
                                        button_type: ButtonType::PureIcon,
                                        variant: ButtonVariant::Icon,
                                        icon: Some(IconType::User),
                                        hover_color: Some("#4ecdc4".to_string()),
                                    }
                                }
                            }
                        }
                    }

                    // 按钮组
                    section {
                        id: "group",
                        class: "mb-12",
                        h2 {
                            class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                            "按钮组"
                        }
                        p {
                            class: "text-gray-600 dark:text-gray-300 mb-4",
                            "按钮组可以将多个相关按钮组合在一起，支持紧凑模式（按钮之间无间距）和普通模式。"
                        }

                        DemoBox {
                            title: "普通按钮组".to_string(),
                            description: "普通模式的按钮组，按钮之间有间距。".to_string(),
                            code: r#"use helixui::components::{ButtonGroup, ButtonGroupItemProps, ButtonType, ButtonVariant, IconType};

rsx! {
    ButtonGroup {
        buttons: vec![
            ButtonGroupItemProps {
                button_type: ButtonType::Primary,
                children: rsx! { "保存" },
                ..Default::default()
            },
            ButtonGroupItemProps {
                button_type: ButtonType::Default,
                children: rsx! { "取消" },
                ..Default::default()
            },
        ],
    }
}"#.to_string(),
                            children: rsx! {
                                ButtonGroup {
                                    buttons: vec![
                                        ButtonGroupItemProps {
                                            button_type: ButtonType::Primary,
                                            children: rsx! { "保存" },
                                            ..Default::default()
                                        },
                                        ButtonGroupItemProps {
                                            button_type: ButtonType::Default,
                                            children: rsx! { "取消" },
                                            ..Default::default()
                                        },
                                    ],
                                }
                            }
                        }

                        DemoBox {
                            title: "紧凑按钮组".to_string(),
                            description: "紧凑模式的按钮组，按钮之间无间距，两侧按钮为半圆边框。".to_string(),
                            code: r#"use helixui::components::{ButtonGroup, ButtonGroupItemProps, ButtonType, ButtonVariant, IconType};

rsx! {
    ButtonGroup {
        compact: true,
        buttons: vec![
            ButtonGroupItemProps {
                button_type: ButtonType::Primary,
                children: rsx! { "上一页" },
                ..Default::default()
            },
            ButtonGroupItemProps {
                button_type: ButtonType::Default,
                children: rsx! { "1" },
                ..Default::default()
            },
            ButtonGroupItemProps {
                button_type: ButtonType::Default,
                children: rsx! { "2" },
                ..Default::default()
            },
            ButtonGroupItemProps {
                button_type: ButtonType::Default,
                children: rsx! { "3" },
                ..Default::default()
            },
            ButtonGroupItemProps {
                button_type: ButtonType::Primary,
                children: rsx! { "下一页" },
                ..Default::default()
            },
        ],
    }
}"#.to_string(),
                            children: rsx! {
                                ButtonGroup {
                                    compact: true,
                                    buttons: vec![
                                        ButtonGroupItemProps {
                                            button_type: ButtonType::Primary,
                                            children: rsx! { "上一页" },
                                            ..Default::default()
                                        },
                                        ButtonGroupItemProps {
                                            button_type: ButtonType::Default,
                                            children: rsx! { "1" },
                                            ..Default::default()
                                        },
                                        ButtonGroupItemProps {
                                            button_type: ButtonType::Default,
                                            children: rsx! { "2" },
                                            ..Default::default()
                                        },
                                        ButtonGroupItemProps {
                                            button_type: ButtonType::Default,
                                            children: rsx! { "3" },
                                            ..Default::default()
                                        },
                                        ButtonGroupItemProps {
                                            button_type: ButtonType::Primary,
                                            children: rsx! { "下一页" },
                                            ..Default::default()
                                        },
                                    ],
                                }
                            }
                        }

                        DemoBox {
                            title: "图标按钮组".to_string(),
                            description: "包含图标按钮的按钮组。".to_string(),
                            code: r#"use helixui::components::{ButtonGroup, ButtonGroupItemProps, ButtonType, ButtonVariant, IconType};

rsx! {
    ButtonGroup {
        compact: true,
        buttons: vec![
            ButtonGroupItemProps {
                button_type: ButtonType::Default,
                variant: ButtonVariant::Icon,
                icon: Some(IconType::ChevronLeft),
                children: rsx! {},
                ..Default::default()
            },
            ButtonGroupItemProps {
                button_type: ButtonType::Default,
                variant: ButtonVariant::Icon,
                icon: Some(IconType::ChevronRight),
                children: rsx! {},
                ..Default::default()
            },
        ],
    }
}"#.to_string(),
                            children: rsx! {
                                ButtonGroup {
                                    compact: true,
                                    buttons: vec![
                                        ButtonGroupItemProps {
                                            button_type: ButtonType::Default,
                                            variant: ButtonVariant::Icon,
                                            icon: Some(IconType::ChevronLeft),
                                            children: rsx! {},
                                            ..Default::default()
                                        },
                                        ButtonGroupItemProps {
                                            button_type: ButtonType::Default,
                                            variant: ButtonVariant::Icon,
                                            icon: Some(IconType::ChevronRight),
                                            children: rsx! {},
                                            ..Default::default()
                                        },
                                    ],
                                }
                            }
                        }
                    }

                    // API
                    section {
                id: "api",
                class: "mb-12",
                h2 {
                    class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                    "API"
                }

                h3 {
                    class: "text-xl font-semibold text-gray-900 dark:text-white mb-3",
                    "Button Props"
                }

                div {
                    class: "overflow-x-auto",
                    table {
                        class: "w-full text-left border-collapse",
                        thead {
                            tr {
                                class: "border-b border-gray-200 dark:border-gray-700",
                                th { class: "p-3 text-gray-900 dark:text-white font-semibold", "名称" }
                                th { class: "p-3 text-gray-900 dark:text-white font-semibold", "类型" }
                                th { class: "p-3 text-gray-900 dark:text-white font-semibold", "默认值" }
                                th { class: "p-3 text-gray-900 dark:text-white font-semibold", "说明" }
                            }
                        }
                        tbody {
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "button_type" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonType" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "Default" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮的类型" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "size" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonSize" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "Medium" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮的尺寸" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "disabled" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "是否禁用" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "secondary" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "是否为次要按钮" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "dashed" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "是否为虚线边框" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "variant" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonVariant" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "Text" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮变体（Text/Icon/IconText）" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "icon" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<IconType>" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮图标（仅 Icon 和 IconText 变体）" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "shape" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonShape" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "Rounded" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮形状（Default/Rounded/Circle/Ellipse）" }
                            }
                            tr {
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "class" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<String>" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "自定义 CSS 类名" }
                            }
                        }
                    }
                }

                h3 {
                    class: "text-xl font-semibold text-gray-900 dark:text-white mb-3 mt-8",
                    "ButtonGroup Props"
                }

                div {
                    class: "overflow-x-auto",
                    table {
                        class: "w-full border-collapse border border-gray-200 dark:border-gray-700",
                        thead {
                            tr {
                                class: "bg-gray-50 dark:bg-gray-800",
                                th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "属性" }
                                th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "类型" }
                                th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "默认值" }
                                th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "说明" }
                            }
                        }
                        tbody {
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "buttons" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Vec<ButtonGroupItemProps>" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "-" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮列表" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "size" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonSize" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "Medium" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮组尺寸" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "shape" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonShape" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "Rounded" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮组形状" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "compact" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "是否紧凑模式（按钮之间无间距，两侧半圆边框）" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "class" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<String>" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "自定义 CSS 类名" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "hover_color" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<String>" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "自定义 hover 颜色（如 #ff6b6b）" }
                            }
                        }
                    }
                }

                h3 {
                    class: "text-xl font-semibold text-gray-900 dark:text-white mb-3 mt-8",
                    "ButtonGroupItemProps"
                }

                div {
                    class: "overflow-x-auto",
                    table {
                        class: "w-full border-collapse border border-gray-200 dark:border-gray-700",
                        thead {
                            tr {
                                class: "bg-gray-50 dark:bg-gray-800",
                                th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "属性" }
                                th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "类型" }
                                th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "默认值" }
                                th { class: "p-3 text-left text-gray-900 dark:text-white font-semibold", "说明" }
                            }
                        }
                        tbody {
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "button_type" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonType" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "Default" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮类型" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "variant" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "ButtonVariant" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "Text" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮变体" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "disabled" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "是否禁用" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "secondary" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "是否为次要样式" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "dashed" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "是否为虚线边框" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "icon" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<IconType>" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮图标" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "onclick" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Option<EventHandler<()>>" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "None" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "点击事件处理器" }
                            }
                            tr {
                                class: "border-b border-gray-100 dark:border-gray-800",
                                td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "children" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "Element" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "-" }
                                td { class: "p-3 text-gray-600 dark:text-gray-300", "按钮内容" }
                            }
                        }
                    }
                }
            }
            }
        }
    }
}

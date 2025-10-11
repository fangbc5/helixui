use dioxus::prelude::*;

/// Footer 链接项
#[derive(Props, Clone, PartialEq)]
pub struct FooterLink {
    pub text: String,
    pub href: Option<String>,
}

/// Footer 分类
#[derive(Props, Clone, PartialEq)]
pub struct FooterSection {
    pub title: String,
    pub links: Vec<FooterLink>,
}

/// Footer 组件属性
#[derive(Props, Clone, PartialEq)]
pub struct FooterProps {
    #[props(default)]
    pub sections: Vec<FooterSection>,
    #[props(default)]
    pub version: Option<String>,
    #[props(default)]
    pub made_by: Option<String>,
}

/// Footer 组件
#[component]
pub fn Footer(props: FooterProps) -> Element {
    rsx! {
        footer {
            class: "bg-gray-50 dark:bg-gray-900 border-t border-gray-200 dark:border-gray-700",

            // Footer 内容区域
            div {
                class: "container mx-auto px-4 py-12",

                // Footer 链接区域
                if !props.sections.is_empty() {
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8 mb-8",

                        for section in props.sections.iter() {
                            div {
                                class: "space-y-3",

                                h3 {
                                    class: "text-sm font-semibold text-gray-900 dark:text-white uppercase tracking-wider",
                                    "{section.title}"
                                }

                                ul {
                                    class: "space-y-2",

                                    for link in section.links.iter() {
                                        li {
                                            if let Some(href) = &link.href {
                                                a {
                                                    href: href.as_str(),
                                                    class: "text-sm text-gray-600 dark:text-gray-400 hover:text-green-600 dark:hover:text-green-400 transition-colors",
                                                    "{link.text}"
                                                }
                                            } else {
                                                span {
                                                    class: "text-sm text-gray-600 dark:text-gray-400",
                                                    "{link.text}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 版权信息
                div {
                    class: "pt-8 border-t border-gray-200 dark:border-gray-700",

                    div {
                        class: "flex flex-col md:flex-row md:items-center md:justify-between space-y-2 md:space-y-0",

                        // 左侧版权信息
                        div {
                            class: "text-sm text-gray-500 dark:text-gray-400",
                            if let Some(version) = &props.version {
                                span {
                                    "HelixUI {version}"
                                }
                                span {
                                    class: "mx-2",
                                    "·"
                                }
                            }
                            if let Some(made_by) = &props.made_by {
                                span {
                                    "Made by {made_by}"
                                }
                            } else {
                                span {
                                    "Made with ❤️ by HelixUI Team"
                                }
                            }
                        }

                        // 右侧设置图标
                        div {
                            class: "flex items-center space-x-4",

                            button {
                                class: "p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors",
                                title: "设置",
                                svg {
                                    class: "w-4 h-4",
                                    fill: "none",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                                    }
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 默认的 Footer 组件（使用预设内容）
#[component]
pub fn DefaultFooter() -> Element {
    let sections = vec![
        FooterSection {
            title: "资源".to_string(),
            links: vec![
                FooterLink {
                    text: "设计资源".to_string(),
                    href: Some("/docs/guide".to_string()),
                },
                FooterLink {
                    text: "图标库".to_string(),
                    href: Some("/component/icon".to_string()),
                },
                FooterLink {
                    text: "社区精选资源".to_string(),
                    href: Some("https://github.com/fangbc5/helixui".to_string()),
                },
            ],
        },
        FooterSection {
            title: "帮助".to_string(),
            links: vec![
                FooterLink {
                    text: "常见问题".to_string(),
                    href: Some("/docs/guide".to_string()),
                },
                FooterLink {
                    text: "更新日志".to_string(),
                    href: Some("/docs/version".to_string()),
                },
                FooterLink {
                    text: "报告 Bug".to_string(),
                    href: Some("https://github.com/fangbc5/helixui/issues".to_string()),
                },
            ],
        },
        FooterSection {
            title: "社区".to_string(),
            links: vec![
                FooterLink {
                    text: "GitHub".to_string(),
                    href: Some("https://github.com/fangbc5/helixui".to_string()),
                },
                FooterLink {
                    text: "Discord".to_string(),
                    href: Some("https://discord.gg/BxxwDrhw".to_string()),
                },
                FooterLink {
                    text: "钉钉".to_string(),
                    href: Some("#".to_string()),
                },
            ],
        },
        FooterSection {
            title: "联系我们".to_string(),
            links: vec![FooterLink {
                text: "Beem".to_string(),
                href: Some("https://www.beem.sa/".to_string()),
            }],
        },
    ];

    rsx! {
        Footer {
            sections,
            version: Some("0.1.0".to_string()),
            made_by: Some("HelixUI Team".to_string()),
        }
    }
}

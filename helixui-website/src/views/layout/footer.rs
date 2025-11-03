use dioxus::prelude::*;
use helixui::{AlignItems, Flex, Justify};

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
            class: "bg-gray-50 dark:bg-gray-900 dark:border-gray-700",

            // Footer 内容区域（占满宽度，不再用 container 限制最大宽度）
            div {
                class: "max-w-none px-4 pt-8",

                // Footer 链接区域
                if !props.sections.is_empty() {
                    Flex {
                        wrap: true,
                        justify: Some(Justify::SpaceAround),
                        gap: Some(16),
                        class: Some("mb-4".to_string()),

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

                // 版权信息（简洁版，居中一行）
                Flex {
                    justify: Some(Justify::Center),
                    align: Some(AlignItems::Center),
                    class: Some("py-4 border-t border-gray-200 dark:border-gray-700 w-full".to_string()),
                    p {
                        class: "text-xs text-gray-500 dark:text-gray-400",
                        if let Some(version) = &props.version {
                            span { "HelixUI {version} · " }
                        }
                        span {
                            if let Some(made_by) = &props.made_by { "Made by {made_by}" } else { "Made with ❤️ by HelixUI Team" }
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
                    href: Some("https://discord.gg/62xP3wRDXf".to_string()),
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

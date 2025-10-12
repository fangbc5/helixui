use helixui::components::{DemoBox, Icon, IconSize, IconType};
use crate::views::layout::TocItem;
use crate::views::layout::{ComponentsSidebar, DocPage};
use dioxus::prelude::*;

/// Icon 组件文档页面
#[component]
pub fn IconPage() -> Element {
    // 定义目录项
    let toc_items = vec![
        TocItem {
            id: "basic".to_string(),
            title: "演示".to_string(),
            level: 1,
        },
        TocItem {
            id: "size".to_string(),
            title: "尺寸".to_string(),
            level: 1,
        },
        TocItem {
            id: "color".to_string(),
            title: "颜色".to_string(),
            level: 1,
        },
        TocItem {
            id: "status".to_string(),
            title: "状态".to_string(),
            level: 1,
        },
        TocItem {
            id: "loading".to_string(),
            title: "加载".to_string(),
            level: 1,
        },
        TocItem {
            id: "basic-operations".to_string(),
            title: "基础操作".to_string(),
            level: 1,
        },
        TocItem {
            id: "navigation".to_string(),
            title: "导航箭头".to_string(),
            level: 1,
        },
        TocItem {
            id: "status-feedback".to_string(),
            title: "状态反馈".to_string(),
            level: 1,
        },
        TocItem {
            id: "search-filter".to_string(),
            title: "搜索筛选".to_string(),
            level: 1,
        },
        TocItem {
            id: "user-account".to_string(),
            title: "用户账户".to_string(),
            level: 1,
        },
        TocItem {
            id: "file-document".to_string(),
            title: "文件文档".to_string(),
            level: 1,
        },
        TocItem {
            id: "communication".to_string(),
            title: "通信社交".to_string(),
            level: 1,
        },
        TocItem {
            id: "media-playback".to_string(),
            title: "媒体播放".to_string(),
            level: 1,
        },
        TocItem {
            id: "shopping-business".to_string(),
            title: "购物商务".to_string(),
            level: 1,
        },
        TocItem {
            id: "location-map".to_string(),
            title: "位置地图".to_string(),
            level: 1,
        },
        TocItem {
            id: "time-date".to_string(),
            title: "时间日期".to_string(),
            level: 1,
        },
        TocItem {
            id: "settings-tools".to_string(),
            title: "设置工具".to_string(),
            level: 1,
        },
        TocItem {
            id: "interface-layout".to_string(),
            title: "界面布局".to_string(),
            level: 1,
        },
        TocItem {
            id: "theme-appearance".to_string(),
            title: "主题外观".to_string(),
            level: 1,
        },
        TocItem {
            id: "network-connection".to_string(),
            title: "网络连接".to_string(),
            level: 1,
        },
        TocItem {
            id: "brand-social".to_string(),
            title: "品牌社交".to_string(),
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
                        "图标 Icon"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300",
                        "语义化的矢量图形，提供丰富的图标库。"
                    }
                }

                // 基础用法
                section {
                    id: "basic",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "基础用法"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "通过 icon 属性指定图标类型。"
                    }

                    DemoBox {
                        title: "基础图标".to_string(),
                        description: "最基本的图标用法".to_string(),
                        code: r#"use helixui::components::{Icon, IconType};

rsx! {
    Icon { icon: IconType::Home }
    Icon { icon: IconType::User }
    Icon { icon: IconType::Settings }
    Icon { icon: IconType::Search }
    Icon { icon: IconType::Menu }
}"#.to_string(),

                        div {
                            class: "flex items-center gap-4 flex-wrap",
                            Icon { icon: IconType::Home }
                            Icon { icon: IconType::User }
                            Icon { icon: IconType::Settings }
                            Icon { icon: IconType::Search }
                            Icon { icon: IconType::Menu }
                        }
                    }
                }

                // 图标尺寸
                section {
                    id: "size",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "图标尺寸"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "通过 size 属性设置图标大小。"
                    }

                    DemoBox {
                        title: "不同尺寸".to_string(),
                        description: "支持 Small、Medium、Large、XLarge 四种尺寸".to_string(),
                        code: r#"use helixui::components::{Icon, IconSize, IconType};

rsx! {
    Icon { icon: IconType::Home, size: IconSize::Small }
    Icon { icon: IconType::Home, size: IconSize::Medium }
    Icon { icon: IconType::Home, size: IconSize::Large }
    Icon { icon: IconType::Home, size: IconSize::XLarge }
}"#.to_string(),

                        div {
                            class: "flex items-center gap-6",
                            div {
                                class: "flex flex-col items-center gap-2",
                                Icon { icon: IconType::Home, size: IconSize::Small }
                                span { class: "text-xs text-gray-500 dark:text-gray-400", "Small" }
                            }
                            div {
                                class: "flex flex-col items-center gap-2",
                                Icon { icon: IconType::Home, size: IconSize::Medium }
                                span { class: "text-xs text-gray-500 dark:text-gray-400", "Medium" }
                            }
                            div {
                                class: "flex flex-col items-center gap-2",
                                Icon { icon: IconType::Home, size: IconSize::Large }
                                span { class: "text-xs text-gray-500 dark:text-gray-400", "Large" }
                            }
                            div {
                                class: "flex flex-col items-center gap-2",
                                Icon { icon: IconType::Home, size: IconSize::XLarge }
                                span { class: "text-xs text-gray-500 dark:text-gray-400", "XLarge" }
                            }
                        }
                    }
                }

                // 图标颜色
                section {
                    id: "color",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "图标颜色"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "使用 Tailwind CSS 的 text-* 类名自定义颜色。"
                    }

                    DemoBox {
                        title: "彩色图标".to_string(),
                        description: "通过 class 属性设置图标颜色".to_string(),
                        code: r#"use helixui::components::{Icon, IconType};

rsx! {
    Icon { icon: IconType::Home, class: "text-blue-500".to_string() }
    Icon { icon: IconType::User, class: "text-green-500".to_string() }
    Icon { icon: IconType::Settings, class: "text-purple-500".to_string() }
    Icon { icon: IconType::Search, class: "text-orange-500".to_string() }
    Icon { icon: IconType::Menu, class: "text-red-500".to_string() }
}"#.to_string(),

                        div {
                            class: "flex items-center gap-4",
                            Icon { icon: IconType::Home, class: "text-blue-500".to_string() }
                            Icon { icon: IconType::User, class: "text-green-500".to_string() }
                            Icon { icon: IconType::Settings, class: "text-purple-500".to_string() }
                            Icon { icon: IconType::Search, class: "text-orange-500".to_string() }
                            Icon { icon: IconType::Menu, class: "text-red-500".to_string() }
                        }
                    }
                }

                // 状态图标
                section {
                    id: "status",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "状态图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "常用的状态指示图标。"
                    }

                    div {
                        class: "p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700",
                        div {
                            class: "grid grid-cols-2 md:grid-cols-4 gap-4",

                            div {
                                class: "flex items-center gap-2",
                                Icon { icon: IconType::Success, class: "text-green-500".to_string() }
                                span { class: "text-gray-700 dark:text-gray-300", "Success" }
                            }

                            div {
                                class: "flex items-center gap-2",
                                Icon { icon: IconType::Info, class: "text-blue-500".to_string() }
                                span { class: "text-gray-700 dark:text-gray-300", "Info" }
                            }

                            div {
                                class: "flex items-center gap-2",
                                Icon { icon: IconType::Warning, class: "text-yellow-500".to_string() }
                                span { class: "text-gray-700 dark:text-gray-300", "Warning" }
                            }

                            div {
                                class: "flex items-center gap-2",
                                Icon { icon: IconType::Error, class: "text-red-500".to_string() }
                                span { class: "text-gray-700 dark:text-gray-300", "Error" }
                            }
                        }
                    }
                }

                // 加载动画
                section {
                    id: "loading",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "加载动画"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "设置 spin 属性使图标旋转。"
                    }

                    DemoBox {
                        title: "旋转动画".to_string(),
                        description: "任何图标都可以设置 spin 属性实现旋转效果".to_string(),
                        code: r#"use helixui::components::{Icon, IconSize, IconType};

rsx! {
    Icon { 
        icon: IconType::Loading, 
        spin: true, 
        class: "text-blue-500".to_string() 
    }
    Icon { 
        icon: IconType::Loading, 
        spin: true, 
        size: IconSize::Large, 
        class: "text-green-500".to_string() 
    }
    Icon { 
        icon: IconType::Settings, 
        spin: true, 
        class: "text-purple-500".to_string() 
    }
}"#.to_string(),

                        div {
                            class: "flex items-center gap-4",
                            Icon { icon: IconType::Loading, spin: true, class: "text-blue-500".to_string() }
                            Icon { icon: IconType::Loading, spin: true, size: IconSize::Large, class: "text-green-500".to_string() }
                            Icon { icon: IconType::Settings, spin: true, class: "text-purple-500".to_string() }
                        }
                    }
                }

                // 基础操作图标
                section {
                    id: "basic-operations",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "基础操作图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "常用的基础操作图标，包括增删改查、保存、下载等操作。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Check", IconType::Check),
                            ("Close", IconType::Close),
                            ("Plus", IconType::Plus),
                            ("Minus", IconType::Minus),
                            ("Edit", IconType::Edit),
                            ("Delete", IconType::Delete),
                            ("Copy", IconType::Copy),
                            ("Cut", IconType::Cut),
                            ("Paste", IconType::Paste),
                            ("Undo", IconType::Undo),
                            ("Redo", IconType::Redo),
                            ("Save", IconType::Save),
                            ("Download", IconType::Download),
                            ("Upload", IconType::Upload),
                            ("Print", IconType::Print),
                            ("Refresh", IconType::Refresh),
                            ("Loading", IconType::Loading),
                        ]
                    }
                }

                // 导航箭头图标
                section {
                    id: "navigation",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "导航箭头图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于导航和方向指示的箭头图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("ChevronDown", IconType::ChevronDown),
                            ("ChevronUp", IconType::ChevronUp),
                            ("ChevronLeft", IconType::ChevronLeft),
                            ("ChevronRight", IconType::ChevronRight),
                            ("ChevronDoubleDown", IconType::ChevronDoubleDown),
                            ("ChevronDoubleUp", IconType::ChevronDoubleUp),
                            ("ChevronDoubleLeft", IconType::ChevronDoubleLeft),
                            ("ChevronDoubleRight", IconType::ChevronDoubleRight),
                            ("ArrowUp", IconType::ArrowUp),
                            ("ArrowDown", IconType::ArrowDown),
                            ("ArrowLeft", IconType::ArrowLeft),
                            ("ArrowRight", IconType::ArrowRight),
                            ("ArrowUpLeft", IconType::ArrowUpLeft),
                            ("ArrowUpRight", IconType::ArrowUpRight),
                            ("ArrowDownLeft", IconType::ArrowDownLeft),
                            ("ArrowDownRight", IconType::ArrowDownRight),
                        ]
                    }
                }

                // 状态反馈图标
                section {
                    id: "status-feedback",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "状态反馈图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于显示各种状态和反馈信息的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Info", IconType::Info),
                            ("Warning", IconType::Warning),
                            ("Error", IconType::Error),
                            ("Success", IconType::Success),
                            ("Question", IconType::Question),
                            ("Exclamation", IconType::Exclamation),
                            ("Ban", IconType::Ban),
                            ("Lock", IconType::Lock),
                            ("Unlock", IconType::Unlock),
                            ("Eye", IconType::Eye),
                            ("EyeSlash", IconType::EyeSlash),
                        ]
                    }
                }

                // 搜索和筛选图标
                section {
                    id: "search-filter",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "搜索和筛选图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于搜索、筛选和排序功能的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Search", IconType::Search),
                            ("Filter", IconType::Filter),
                            ("Sort", IconType::Sort),
                            ("SortAsc", IconType::SortAsc),
                            ("SortDesc", IconType::SortDesc),
                            ("Grid", IconType::Grid),
                            ("List", IconType::List),
                            ("Table", IconType::Table),
                        ]
                    }
                }

                // 用户和账户图标
                section {
                    id: "user-account",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "用户和账户图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "与用户管理、账户和权限相关的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("User", IconType::User),
                            ("Users", IconType::Users),
                            ("UserPlus", IconType::UserPlus),
                            ("UserMinus", IconType::UserMinus),
                            ("UserEdit", IconType::UserEdit),
                            ("UserCheck", IconType::UserCheck),
                            ("UserX", IconType::UserX),
                            ("LogIn", IconType::LogIn),
                            ("LogOut", IconType::LogOut),
                            ("Key", IconType::Key),
                            ("Shield", IconType::Shield),
                            ("Verified", IconType::Verified),
                        ]
                    }
                }

                // 文件和文档图标
                section {
                    id: "file-document",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "文件和文档图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于文件管理、文档操作和媒体文件的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("File", IconType::File),
                            ("Folder", IconType::Folder),
                            ("FolderPlus", IconType::FolderPlus),
                            ("FolderMinus", IconType::FolderMinus),
                            ("FolderOpen", IconType::FolderOpen),
                            ("Document", IconType::Document),
                            ("DocumentPlus", IconType::DocumentPlus),
                            ("DocumentEdit", IconType::DocumentEdit),
                            ("DocumentCheck", IconType::DocumentCheck),
                            ("DocumentX", IconType::DocumentX),
                            ("DocumentDownload", IconType::DocumentDownload),
                            ("DocumentUpload", IconType::DocumentUpload),
                            ("Image", IconType::Image),
                            ("ImagePlus", IconType::ImagePlus),
                            ("ImageBroken", IconType::ImageBroken),
                            ("Video", IconType::Video),
                            ("Audio", IconType::Audio),
                            ("FilePdf", IconType::FilePdf),
                            ("Archive", IconType::Archive),
                        ]
                    }
                }

                // 通信和社交图标
                section {
                    id: "communication",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "通信和社交图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于通信、社交和分享功能的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Mail", IconType::Mail),
                            ("MailPlus", IconType::MailPlus),
                            ("MailCheck", IconType::MailCheck),
                            ("MailX", IconType::MailX),
                            ("Phone", IconType::Phone),
                            ("PhonePlus", IconType::PhonePlus),
                            ("PhoneCheck", IconType::PhoneCheck),
                            ("PhoneX", IconType::PhoneX),
                            ("Message", IconType::Message),
                            ("MessagePlus", IconType::MessagePlus),
                            ("MessageCheck", IconType::MessageCheck),
                            ("MessageX", IconType::MessageX),
                            ("Chat", IconType::Chat),
                            ("Bell", IconType::Bell),
                            ("BellPlus", IconType::BellPlus),
                            ("BellX", IconType::BellX),
                            ("Share", IconType::Share),
                            ("Link", IconType::Link),
                            ("LinkBreak", IconType::LinkBreak),
                            ("ExternalLink", IconType::ExternalLink),
                        ]
                    }
                }

                // 媒体和播放图标
                section {
                    id: "media-playback",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "媒体和播放图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于媒体播放、录音和摄像功能的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Play", IconType::Play),
                            ("Pause", IconType::Pause),
                            ("Stop", IconType::Stop),
                            ("FastForward", IconType::FastForward),
                            ("Rewind", IconType::Rewind),
                            ("SkipNext", IconType::SkipNext),
                            ("SkipPrevious", IconType::SkipPrevious),
                            ("Repeat", IconType::Repeat),
                            ("Shuffle", IconType::Shuffle),
                            ("Volume", IconType::Volume),
                            ("VolumePlus", IconType::VolumePlus),
                            ("VolumeMinus", IconType::VolumeMinus),
                            ("VolumeX", IconType::VolumeX),
                            ("Mic", IconType::Mic),
                            ("MicPlus", IconType::MicPlus),
                            ("MicX", IconType::MicX),
                            ("Camera", IconType::Camera),
                            ("CameraPlus", IconType::CameraPlus),
                            ("CameraX", IconType::CameraX),
                        ]
                    }
                }

                // 购物和商务图标
                section {
                    id: "shopping-business",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "购物和商务图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于电商、商务和金融功能的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("ShoppingCart", IconType::ShoppingCart),
                            ("ShoppingCartPlus", IconType::ShoppingCartPlus),
                            ("ShoppingCartMinus", IconType::ShoppingCartMinus),
                            ("ShoppingCartX", IconType::ShoppingCartX),
                            ("ShoppingBag", IconType::ShoppingBag),
                            ("CreditCard", IconType::CreditCard),
                            ("Wallet", IconType::Wallet),
                            ("Coin", IconType::Coin),
                            ("Bank", IconType::Bank),
                            ("Chart", IconType::Chart),
                            ("ChartPlus", IconType::ChartPlus),
                            ("ChartMinus", IconType::ChartMinus),
                            ("TrendingUp", IconType::TrendingUp),
                            ("TrendingDown", IconType::TrendingDown),
                            ("Calculator", IconType::Calculator),
                            ("Receipt", IconType::Receipt),
                        ]
                    }
                }

                // 位置和地图图标
                section {
                    id: "location-map",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "位置和地图图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于地理位置、地图和导航功能的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Location", IconType::Location),
                            ("LocationPlus", IconType::LocationPlus),
                            ("LocationMinus", IconType::LocationMinus),
                            ("LocationX", IconType::LocationX),
                            ("Map", IconType::Map),
                            ("MapPlus", IconType::MapPlus),
                            ("MapMinus", IconType::MapMinus),
                            ("MapX", IconType::MapX),
                            ("Navigation", IconType::Navigation),
                            ("Compass", IconType::Compass),
                            ("Marker", IconType::Marker),
                            ("MarkerPlus", IconType::MarkerPlus),
                            ("MarkerMinus", IconType::MarkerMinus),
                            ("MarkerX", IconType::MarkerX),
                        ]
                    }
                }

                // 时间和日期图标
                section {
                    id: "time-date",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "时间和日期图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于时间管理、日历和计时功能的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Clock", IconType::Clock),
                            ("ClockPlus", IconType::ClockPlus),
                            ("ClockMinus", IconType::ClockMinus),
                            ("ClockX", IconType::ClockX),
                            ("Calendar", IconType::Calendar),
                            ("CalendarPlus", IconType::CalendarPlus),
                            ("CalendarMinus", IconType::CalendarMinus),
                            ("CalendarX", IconType::CalendarX),
                            ("CalendarCheck", IconType::CalendarCheck),
                            ("Timer", IconType::Timer),
                            ("Stopwatch", IconType::Stopwatch),
                            ("History", IconType::History),
                        ]
                    }
                }

                // 设置和工具图标
                section {
                    id: "settings-tools",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "设置和工具图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于设置、工具和配置功能的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Settings", IconType::Settings),
                            ("Gear", IconType::Gear),
                            ("Wrench", IconType::Wrench),
                            ("Screwdriver", IconType::Screwdriver),
                            ("Hammer", IconType::Hammer),
                            ("Tools", IconType::Tools),
                            ("Cog", IconType::Cog),
                            ("Tool", IconType::Tool),
                        ]
                    }
                }

                // 界面和布局图标
                section {
                    id: "interface-layout",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "界面和布局图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于界面布局、窗口管理和导航的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Home", IconType::Home),
                            ("Menu", IconType::Menu),
                            ("MenuBurger", IconType::MenuBurger),
                            ("MenuX", IconType::MenuX),
                            ("Sidebar", IconType::Sidebar),
                            ("SidebarPlus", IconType::SidebarPlus),
                            ("SidebarMinus", IconType::SidebarMinus),
                            ("SidebarX", IconType::SidebarX),
                            ("Panel", IconType::Panel),
                            ("PanelPlus", IconType::PanelPlus),
                            ("PanelMinus", IconType::PanelMinus),
                            ("PanelX", IconType::PanelX),
                            ("Window", IconType::Window),
                            ("WindowPlus", IconType::WindowPlus),
                            ("WindowMinus", IconType::WindowMinus),
                            ("WindowX", IconType::WindowX),
                            ("Maximize", IconType::Maximize),
                            ("Minimize", IconType::Minimize),
                            ("Restore", IconType::Restore),
                            ("Fullscreen", IconType::Fullscreen),
                            ("FullscreenExit", IconType::FullscreenExit),
                        ]
                    }
                }

                // 主题和外观图标
                section {
                    id: "theme-appearance",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "主题和外观图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于主题切换、颜色和外观设置的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Moon", IconType::Moon),
                            ("Sun", IconType::Sun),
                            ("Palette", IconType::Palette),
                            ("Paintbrush", IconType::Paintbrush),
                            ("Color", IconType::Color),
                            ("Contrast", IconType::Contrast),
                            ("Brightness", IconType::Brightness),
                            ("HighContrast", IconType::HighContrast),
                        ]
                    }
                }

                // 网络和连接图标
                section {
                    id: "network-connection",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "网络和连接图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "用于网络连接、信号和云服务的图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("Wifi", IconType::Wifi),
                            ("WifiPlus", IconType::WifiPlus),
                            ("WifiMinus", IconType::WifiMinus),
                            ("WifiX", IconType::WifiX),
                            ("Bluetooth", IconType::Bluetooth),
                            ("BluetoothPlus", IconType::BluetoothPlus),
                            ("BluetoothMinus", IconType::BluetoothMinus),
                            ("BluetoothX", IconType::BluetoothX),
                            ("Signal", IconType::Signal),
                            ("SignalPlus", IconType::SignalPlus),
                            ("SignalMinus", IconType::SignalMinus),
                            ("SignalX", IconType::SignalX),
                            ("Router", IconType::Router),
                            ("Server", IconType::Server),
                            ("Database", IconType::Database),
                            ("Cloud", IconType::Cloud),
                            ("CloudPlus", IconType::CloudPlus),
                            ("CloudMinus", IconType::CloudMinus),
                            ("CloudX", IconType::CloudX),
                            ("CloudDownload", IconType::CloudDownload),
                            ("CloudUpload", IconType::CloudUpload),
                        ]
                    }
                }

                // 品牌和社交图标
                section {
                    id: "brand-social",
                    class: "mb-12",
                    h2 {
                        class: "text-2xl font-semibold text-gray-900 dark:text-white mb-4",
                        "品牌和社交图标"
                    }
                    p {
                        class: "text-gray-600 dark:text-gray-300 mb-4",
                        "各大品牌和社交平台的官方图标。"
                    }

                    IconCategory {
                        icons: vec![
                            ("GitHub", IconType::GitHub),
                            ("Twitter", IconType::Twitter),
                            ("Facebook", IconType::Facebook),
                            ("Instagram", IconType::Instagram),
                            ("LinkedIn", IconType::LinkedIn),
                            ("YouTube", IconType::YouTube),
                            ("Discord", IconType::Discord),
                            ("Slack", IconType::Slack),
                            ("Google", IconType::Google),
                            ("Apple", IconType::Apple),
                            ("Microsoft", IconType::Microsoft),
                            ("Amazon", IconType::Amazon),
                            ("Netflix", IconType::Netflix),
                            ("Spotify", IconType::Spotify),
                        ]
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

                    div {
                        class: "overflow-x-auto",
                        table {
                            class: "w-full text-left border-collapse",
                            thead {
                                tr {
                                    class: "border-b border-gray-200 dark:border-gray-700",
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "属性" }
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "说明" }
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "类型" }
                                    th { class: "p-3 text-gray-900 dark:text-white font-semibold", "默认值" }
                                }
                            }
                            tbody {
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "icon" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "图标类型" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "IconType" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "-" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "size" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "图标尺寸" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "IconSize" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "Medium" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "color" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "图标颜色" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "String" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "currentColor" }
                                }
                                tr {
                                    class: "border-b border-gray-100 dark:border-gray-800",
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "class" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "自定义类名" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "String" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "\"\""  }
                                }
                                tr {
                                    td { class: "p-3 text-gray-900 dark:text-white font-mono text-sm", "spin" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "是否旋转" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300 font-mono text-sm", "bool" }
                                    td { class: "p-3 text-gray-600 dark:text-gray-300", "false" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn IconCategory(icons: Vec<(&'static str, IconType)>) -> Element {
    rsx! {
        div {
            class: "p-6 bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700",
            div {
                class: "grid grid-cols-3 md:grid-cols-6 lg:grid-cols-8 gap-4",
                for (name, icon_type) in icons.iter() {
                    IconItem { icon: *icon_type, name: *name }
                }
            }
        }
    }
}

#[component]
fn IconItem(icon: IconType, name: &'static str) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center p-4 hover:bg-gray-50 dark:hover:bg-gray-700 rounded-lg cursor-pointer transition-colors",
            Icon { icon: icon, size: IconSize::Large }
            span { class: "text-xs text-gray-600 dark:text-gray-400 mt-2 text-center", "{name}" }
        }
    }
}

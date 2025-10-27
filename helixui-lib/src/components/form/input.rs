use dioxus::prelude::*;

/// Input 组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// 输入框的类型
    #[props(default = String::from("text"))]
    pub input_type: String,

    /// 输入框的值
    pub value: Option<String>,

    /// 占位符文本
    #[props(default)]
    pub placeholder: Option<String>,

    /// 是否禁用
    #[props(default = false)]
    pub disabled: bool,

    /// 是否只读
    #[props(default = false)]
    pub readonly: bool,

    /// 值变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,

    /// 额外的class名称
    #[props(default)]
    pub class: Option<String>,
}

/// Input 组件
#[component]
pub fn Input(props: InputProps) -> Element {
    // 基础样式类
    let base_classes = "relative flex box-border flex-row items-center justify-between px-3 py-2 gap-1 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 cursor-text transition-all duration-100 ease-out";

    // 构建完整的class字符串
    let mut classes = base_classes.to_string();
    classes.push_str(" placeholder:text-gray-400 dark:placeholder:text-gray-500"); // placeholder颜色
    classes.push_str(" disabled:cursor-not-allowed disabled:text-gray-400 dark:disabled:text-gray-600 disabled:bg-gray-100 dark:disabled:bg-gray-900"); // disabled状态
    classes.push_str(" hover:border-gray-400 dark:hover:border-gray-500"); // hover状态
    classes.push_str(" focus:border-blue-500 dark:focus:border-blue-400 focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 focus:outline-none"); // focus状态
    classes.push_str(" read-only:bg-gray-50 dark:read-only:bg-gray-900"); // readonly状态

    if !props.class.clone().unwrap_or_default().is_empty() {
        classes.push_str(&format!(" {}", props.class.unwrap_or_default()));
    }

    rsx! {
        input {
            r#type: props.input_type,
            value: props.value,
            placeholder: props.placeholder,
            disabled: props.disabled,
            readonly: props.readonly,
            class: classes,
            onchange: move |e| {
                if let Some(ref handler) = props.on_change {
                    handler.call(e.value());
                }
            },
        }
    }
}

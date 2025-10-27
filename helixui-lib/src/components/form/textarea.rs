use dioxus::prelude::*;

/// 文本域组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct TextAreaProps {
    /// 输入框的值
    #[props(default)]
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
    
    /// 行数
    #[props(default = 3)]
    pub rows: u32,
    
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 值变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
}

/// 文本域组件
#[component]
pub fn TextArea(props: TextAreaProps) -> Element {
    let base_class = "px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:bg-gray-100 disabled:cursor-not-allowed resize-none";
    
    let final_class = if let Some(custom_class) = props.class {
        format!("{} {}", base_class, custom_class)
    } else {
        base_class.to_string()
    };

    rsx! {
        textarea {
            value: props.value.unwrap_or_default(),
            placeholder: props.placeholder.unwrap_or_default(),
            disabled: props.disabled,
            readonly: props.readonly,
            rows: props.rows,
            class: final_class,
            oninput: move |event| {
                if let Some(handler) = &props.on_change {
                    handler.call(event.value());
                }
            }
        }
    }
}

use dioxus::prelude::*;

/// 输入框组件的属性
#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
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
    
    /// 输入框类型
    #[props(default = InputType::Text)]
    pub input_type: InputType,
    
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    
    /// 值变化时的回调
    #[props(default)]
    pub on_change: Option<EventHandler<String>>,
}

/// 输入框类型
#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
    Tel,
    Url,
}

impl InputType {
    fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Email => "email",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
        }
    }
}

/// 输入框组件
#[component]
pub fn Input(props: InputProps) -> Element {
    let base_class = "px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 disabled:bg-gray-100 disabled:cursor-not-allowed";
    
    let final_class = if let Some(custom_class) = props.class {
        format!("{} {}", base_class, custom_class)
    } else {
        base_class.to_string()
    };

    rsx! {
        input {
            r#type: props.input_type.as_str(),
            value: props.value.unwrap_or_default(),
            placeholder: props.placeholder.unwrap_or_default(),
            disabled: props.disabled,
            readonly: props.readonly,
            class: final_class,
            oninput: move |event| {
                if let Some(handler) = &props.on_change {
                    handler.call(event.value());
                }
            }
        }
    }
}

use dioxus::document;
use dioxus::prelude::*;

/// 水印组件属性
#[derive(Props, Clone, PartialEq)]
pub struct WatermarkProps {
    /// 水印文本内容
    pub content: String,
    /// 水印图片 URL（如果提供，将优先使用图片而不是文本）
    #[props(default)]
    pub image: Option<String>,
    /// 字体大小（像素）
    #[props(default = 16.0)]
    pub font_size: f64,
    /// 字体颜色
    #[props(default = String::from("rgba(0,0,0,0.15)"))]
    pub color: String,
    /// 旋转角度（度）
    #[props(default = -22.0)]
    pub rotate: f64,
    /// 水印之间的宽度间隔（像素）
    #[props(default = 100.0)]
    pub gap_x: f64,
    /// 水印之间的高度间隔（像素）
    #[props(default = 100.0)]
    pub gap_y: f64,
    /// 水印块的宽度（像素）
    #[props(default = 120.0)]
    pub width: f64,
    /// 水印块的高度（像素）
    #[props(default = 64.0)]
    pub height: f64,
    /// 字体族
    #[props(default = String::from("Arial"))]
    pub font_family: String,
    /// 是否启用水印
    #[props(default = true)]
    pub enabled: bool,
    /// 自定义类名
    #[props(default)]
    pub class: Option<String>,
    /// 子元素
    pub children: Element,
}

/// 水印组件
#[component]
pub fn Watermark(props: WatermarkProps) -> Element {
    // 生成唯一ID
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let watermark_id = use_signal(|| {
        let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("watermark-{}", id)
    });

    // 使用 use_effect 生成水印背景
    use_effect(move || {
        if !props.enabled {
            return;
        }

        let id = watermark_id.peek().clone();
        let content = props.content.clone();
        let image = props.image.clone();
        let font_size = props.font_size;
        let color = props.color.clone();
        let rotate = props.rotate;
        let gap_x = props.gap_x;
        let gap_y = props.gap_y;
        let width = props.width;
        let height = props.height;
        let font_family = props.font_family.clone();

        spawn(async move {
            let script = if let Some(img_url) = image {
                // 使用图片作为水印
                let escaped_url = img_url.replace('\'', "\\'").replace('\\', "\\\\");
                format!(
                    r#"
                    (function() {{
                        const id = '{id}';
                        function applyWatermark() {{
                            const element = document.getElementById(id);
                            if (!element) {{
                                setTimeout(applyWatermark, 50);
                                return;
                            }}
                            
                            const img = new Image();
                            img.crossOrigin = 'anonymous';
                            img.onload = function() {{
                                const canvas = document.createElement('canvas');
                                const ctx = canvas.getContext('2d');
                                const angle = {rotate} * Math.PI / 180;
                                
                                // 计算旋转后的尺寸
                                const sin = Math.abs(Math.sin(angle));
                                const cos = Math.abs(Math.cos(angle));
                                const w = {width};
                                const h = {height};
                                const rotatedWidth = w * cos + h * sin;
                                const rotatedHeight = w * sin + h * cos;
                                
                                canvas.width = rotatedWidth + {gap_x} * 2;
                                canvas.height = rotatedHeight + {gap_y} * 2;
                                ctx.translate(canvas.width / 2, canvas.height / 2);
                                ctx.rotate(angle);
                                ctx.drawImage(img, -w / 2, -h / 2, w, h);
                                
                                const dataUrl = canvas.toDataURL('image/png');
                                element.style.backgroundImage = `url(${{dataUrl}})`;
                                element.style.backgroundRepeat = 'repeat';
                                element.style.backgroundPosition = '0 0';
                            }};
                            img.onerror = function() {{
                                console.warn('Watermark image load failed');
                            }};
                            img.src = '{escaped_url}';
                        }}
                        applyWatermark();
                    }})();
                    "#
                )
            } else {
                // 使用文本作为水印
                let escaped_content = content
                    .replace('\\', "\\\\")
                    .replace('\'', "\\'")
                    .replace('\n', "\\n")
                    .replace('"', "\\\"");
                format!(
                    r#"
                    (function() {{
                        const id = '{id}';
                        function applyWatermark() {{
                            const element = document.getElementById(id);
                            if (!element) {{
                                setTimeout(applyWatermark, 50);
                                return;
                            }}
                            
                            const canvas = document.createElement('canvas');
                            const ctx = canvas.getContext('2d');
                            const angle = {rotate} * Math.PI / 180;
                            
                            // 计算旋转后的尺寸
                            const sin = Math.abs(Math.sin(angle));
                            const cos = Math.abs(Math.cos(angle));
                            const w = {width};
                            const h = {height};
                            const rotatedWidth = w * cos + h * sin;
                            const rotatedHeight = w * sin + h * cos;
                            
                            canvas.width = rotatedWidth + {gap_x} * 2;
                            canvas.height = rotatedHeight + {gap_y} * 2;
                            
                            ctx.translate(canvas.width / 2, canvas.height / 2);
                            ctx.rotate(angle);
                            ctx.font = `{font_size}px {font_family}`;
                            ctx.fillStyle = '{color}';
                            ctx.textAlign = 'center';
                            ctx.textBaseline = 'middle';
                            
                            // 支持多行文本
                            const lines = '{escaped_content}'.split('\\n');
                            const lineHeight = {font_size} * 1.2;
                            const startY = -(lines.length - 1) * lineHeight / 2;
                            
                            lines.forEach((line, index) => {{
                                ctx.fillText(line, 0, startY + index * lineHeight);
                            }});
                            
                            const dataUrl = canvas.toDataURL('image/png');
                            element.style.backgroundImage = `url(${{dataUrl}})`;
                            element.style.backgroundRepeat = 'repeat';
                            element.style.backgroundPosition = '0 0';
                        }}
                        applyWatermark();
                    }})();
                    "#
                )
            };

            // 执行脚本生成水印
            let mut eval = document::eval(&script);
            _ = eval.recv::<()>().await;
        });
    });

    let base_class = "relative";
    let user_class = props.class.as_deref().unwrap_or("");

    rsx! {
        div {
            class: "{base_class} {user_class}",
            // 内容
            {props.children}
            // 水印覆盖层（置于内容之上，指针不可交互）
            if props.enabled {
                div {
                    id: watermark_id(),
                    class: "pointer-events-none absolute inset-0 z-10",
                }
            }
        }
    }
}

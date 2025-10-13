/// Z-Index 配置
#[derive(Debug, Clone)]
pub struct ZIndexConfig {
    pub base: u32,
    pub step: u32,
    pub max: u32,
    pub message: u32,
    pub modal: u32,
    pub dialog: u32,
    pub popover: u32,
    pub tooltip: u32,
}

impl Default for ZIndexConfig {
    fn default() -> Self {
        Self {
            base: 1000,
            step: 10,
            max: 9999,
            message: 2000,
            modal: 3000,
            dialog: 4000,
            popover: 5000,
            tooltip: 6000,
        }
    }
}

impl ZIndexConfig {
    /// 创建新的 z-index 配置
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置基础 z-index
    pub fn with_base(mut self, base: u32) -> Self {
        self.base = base;
        self
    }

    /// 设置步长
    pub fn with_step(mut self, step: u32) -> Self {
        self.step = step;
        self
    }

    /// 设置最大值
    pub fn with_max(mut self, max: u32) -> Self {
        self.max = max;
        self
    }

    /// 设置消息 z-index
    pub fn with_message(mut self, message: u32) -> Self {
        self.message = message;
        self
    }

    /// 设置模态框 z-index
    pub fn with_modal(mut self, modal: u32) -> Self {
        self.modal = modal;
        self
    }

    /// 设置对话框 z-index
    pub fn with_dialog(mut self, dialog: u32) -> Self {
        self.dialog = dialog;
        self
    }

    /// 设置弹出层 z-index
    pub fn with_popover(mut self, popover: u32) -> Self {
        self.popover = popover;
        self
    }

    /// 设置工具提示 z-index
    pub fn with_tooltip(mut self, tooltip: u32) -> Self {
        self.tooltip = tooltip;
        self
    }

    /// 获取指定类型的 z-index
    pub fn get_z_index(&self, overlay_type: &str) -> u32 {
        match overlay_type {
            "message" => self.message,
            "modal" => self.modal,
            "dialog" => self.dialog,
            "popover" => self.popover,
            "tooltip" => self.tooltip,
            _ => self.base,
        }
    }

    /// 计算下一个 z-index
    pub fn get_next_z_index(&self, current: u32) -> u32 {
        let next = current + self.step;
        if next > self.max {
            self.base
        } else {
            next
        }
    }
}

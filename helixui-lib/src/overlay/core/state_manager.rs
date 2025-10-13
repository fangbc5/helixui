use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

/// 全局状态管理器
pub struct GlobalStateManager {
    overlays: HashMap<String, OverlayState>,
    z_index_stack: Vec<u32>,
    next_z_index: u32,
}

/// Overlay 状态
#[derive(Clone, Debug)]
pub struct OverlayState {
    pub id: String,
    pub component_type: OverlayType,
    pub z_index: u32,
    pub visible: bool,
    pub data: String, // 简化为字符串，实际应用中可以使用 Box<dyn Any>
}

/// Overlay 类型枚举
#[derive(Clone, Debug, PartialEq)]
pub enum OverlayType {
    Message,
    Modal,
    Dialog,
    Popover,
    Tooltip,
    Notification,
    Toast,
}

impl GlobalStateManager {
    pub fn new() -> Self {
        Self {
            overlays: HashMap::new(),
            z_index_stack: Vec::new(),
            next_z_index: 1000,
        }
    }

    /// 添加 Overlay
    pub fn add_overlay(&mut self, id: String, overlay_type: OverlayType, data: String) -> u32 {
        let z_index = self.next_z_index;
        let state = OverlayState {
            id: id.clone(),
            component_type: overlay_type,
            z_index,
            visible: true,
            data,
        };

        self.overlays.insert(id, state);
        self.z_index_stack.push(z_index);
        self.next_z_index += 10;

        z_index
    }

    /// 移除 Overlay
    pub fn remove_overlay(&mut self, id: &str) {
        if let Some(state) = self.overlays.remove(id) {
            self.z_index_stack.retain(|&z| z != state.z_index);
        }
    }

    /// 显示 Overlay
    pub fn show_overlay(&mut self, id: &str) {
        if let Some(state) = self.overlays.get_mut(id) {
            state.visible = true;
        }
    }

    /// 隐藏 Overlay
    pub fn hide_overlay(&mut self, id: &str) {
        if let Some(state) = self.overlays.get_mut(id) {
            state.visible = false;
        }
    }

    /// 获取所有可见的 Overlay
    pub fn get_visible_overlays(&self) -> Vec<OverlayState> {
        self.overlays
            .values()
            .filter(|state| state.visible)
            .cloned()
            .collect()
    }

    /// 获取指定类型的 Overlay
    pub fn get_overlays_by_type(&self, overlay_type: OverlayType) -> Vec<OverlayState> {
        self.overlays
            .values()
            .filter(|state| state.component_type == overlay_type)
            .cloned()
            .collect()
    }

    /// 获取下一个 z-index
    pub fn get_next_z_index(&self) -> u32 {
        self.next_z_index
    }

    /// 清理所有 Overlay
    pub fn clear_all(&mut self) {
        self.overlays.clear();
        self.z_index_stack.clear();
        self.next_z_index = 1000;
    }
}

/// 全局状态管理器实例
static GLOBAL_STATE_MANAGER: LazyLock<Mutex<GlobalStateManager>> =
    LazyLock::new(|| Mutex::new(GlobalStateManager::new()));

/// 获取全局状态管理器
pub fn get_global_state_manager() -> &'static Mutex<GlobalStateManager> {
    &GLOBAL_STATE_MANAGER
}

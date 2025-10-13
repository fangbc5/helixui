use dioxus::prelude::*;
use std::any::Any;
use std::collections::HashMap;

/// Overlay 上下文
#[derive(Debug, Clone)]
pub struct OverlayContext {
    pub id: String,
    pub z_index: u32,
    pub visible: bool,
    pub data: HashMap<String, String>,
}

/// Overlay 插件 trait
pub trait OverlayPlugin: Send + Sync {
    /// 插件名称
    fn name(&self) -> &str;

    /// 插件版本
    fn version(&self) -> &str;

    /// 初始化插件
    fn initialize(&mut self, context: &OverlayContext) -> Result<(), String>;

    /// 清理插件
    fn cleanup(&mut self) -> Result<(), String>;

    /// 处理事件
    fn handle_event(&mut self, event: &str, data: &HashMap<String, String>) -> Result<(), String>;

    /// 获取插件数据
    fn get_data(&self) -> HashMap<String, String>;

    /// 设置插件数据
    fn set_data(&mut self, data: HashMap<String, String>) -> Result<(), String>;
}

/// 插件管理器
#[derive(Clone)]
pub struct PluginManager {
    plugins: Signal<HashMap<String, Box<dyn OverlayPlugin>>>,
    context: Signal<OverlayContext>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: use_signal(|| HashMap::new()),
            context: use_signal(|| OverlayContext {
                id: String::new(),
                z_index: 0,
                visible: false,
                data: HashMap::new(),
            }),
        }
    }

    /// 注册插件
    pub fn register_plugin(&mut self, plugin: Box<dyn OverlayPlugin>) -> Result<(), String> {
        let name = plugin.name().to_string();
        let mut plugins = self.plugins.write();

        if plugins.contains_key(&name) {
            return Err(format!("Plugin '{}' is already registered", name));
        }

        plugins.insert(name, plugin);
        Ok(())
    }

    /// 移除插件
    pub fn unregister_plugin(&mut self, name: &str) -> Result<(), String> {
        let mut plugins = self.plugins.write();

        if let Some(mut plugin) = plugins.remove(name) {
            plugin.cleanup()?;
            Ok(())
        } else {
            Err(format!("Plugin '{}' not found", name))
        }
    }

    /// 获取插件
    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn OverlayPlugin>> {
        self.plugins.read().get(name)
    }

    /// 获取插件（可变）
    pub fn get_plugin_mut(&mut self, name: &str) -> Option<&mut Box<dyn OverlayPlugin>> {
        self.plugins.write().get_mut(name)
    }

    /// 初始化所有插件
    pub fn initialize_all(&mut self) -> Result<(), String> {
        let context = self.context.read().clone();
        let mut plugins = self.plugins.write();

        for (name, plugin) in plugins.iter_mut() {
            if let Err(e) = plugin.initialize(&context) {
                return Err(format!("Failed to initialize plugin '{}': {}", name, e));
            }
        }

        Ok(())
    }

    /// 清理所有插件
    pub fn cleanup_all(&mut self) -> Result<(), String> {
        let mut plugins = self.plugins.write();

        for (name, plugin) in plugins.iter_mut() {
            if let Err(e) = plugin.cleanup() {
                return Err(format!("Failed to cleanup plugin '{}': {}", name, e));
            }
        }

        Ok(())
    }

    /// 处理事件
    pub fn handle_event(
        &mut self,
        event: &str,
        data: HashMap<String, String>,
    ) -> Result<(), String> {
        let mut plugins = self.plugins.write();

        for (name, plugin) in plugins.iter_mut() {
            if let Err(e) = plugin.handle_event(event, &data) {
                return Err(format!(
                    "Plugin '{}' failed to handle event '{}': {}",
                    name, event, e
                ));
            }
        }

        Ok(())
    }

    /// 设置上下文
    pub fn set_context(&mut self, context: OverlayContext) {
        self.context.set(context);
    }

    /// 获取上下文
    pub fn get_context(&self) -> OverlayContext {
        self.context.read().clone()
    }

    /// 获取所有插件名称
    pub fn get_plugin_names(&self) -> Vec<String> {
        self.plugins.read().keys().cloned().collect()
    }

    /// 检查插件是否存在
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.read().contains_key(name)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

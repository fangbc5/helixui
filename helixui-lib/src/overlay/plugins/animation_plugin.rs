use crate::overlay::plugins::{OverlayContext, OverlayPlugin};
use std::collections::HashMap;

/// 动画插件
pub struct AnimationPlugin {
    name: String,
    version: String,
    data: HashMap<String, String>,
}

impl AnimationPlugin {
    pub fn new() -> Self {
        Self {
            name: "animation".to_string(),
            version: "1.0.0".to_string(),
            data: HashMap::new(),
        }
    }
}

impl OverlayPlugin for AnimationPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn initialize(&mut self, context: &OverlayContext) -> Result<(), String> {
        self.data
            .insert("overlay_id".to_string(), context.id.clone());
        self.data
            .insert("z_index".to_string(), context.z_index.to_string());
        Ok(())
    }

    fn cleanup(&mut self) -> Result<(), String> {
        self.data.clear();
        Ok(())
    }

    fn handle_event(&mut self, event: &str, data: &HashMap<String, String>) -> Result<(), String> {
        match event {
            "show" => {
                self.data.insert("visible".to_string(), "true".to_string());
                self.data
                    .insert("animation_state".to_string(), "entering".to_string());
            }
            "hide" => {
                self.data.insert("visible".to_string(), "false".to_string());
                self.data
                    .insert("animation_state".to_string(), "exiting".to_string());
            }
            "animation_complete" => {
                self.data
                    .insert("animation_state".to_string(), "idle".to_string());
            }
            _ => {}
        }
        Ok(())
    }

    fn get_data(&self) -> HashMap<String, String> {
        self.data.clone()
    }

    fn set_data(&mut self, data: HashMap<String, String>) -> Result<(), String> {
        self.data = data;
        Ok(())
    }
}

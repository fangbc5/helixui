use crate::overlay::plugins::{OverlayPlugin, OverlayContext};
use std::collections::HashMap;

/// 无障碍插件
pub struct AccessibilityPlugin {
    name: String,
    version: String,
    data: HashMap<String, String>,
}

impl AccessibilityPlugin {
    pub fn new() -> Self {
        Self {
            name: "accessibility".to_string(),
            version: "1.0.0".to_string(),
            data: HashMap::new(),
        }
    }
}

impl OverlayPlugin for AccessibilityPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn initialize(&mut self, context: &OverlayContext) -> Result<(), String> {
        self.data.insert("overlay_id".to_string(), context.id.clone());
        self.data.insert("aria_live".to_string(), "polite".to_string());
        self.data.insert("role".to_string(), "alert".to_string());
        self.data.insert("tab_index".to_string(), "0".to_string());
        Ok(())
    }
    
    fn cleanup(&mut self) -> Result<(), String> {
        self.data.clear();
        Ok(())
    }
    
    fn handle_event(&mut self, event: &str, data: &HashMap<String, String>) -> Result<(), String> {
        match event {
            "focus" => {
                self.data.insert("focused".to_string(), "true".to_string());
            },
            "blur" => {
                self.data.insert("focused".to_string(), "false".to_string());
            },
            "keydown" => {
                if let Some(key) = data.get("key") {
                    match key.as_str() {
                        "Escape" => {
                            self.data.insert("escape_pressed".to_string(), "true".to_string());
                        },
                        "Enter" => {
                            self.data.insert("enter_pressed".to_string(), "true".to_string());
                        },
                        _ => {}
                    }
                }
            },
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

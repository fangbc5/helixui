use crate::overlay::plugins::{OverlayPlugin, OverlayContext};
use std::collections::HashMap;

/// 主题插件
pub struct ThemePlugin {
    name: String,
    version: String,
    data: HashMap<String, String>,
}

impl ThemePlugin {
    pub fn new() -> Self {
        Self {
            name: "theme".to_string(),
            version: "1.0.0".to_string(),
            data: HashMap::new(),
        }
    }
}

impl OverlayPlugin for ThemePlugin {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn initialize(&mut self, context: &OverlayContext) -> Result<(), String> {
        self.data.insert("overlay_id".to_string(), context.id.clone());
        self.data.insert("theme_mode".to_string(), "auto".to_string());
        self.data.insert("primary_color".to_string(), "#3b82f6".to_string());
        self.data.insert("secondary_color".to_string(), "#6b7280".to_string());
        Ok(())
    }
    
    fn cleanup(&mut self) -> Result<(), String> {
        self.data.clear();
        Ok(())
    }
    
    fn handle_event(&mut self, event: &str, data: &HashMap<String, String>) -> Result<(), String> {
        match event {
            "theme_change" => {
                if let Some(theme_mode) = data.get("theme_mode") {
                    self.data.insert("theme_mode".to_string(), theme_mode.clone());
                }
            },
            "color_change" => {
                if let Some(primary_color) = data.get("primary_color") {
                    self.data.insert("primary_color".to_string(), primary_color.clone());
                }
                if let Some(secondary_color) = data.get("secondary_color") {
                    self.data.insert("secondary_color".to_string(), secondary_color.clone());
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

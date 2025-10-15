use super::tokens::ThemeTokens;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeConfig {
    pub tokens: ThemeTokens,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            tokens: ThemeTokens::default(),
        }
    }
}

impl ThemeConfig {
    pub fn spacing_px(&self, v: &str) -> u32 {
        let s = &self.tokens.spacing;
        match v {
            "xs" => s.xs,
            "sm" => s.sm,
            "md" => s.md,
            "lg" => s.lg,
            "xl" => s.xl,
            "xxl" => s.xxl,
            _ => 0,
        }
    }

    pub fn breakpoint_px(&self, name: &str) -> u32 {
        let b = &self.tokens.breakpoints;
        match name {
            "sm" => b.sm,
            "md" => b.md,
            "lg" => b.lg,
            "xl" => b.xl,
            "xxl" => b.xxl,
            _ => b.md,
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ThemeProviderProps {
    children: Element,
}

#[allow(non_snake_case)]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    rsx! { {props.children} }
}

pub fn use_theme() -> ThemeConfig {
    ThemeConfig::default()
}

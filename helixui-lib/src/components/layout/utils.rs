pub mod controlled {
    use dioxus::prelude::*;

    #[derive(Props, PartialEq, Clone)]
    pub struct Controlled<T: Clone + PartialEq + 'static> {
        #[props(optional)]
        pub value: Option<T>,
        #[props(optional)]
        pub default_value: Option<T>,
        #[props(optional)]
        pub on_change: Option<EventHandler<T>>,
    }

    impl<T: Clone + PartialEq + 'static> Controlled<T> {
        pub fn resolve<'a>(&self, inner: &'a T) -> T {
            self.value.clone().unwrap_or_else(|| inner.clone())
        }
        pub fn set_and_emit(&self, next: T, set_inner: impl FnOnce(T)) {
            if self.value.is_some() {
                if let Some(cb) = &self.on_change {
                    cb.call(next);
                }
            } else {
                set_inner(next.clone());
                if let Some(cb) = &self.on_change {
                    cb.call(next);
                }
            }
        }
    }
}

use super::tokens::{Breakpoint, SpacingToken};
use std::collections::HashMap;

/// 响应式尺寸配置
pub type ResponsiveSize = HashMap<Breakpoint, i32>;

/// 计算间距值
pub fn calc_gap(
    size: Option<i32>,
    spacing_token: Option<SpacingToken>,
    responsive_size: Option<&ResponsiveSize>,
    theme: &crate::components::layout::theme::ThemeConfig,
    current_breakpoint: &str,
) -> i32 {
    // 优先使用响应式尺寸
    if let Some(resp_size) = responsive_size {
        let bp = match current_breakpoint {
            "sm" => Breakpoint::Sm,
            "md" => Breakpoint::Md,
            "lg" => Breakpoint::Lg,
            "xl" => Breakpoint::Xl,
            "xxl" => Breakpoint::Xxl,
            _ => Breakpoint::Md,
        };
        return resp_size.get(&bp).copied().unwrap_or(8);
    }

    // 使用直接尺寸
    if let Some(s) = size {
        return s;
    }

    // 使用间距令牌
    let token = spacing_token.unwrap_or(SpacingToken::Sm);
    match token {
        SpacingToken::Xs => theme.spacing_px("xs") as i32,
        SpacingToken::Sm => theme.spacing_px("sm") as i32,
        SpacingToken::Md => theme.spacing_px("md") as i32,
        SpacingToken::Lg => theme.spacing_px("lg") as i32,
        SpacingToken::Xl => theme.spacing_px("xl") as i32,
        SpacingToken::Xxl => theme.spacing_px("xxl") as i32,
    }
}

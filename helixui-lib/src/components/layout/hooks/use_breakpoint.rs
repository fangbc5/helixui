
/// 当前断点状态
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BreakpointState {
    Xs,  // < 576px
    Sm,  // ≥ 576px
    Md,  // ≥ 768px
    Lg,  // ≥ 1024px
    Xl,  // ≥ 1280px
    Xxl, // ≥ 1536px
}

impl BreakpointState {
    pub fn as_str(&self) -> &'static str {
        match self {
            BreakpointState::Xs => "xs",
            BreakpointState::Sm => "sm",
            BreakpointState::Md => "md",
            BreakpointState::Lg => "lg",
            BreakpointState::Xl => "xl",
            BreakpointState::Xxl => "xxl",
        }
    }
}

/// 获取当前断点
pub fn use_breakpoint() -> String {
    // 这里简化实现，实际应该监听窗口大小变化
    // 暂时返回默认值
    "md".to_string()
}

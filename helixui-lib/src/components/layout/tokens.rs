#[derive(Clone, Debug, PartialEq)]
pub struct Breakpoints {
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
    pub xxl: u32,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            sm: 640,
            md: 768,
            lg: 1024,
            xl: 1280,
            xxl: 1536,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpacingScale {
    pub xs: u32,
    pub sm: u32,
    pub md: u32,
    pub lg: u32,
    pub xl: u32,
    pub xxl: u32,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            xs: 4,
            sm: 8,
            md: 12,
            lg: 16,
            xl: 24,
            xxl: 32,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Layering {
    pub header: i32,
    pub sider: i32,
    pub affix: i32,
    pub split_handle: i32,
}

impl Default for Layering {
    fn default() -> Self {
        Self {
            header: 800,
            sider: 900,
            affix: 1000,
            split_handle: 1100,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ThemeTokens {
    pub breakpoints: Breakpoints,
    pub spacing: SpacingScale,
    pub layers: Layering,
}

impl Default for ThemeTokens {
    fn default() -> Self {
        Self {
            breakpoints: Breakpoints::default(),
            spacing: SpacingScale::default(),
            layers: Layering::default(),
        }
    }
}

/// 断点枚举
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Breakpoint {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl Breakpoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Breakpoint::Sm => "sm",
            Breakpoint::Md => "md",
            Breakpoint::Lg => "lg",
            Breakpoint::Xl => "xl",
            Breakpoint::Xxl => "xxl",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpacingToken {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
}

impl SpacingToken {
    pub fn to_px(&self) -> i32 {
        match self {
            SpacingToken::Xs => 4,
            SpacingToken::Sm => 8,
            SpacingToken::Md => 16,
            SpacingToken::Lg => 24,
            SpacingToken::Xl => 32,
            SpacingToken::Xxl => 48,
        }
    }
}

impl Into<i32> for SpacingToken {
    fn into(self) -> i32 {
        self.to_px()
    }
}

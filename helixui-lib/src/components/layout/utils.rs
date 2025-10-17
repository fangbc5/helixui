use super::tokens::{Breakpoint, SpacingToken, ThemeTokens};
use std::cmp::Ordering;
use std::collections::BTreeMap;

/// 生成响应式类名
pub fn generate_responsive_classes<T>(
    base_class: &str,
    responsive_config: &Option<BTreeMap<Breakpoint, T>>,
    value_to_class: impl Fn(&T) -> String,
) -> String {
    let mut classes = vec![base_class.to_string()];
    if let Some(config) = responsive_config {
        for (bp, value) in config {
            classes.push(format!(
                "{}{}",
                bp.to_tailwind_prefix(),
                value_to_class(value)
            ));
        }
    }
    classes.join(" ")
}

/// 响应式尺寸类型：断点 -> 像素值
pub type ResponsiveSize = BTreeMap<Breakpoint, i32>;

/// 计算间距值（px）
/// 优先级：spacing_token > responsive_size(current) > base > theme.spacing.md
pub fn calc_gap(
    base: Option<i32>,
    spacing_token: Option<SpacingToken>,
    responsive_size: Option<&ResponsiveSize>,
    theme: &ThemeTokens,
    current: &Breakpoint,
) -> i32 {
    if let Some(token) = spacing_token {
        return token.to_px();
    }

    if let Some(map) = responsive_size {
        // 精确匹配
        if let Some(val) = map.get(current) {
            return *val;
        }
        // 向下寻找最接近的断点值
        let mut best: Option<(Breakpoint, i32)> = None;
        for (bp, val) in map.iter() {
            match bp.cmp(current) {
                Ordering::Greater => continue,
                _ => {
                    best = Some((*bp, *val));
                }
            }
        }
        if let Some((_, v)) = best {
            return v;
        }
    }

    base.unwrap_or(theme.spacing.md as i32)
}

/// Gap 类生成（标准 Tailwind 或自定义 px）
pub fn gap_to_tailwind_class(gap: i32) -> String {
    const STANDARD_GAPS: &[i32] = &[0, 1, 2, 3, 4, 6, 8, 12, 16, 20, 24, 32, 40, 48, 56, 64];
    if STANDARD_GAPS.contains(&gap) {
        format!("gap-{}", gap)
    } else {
        format!("gap-[{}px]", gap)
    }
}

/// Tailwind 栅格列数类
pub fn cols_to_tailwind_class(cols: u16) -> String {
    if (1..=24).contains(&cols) {
        format!("grid-cols-{}", cols)
    } else {
        format!("grid-cols-[{}]", cols)
    }
}

/// 生成 grid-column 样式
pub fn generate_grid_column_style(
    col: Option<u16>,
    span: Option<u16>,
    offset: Option<u16>,
) -> String {
    if let Some(col) = col {
        let start = col;
        let end = if let Some(span) = span {
            start + span - 1
        } else {
            start
        };
        format!("grid-column: {} / {};", start, end + 1)
    } else {
        let span = span.unwrap_or(1);
        let offset = offset.unwrap_or(0);
        format!("grid-column: {} / span {};", offset + 1, span)
    }
}

/// 生成 grid-row 样式
pub fn generate_grid_row_style(row: Option<u16>, row_span: Option<u16>) -> String {
    if let Some(row) = row {
        let start = row;
        let end = if let Some(span) = row_span {
            start + span - 1
        } else {
            start
        };
        format!("grid-row: {} / {};", start, end + 1)
    } else if let Some(span) = row_span {
        format!("grid-row: span {};", span)
    } else {
        String::new()
    }
}

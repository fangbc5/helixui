#[cfg(target_arch = "wasm32")]
use js_sys::Date as JsDate;
#[cfg(not(target_arch = "wasm32"))]
use time::{Date, Duration, OffsetDateTime};
#[cfg(target_arch = "wasm32")]
use time::{Date, Month};

/// 获取当前UTC日期和时间
#[cfg(not(target_arch = "wasm32"))]
pub fn now_utc() -> Date {
    OffsetDateTime::now_utc().date()
}
#[cfg(target_arch = "wasm32")]
pub fn now_utc() -> Date {
    // JS时间戳（毫秒）
    let js_date = JsDate::new_0();
    let year = js_date.get_utc_full_year() as i32;
    let month_index = js_date.get_utc_month() as u8 + 1; // JS: 0~11, Rust: 1~12
    let day = js_date.get_utc_date() as u8;
    Date::from_calendar_date(year, Month::try_from(month_index).unwrap(), day).unwrap()
}

/// 获取今天的UTC日期（等价于now_utc）
pub fn today_utc() -> Date {
    now_utc()
}

/// 日期加天数
#[cfg(not(target_arch = "wasm32"))]
pub fn date_add_days(date: Date, days: i64) -> Date {
    date.saturating_add(Duration::days(days))
}
#[cfg(target_arch = "wasm32")]
pub fn date_add_days(date: Date, days: i64) -> Date {
    // 转成js_sys::Date，加天后再转回
    let js = JsDate::new_with_year_month_day(
        date.year() as u32, // 修正为u32
        (date.month() as u32 - 1) as i32,
        date.day() as i32,
    );
    let day_value = js.get_utc_date() as i64 + days;
    js.set_utc_date(day_value as u32); // 修正为u32
    let year = js.get_utc_full_year() as i32;
    let month_index = js.get_utc_month() as u8 + 1;
    let day = js.get_utc_date() as u8;
    Date::from_calendar_date(year, Month::try_from(month_index).unwrap(), day).unwrap()
}

/// 日期减天数
pub fn date_sub_days(date: Date, days: i64) -> Date {
    date_add_days(date, -days)
}

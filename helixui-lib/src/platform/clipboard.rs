#[cfg(target_family = "wasm")]
pub async fn copy_to_clipboard(text: String) {
    use wasm_bindgen_futures::JsFuture;

    if let Some(window) = web_sys::window() {
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let _ = JsFuture::from(clipboard.write_text(&text)).await;
    }
}

#[cfg(not(target_family = "wasm"))]
pub async fn copy_to_clipboard(text: String) {
    if let Ok(mut clipboard) = arboard::Clipboard::new() {
        let _ = clipboard.set_text(text);
    }
}

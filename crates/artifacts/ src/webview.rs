pub struct WebView;

impl WebView {
    pub fn render(code: &str) {
        println!("WEBVIEW RENDER START");

        // ⚠️ يجب لاحقاً sandbox execution (iframe / container)
        println!("{}", code);

        println!("WEBVIEW RENDER END");
    }
}

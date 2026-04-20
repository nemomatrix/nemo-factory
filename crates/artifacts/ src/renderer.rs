use crate::webview::WebView;
use crate::graph::GraphRenderer;
use crate::report::ReportBuilder;

pub struct Renderer;

impl Renderer {
    pub fn render(code: &str) {
        if code.contains("React") || code.contains("<html") {
            WebView::render(code);
        } else if code.contains("plot") {
            GraphRenderer::render(code);
        } else {
            ReportBuilder::render(code);
        }
    }
}

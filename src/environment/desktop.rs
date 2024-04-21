use dioxus::{
    desktop::{Config, WindowBuilder},
    dioxus_core::Element,
    prelude::LaunchBuilder,
};

#[cfg(target_os = "macos")]
use dioxus::desktop::LogicalSize;

pub fn init_app(root: fn() -> Element) {
    // Configure dioxus-desktop Tauri window
    let config_builder = Config::default().with_custom_index(
        r#"
            <!DOCTYPE html>
            <html data-bs-theme="light">
                <head>
                    <link rel="stylesheet" href="style/style.css">
                    <meta name="viewport" content="width=device-width, initial-scale=1">
                </head>
                <body>
                    <div id="main"></div>
                    <script type="text/javascript" src="js/darkmode.js"></script>
                </body>
            </html>
        "#
        .to_string(),
    );

    let window_builder = WindowBuilder::new().with_default();

    // Launch the app
    LaunchBuilder::desktop()
        .with_cfg(config_builder.with_window(window_builder))
        .launch(root)
}

trait WindowBuilderExt {
    fn with_default(self) -> Self;
}

impl WindowBuilderExt for WindowBuilder {
    /// Set default window settings
    fn with_default(self) -> Self {
        self.with_title("Dev Widgets")
            .with_resizable(true)
            .with_inner_size(LogicalSize::new(800.0, 800.0))
            .with_min_inner_size(LogicalSize::new(600.0, 300.0))
    }
}

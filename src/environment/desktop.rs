use dioxus::prelude::Component;
use dioxus_desktop::{
    tao::menu::{MenuBar, MenuItem},
    Config as DesktopConfig, WindowBuilder,
};

pub fn init_app(root: Component) {
    // Configure dioxus-desktop Tauri window
    let config_builder = DesktopConfig::default()
        .with_custom_index(
            r#"
                <!DOCTYPE html>
                <html data-bs-theme="light">
                    <head>
                        <title>Dev Widgets</title>
                        <link rel="stylesheet" href="../style/style.css">
                        <meta name="viewport" content="width=device-width, initial-scale=1">
                    </head>
                    <body>
                        <div id="main"></div>
                        <script type="text/javascript">
                            // Set theme to the user's preferred color scheme
                            function updateTheme() {
                            const colorMode = window.matchMedia("(prefers-color-scheme: dark)").matches ?
                                "dark" :
                                "light";
                            document.querySelector("html").setAttribute("data-bs-theme", colorMode);
                            }

                            // Set theme on load
                            updateTheme()

                            // Update theme when the preferred scheme changes
                            window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', updateTheme)
                        </script>
                        <script type="text/javascript" src="../js/bootstrap.min.js"></script>
                    </body>
                </html>
            "#.to_string()
        );

    #[cfg(target_os = "macos")]
    let window_builder = WindowBuilder::new().with_default().with_file_menu();
    #[cfg(not(target_os = "macos"))]
    let window_builder = WindowBuilder::new().with_default();

    // Launch the app
    dioxus_desktop::launch_cfg(
        root,
        config_builder.with_window(window_builder),
    );
}

trait WindowBuilderExt {
    fn with_default(self) -> Self;
    #[cfg(target_os = "macos")]
    fn with_file_menu(self) -> Self;
}

impl WindowBuilderExt for WindowBuilder {
    /// Set default window settings
    fn with_default(self) -> Self {
        self.with_title("Dev Widgets")
            .with_resizable(true)
            .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(
                800.0, 800.0,
            ))
            .with_min_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(
                600.0, 300.0,
            ))
    }

    #[cfg(target_os = "macos")]
    /// Workaround on macOS to get system keyboard shortcuts for copy, paste, etc.
    fn with_file_menu(self) -> Self {
        let mut menu = MenuBar::new();
        let mut app_menu = MenuBar::new();
        app_menu.add_native_item(MenuItem::Quit);
        menu.add_submenu("Dev Widgets", true, app_menu);
        let mut edit_menu = MenuBar::new();
        edit_menu.add_native_item(MenuItem::Undo);
        edit_menu.add_native_item(MenuItem::Redo);
        edit_menu.add_native_item(MenuItem::Separator);
        edit_menu.add_native_item(MenuItem::Cut);
        edit_menu.add_native_item(MenuItem::Copy);
        edit_menu.add_native_item(MenuItem::Paste);
        edit_menu.add_native_item(MenuItem::Separator);
        edit_menu.add_native_item(MenuItem::SelectAll);
        menu.add_submenu("Edit", true, edit_menu);
        self.with_menu(menu)
    }
}
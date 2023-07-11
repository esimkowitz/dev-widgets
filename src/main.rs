#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_desktop::{
    tao::menu::{MenuBar, MenuItem},
    Config as DesktopConfig, WindowBuilder,
};
use dioxus_router::{use_route, Link, Redirect, Route, Router};

#[cfg(debug_assertions)]
use dioxus_hot_reload::{hot_reload_init, Config as HotReloadConfig};
use std::env;

use pages::{home_page::HOME_PAGE_WIDGET_ENTRY, WIDGETS};
use widget_entry::WidgetEntry;

pub mod components;
pub mod pages;
pub mod widget_entry;

fn main() {
    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "1");

        hot_reload_init!(HotReloadConfig::new()
            .with_paths(&["src", "style", "scss"])
            .with_rebuild_command("cargo run"));
    }

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

    // Launch the app
    dioxus_desktop::launch_cfg(
        App,
        config_builder.with_window(if cfg!(target_os = "macos") {
            WindowBuilder::new().with_default().with_file_menu()
        } else {
            WindowBuilder::new().with_default()
        }),
    );
}

trait WindowBuilderExt {
    fn with_default(self) -> Self;
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

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container-fluid",
            Router {
                Sidebar {}
                div {
                    class: "widget-view",
                    Route {
                        to: HOME_PAGE_WIDGET_ENTRY.path,
                        WidgetView {
                            title: HOME_PAGE_WIDGET_ENTRY.title,
                            children: (HOME_PAGE_WIDGET_ENTRY.function)(cx)
                        }
                    }
                    for widget_type in WIDGETS.keys() {
                        for widget_entry in WIDGETS.get(widget_type).unwrap() {
                            Route {
                                to: widget_entry.path,
                                WidgetView {
                                    title: widget_entry.title,
                                    children: (widget_entry.function)(cx)
                                }
                            }
                        }
                    }
                }
                Redirect { from: "", to: HOME_PAGE_WIDGET_ENTRY.path }
            }
        }
    })
}

fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "sidebar",
            div {
                class: "sidebar-list",
                div {
                    class: "accordion",
                    SidebarListItem {
                        widget_entry: HOME_PAGE_WIDGET_ENTRY,
                        icon: (HOME_PAGE_WIDGET_ENTRY.icon)(cx)
                    }
                    for widget_type in WIDGETS.keys() {
                        div {
                            components::accordion::Accordion {
                                title: *widget_type,
                                is_open: true,
                                for widget_entry in WIDGETS.get(widget_type).unwrap() {
                                    SidebarListItem {
                                        widget_entry: *widget_entry,
                                        icon: (widget_entry.icon)(cx)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "vr"
            }
        }
    })
}

#[inline_props]
fn WidgetView<'a>(cx: Scope<'a>, children: Element<'a>, title: &'a str) -> Element {
    cx.render(rsx! {
        h3 {
            class: "widget-title",
            *title
        }
        div {
            class: "widget-body",
            children
        }
        div {
            class: "widget-footer"
        }
    })
}

#[inline_props]
fn SidebarListItem<'a>(cx: Scope<'a>, widget_entry: WidgetEntry, icon: Element<'a>) -> Element {
    let route = use_route(cx);

    let active_str = if route.url().path() == widget_entry.path {
        "active"
    } else {
        ""
    };

    cx.render(rsx! {
        Link {
            class: "btn {active_str}",
            to: widget_entry.path,
            icon
            widget_entry.short_title
        }
    })
}

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

pub fn init_app(root: Component) {
    dioxus_web::launch(root);
}

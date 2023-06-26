mod routes {
    pub mod home;
    pub mod dev;
    pub mod post_detail;
}
mod common {
    pub mod components {
        pub mod nav;
        pub mod select;
        pub mod post_preview;
    }
    pub mod state {
        pub mod user_type;
    }
}
mod app;

use leptos::*;
use crate::app::App;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

mod routes {
    pub mod home;
    pub mod dev {
        pub mod dev;
        pub mod left_menu;
        pub mod promoted_jobs;
    }
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
mod common_test {
    pub mod fixtures;
}
mod app;

use leptos::*;
use crate::app::App;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}

use leptos::*;
use leptos_router::*;
use crate::{common::state::user_type::UserType, routes::{home::Home, dev::Dev}};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (_user_type, set_user_type) = create_signal(cx, UserType::Developer);
    provide_context(cx, set_user_type);

    view! {
        cx,
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <Home /> } />
                    <Route path="/dev" view=|cx| view! { cx, <Dev /> } />
                    <Route path="/employer" view=|cx| view! { cx, <div /> } />
                </Routes>
            </main>
        </Router>
    }
}
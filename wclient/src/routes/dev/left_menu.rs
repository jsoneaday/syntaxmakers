use leptos::{component, Scope, view, IntoView};

#[component]
pub fn LeftMenu(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <nav class="panel-col dev-menu-container">
            <span class="title-font left-menu-header">"@jonny"</span>
            <a class="sub-title-font dev-menu-item">
                <img class="dev-menu-icon" src="safe-box.png" />
                <span>"Saved jobs"</span>
            </a>
            <a class="sub-title-font dev-menu-item">
                <img class="dev-menu-icon" src="notification.png" />
                <span>"Job alerts"</span>
            </a>             
        </nav>
    }
}
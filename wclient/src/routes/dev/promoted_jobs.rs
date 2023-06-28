use leptos::{component, Scope, view, IntoView};

#[component]
pub fn PromotedJobs(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <nav class="panel-col job-menu-container">
            <a class="sub-title-font job-menu-item">
                <img class="job-menu-icon" src="safe-box.png" />
                <span>"Job A"</span>
            </a>
            <a class="sub-title-font job-menu-item">
                <img class="job-menu-icon" src="notification.png" />
                <span>"Job B"</span>
            </a>        
            <a class="sub-title-font job-menu-item">
                <img class="job-menu-icon" src="notification.png" />
                <span>"Job C"</span>
            </a>         
        </nav>
    }
}
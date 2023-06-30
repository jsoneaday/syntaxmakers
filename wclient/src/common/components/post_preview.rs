use leptos::{view, Scope, IntoView, component, Show};

#[derive(Clone)]
pub struct PostPreviewProp {
    pub id: i64,
    pub title: String,
    pub company: String,
    pub location: String,
    pub salary: String,
    pub timestamp: String,
    pub icon_src: String
}

#[component]
pub fn PostPreview(cx: Scope, post_preview: PostPreviewProp, is_small: bool) -> impl IntoView {
    view! {
        cx,
        <div class="post-preview-container">
            <Show 
                when=move || !is_small
                fallback=move |cx| view! {cx, <span />}
            >
                <img class="preview-icon" src=post_preview.icon_src.to_string() />
            </Show>
            <div class="preview-content">
                <div class="title-font">{post_preview.title}</div>
                <div class="sub-title-font">{post_preview.company}</div>
                <div class="normal-font">{post_preview.location}</div>
                <div class="normal-font preview-salary">"Base Salary: "<i>{post_preview.salary}</i></div>
                <Show 
                    when=move || !is_small
                    fallback=move |cx| view! {cx, <span />}
                >
                    <div class="small-font preview-timestamp">{post_preview.timestamp.to_string()}</div>
                </Show>
            </div>
        </div>
    }
}
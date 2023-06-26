use leptos::{view, Scope, IntoView, component};

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
pub fn PostPreview(cx: Scope, post_preview: PostPreviewProp) -> impl IntoView {
    view! {
        cx,
        <div class="post-preview-container">
            <img class="preview-icon" src=post_preview.icon_src />
            <div class="preview-content">
                <div class="title-font">{post_preview.title}</div>
                <div class="sub-title-font">{post_preview.company}</div>
                <div class="normal-font">{post_preview.location}</div>
                <div class="normal-font preview-salary">"Base Salary: "<i>{post_preview.salary}</i></div>
                <div class="small-font preview-timestamp">{post_preview.timestamp}</div>
            </div>
        </div>
    }
}
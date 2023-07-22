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
    let small_title_font = move || if is_small {
        "title-font preview-small-title-font"
    } else {
        "title-font"
    };
    let small_sub_title_font = move || if is_small {
        "sub-title-font preview-small-sub-title-font"
    } else {
        "sub-title-font"
    };
    let small_normal_font = move || if is_small {
        "normal-font preview-small-normal-font"
    } else {
        "normal-font"
    };

    let icon = if is_small {
        None
    } else {
        Some(view! {cx, 
            <img class="preview-icon" src=post_preview.icon_src.to_string() />
        })
    };

    let timestamp = if is_small {
        None
    } else {
        Some(view! { cx,
            <div class="small-font preview-timestamp">{post_preview.timestamp.to_string()}</div>
        })
    };

    view! {
        cx,
        <div class="post-preview-container">
            {icon}
            <div class="preview-content">
                <div class=small_title_font>{post_preview.title}</div>
                <div class=small_sub_title_font>{post_preview.company}</div>
                <div class=small_normal_font>{post_preview.location}</div>
                <div class="normal-font preview-salary">"Base Salary: "<i>{post_preview.salary}</i></div>
                {timestamp}
            </div>
        </div>
    }
}
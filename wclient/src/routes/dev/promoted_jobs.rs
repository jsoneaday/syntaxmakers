use chrono::Utc;
use fake::{faker::{company::en::CompanyName, lorem::en::Sentence}, Fake};
use leptos::{component, Scope, view, IntoView, For, create_signal};
use uuid::Uuid;
use crate::{common::components::post_preview::{PostPreview, PostPreviewProp}, common_test::fixtures::salaries};
use fake::faker::address::en::{StateName, CountryName};

#[component]
pub fn PromotedJobs(cx: Scope) -> impl IntoView {
    let (post_previews, _set_post_previews) = create_signal::<Vec<PostPreviewProp>>(cx, vec![
        PostPreviewProp {
            id: 1,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", StateName().fake::<String>(), CountryName().fake::<String>()),
            salary: salaries().get(1).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-building-cl.png".to_string(),
        },
        PostPreviewProp {
            id: 2,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", StateName().fake::<String>(), CountryName().fake::<String>()),
            salary: salaries().get(0).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-1.png".to_string(),
        },
        PostPreviewProp {
            id: 3,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", StateName().fake::<String>(), CountryName().fake::<String>()),
            salary: salaries().get(1).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-2.png".to_string(),
        }
    ]);
    
    view! {
        cx,
        <div class="panel-col job-menu-container">
            <div class="title-font">"Promoted jobs"</div>
            <ul>
                <For 
                    each=post_previews 
                    key=|_| { Uuid::new_v4() }
                    view=move |cx, post_preview: PostPreviewProp| {
                        view!{
                            cx,
                            <li class="dev-preview-item">
                                <PostPreview post_preview=post_preview is_small=true />
                            </li>
                        }
                    }
                />
            </ul>      
        </div>
    }
}
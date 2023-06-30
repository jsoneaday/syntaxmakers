use chrono::Utc;
use fake::faker::{lorem::en::Sentence, company::en::CompanyName};
use fake::Fake;
use lazy_static::lazy_static;
use leptos::{Scope, IntoView, view, component, create_signal, For};
use crate::common::components::{select::{Select, SelectOption, SelectIcon}, post_preview::{PostPreviewProp, PostPreview}};
use crate::common_test::fixtures::salaries;
use crate::routes::dev::left_menu::LeftMenu;
use crate::routes::dev::promoted_jobs::PromotedJobs;
use uuid::Uuid;



#[component]
pub fn Dev(cx: Scope) -> impl IntoView {
    let (post_previews, _set_post_previews) = create_signal::<Vec<PostPreviewProp>>(cx, vec![
        PostPreviewProp {
            id: 1,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: salaries().get(1).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-building-cl.png".to_string(),
        },
        PostPreviewProp {
            id: 2,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: salaries().get(0).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-1.png".to_string(),
        },
        PostPreviewProp {
            id: 3,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: salaries().get(1).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-2.png".to_string(),
        },
        PostPreviewProp {
            id: 4,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: salaries().get(2).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-3.png".to_string(),
        },
        PostPreviewProp {
            id: 5,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: salaries().get(2).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-4.png".to_string(),
        },
        PostPreviewProp {
            id: 6,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: salaries().get(1).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-5.png".to_string(),
        },
        PostPreviewProp {
            id: 7,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: salaries().get(2).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-6.png".to_string(),
        }
    ]);

    let (search_input, _set_search_input) = create_signal(cx, "");

    view! {
        cx,
        <div class="dev-container">
            <LeftMenu />
            <div class="dev-main">
                <div class="dev-top header-container">
                    <div class="title-font dev-header">
                        "Developer job search"
                    </div>
                    <div class="sub-title-font dev-sub-header">
                        "Enter your preferences to find your next job"
                    </div>
                    <div class="search-header">
                        <input class="search-input" type="test" value=search_input />
                        <button class="primary-btn">"search"</button>
                    </div>
                </div>
                <div class="info-band">
                    <img class="dev-info-band-icon" src="clipboard.png" />
                    "Result count 231"  
                    <img class="dev-info-band-icon" style="margin-left: 1.5em;" src="wall-clock.png" />
                    "Date jun 16, 2023"
                </div>
                <div class="dev-post-preview-container">
                    <ul>
                        <For 
                            each=post_previews 
                            key=|_| { Uuid::new_v4() }
                            view=move |cx, post_preview: PostPreviewProp| {
                                view!{
                                    cx,
                                    <li class="dev-preview-item">
                                        <PostPreview post_preview=post_preview is_small=false />
                                    </li>
                                }
                            }
                        />
                    </ul>
                </div>
            </div>        
            <PromotedJobs />
        </div>
    }
}
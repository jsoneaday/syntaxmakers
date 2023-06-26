use chrono::Utc;
use fake::faker::{lorem::en::Sentence, company::en::CompanyName};
use fake::Fake;
use lazy_static::lazy_static;
use leptos::{Scope, IntoView, view, component, create_signal, For};
use crate::common::components::{select::{Select, SelectOption, SelectIcon}, post_preview::{PostPreviewProp, PostPreview}};
use uuid::Uuid;

lazy_static!{
    static ref PRIM_LANGUAGES: Vec<SelectOption> = vec![
        SelectOption { id: 1, label: "Rust".to_string()},
        SelectOption { id: 7, label: "Go".to_string()},
        SelectOption { id: 10, label: "Ruby".to_string()},
        SelectOption { id: 11, label: "Swift".to_string()},
        SelectOption { id: 12, label: "Kotlin".to_string()},
        SelectOption { id: 13, label: "Scala".to_string()},
        SelectOption { id: 13, label: "Elixir".to_string()}
    ];
    static ref SEC_LANGUAGES: Vec<SelectOption> = {
        let mut sec_languages = PRIM_LANGUAGES.clone();
        sec_languages.insert(0, SelectOption { id: 0, label: "Optional".to_string() });
        sec_languages
    };
    static ref INDUSTRIES: Vec<SelectOption> = vec![
        SelectOption { id: 1, label: "Finance".to_string()},
        SelectOption { id: 2, label: "Crypto".to_string()},
        SelectOption { id: 3, label: "AI/ML".to_string()},
        SelectOption { id: 5, label: "Video Games".to_string()},
    ];
    static ref SALARIES: Vec<SelectOption> = vec![
        SelectOption { id: 1, label: "$200,000+".to_string()},
        SelectOption { id: 2, label: "$300,000+".to_string()},
        SelectOption { id: 3, label: "$400,000+".to_string()},
    ];
    static ref LOCATIONS: Vec<SelectOption> = vec![
        SelectOption { id: 1, label: "Remote".to_string()},
        SelectOption { id: 2, label: "New York, United States".to_string()},
        SelectOption { id: 3, label: "San Francisco, United States".to_string()},
        SelectOption { id: 3, label: "Other".to_string()},
    ];
}

#[component]
pub fn Dev(cx: Scope) -> impl IntoView {
    let (post_previews, _set_post_previews) = create_signal::<Vec<PostPreviewProp>>(cx, vec![
        PostPreviewProp {
            id: 1,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: SALARIES.get(1).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-building-cl.png".to_string(),
        },
        PostPreviewProp {
            id: 2,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: SALARIES.get(0).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-1.png".to_string(),
        },
        PostPreviewProp {
            id: 3,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: SALARIES.get(1).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-2.png".to_string(),
        },
        PostPreviewProp {
            id: 4,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: SALARIES.get(2).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-3.png".to_string(),
        },
        PostPreviewProp {
            id: 5,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: SALARIES.get(2).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-4.png".to_string(),
        },
        PostPreviewProp {
            id: 6,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: SALARIES.get(1).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-5.png".to_string(),
        },
        PostPreviewProp {
            id: 7,
            title: Sentence(5..6).fake(),
            company: CompanyName().fake(),
            location: format!("{}, {}", fake::faker::address::en::StateName().fake::<String>(), fake::faker::address::en::CountryName().fake::<String>()),
            salary: SALARIES.get(2).unwrap().label.clone(),
            timestamp: Utc::now().timestamp().to_string(),
            icon_src: "office-cl-6.png".to_string(),
        }
    ]);

    view! {
        cx,
        <div class="dev-container">
            <div class="dev-top">
                <div class="title-font header-container dev-header">
                    "Select your preferences to find your next job"
                </div>
                <div class="selector-items">
                    <div class="section-container">
                        <Select 
                            id="primary_language_selections".to_string() 
                            label="Primary Language".to_string() 
                            options=PRIM_LANGUAGES.clone()
                            select_icon={SelectIcon{ src: "web-programming.png".to_string(), style: "margin-right: 0.5em;".to_string() }}
                        />
                    </div>
                    <div class="section-container">
                        <Select 
                            id="secondary_language_selections".to_string() 
                            label="Secondary Language".to_string() 
                            options=SEC_LANGUAGES.clone()
                            select_icon={SelectIcon{ src: "web-programming.png".to_string(), style: "margin-right: 0.5em;".to_string() }}
                        />
                    </div>
                    <div class="section-container">
                        <Select 
                            id="industry_selections".to_string() 
                            label="Industry".to_string() 
                            options=INDUSTRIES.clone()
                            select_icon={SelectIcon{ src: "office-building.png".to_string(), style: "margin-right: 0.5em;".to_string() }}
                        />
                    </div>
                    <div class="section-container">
                        <Select 
                            id="salary_selections".to_string() 
                            label="Base Salary".to_string() 
                            options=SALARIES.clone()
                            select_icon={SelectIcon{ src: "save-money.png".to_string(), style: "margin-right: 0.5em;".to_string() }}
                        />
                    </div>
                    <div class="section-container">
                        <Select 
                            id="location_selections".to_string() 
                            label="Location".to_string() 
                            options=LOCATIONS.clone()
                            select_icon={SelectIcon{ src: "location.png".to_string(), style: "margin-right: 0.5em;".to_string() }}
                        />
                    </div>
                    <div class="filter-submit-section">
                        <button class="primary-btn">"Submit"</button>
                    </div>
                </div>            
            </div>
            <div class="info-band">
                <img class="dev-info-band-icon" src="clipboard.png" />
                "Result count 231"  
                <img class="dev-info-band-icon" style="margin-left: 1.5em;" src="wall-clock.png" />
                "Date jun 16, 2023"
            </div>
            <div class="dev-post-preview">
                <ul>
                    <For 
                        each=post_previews 
                        key=|_| { Uuid::new_v4() }
                        view=move |cx, post_preview: PostPreviewProp| {
                            view!{
                                cx,
                                <li>
                                    <PostPreview post_preview=post_preview />
                                </li>
                            }
                        }
                    />
                </ul>
            </div>
        </div>        
    }
}
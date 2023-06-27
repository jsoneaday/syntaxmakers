use leptos::{view, Scope, IntoView, component, create_signal};

#[derive(Clone)]
pub struct SelectOption {
    pub id: usize,
    pub label: String
}

pub struct SelectIcon {
    pub src: String,
    pub style: String
}

#[component]
pub fn Select(cx: Scope, id: String, label: String, options: Vec<SelectOption>, select_icon: SelectIcon) -> impl IntoView {
    let (is_open, _set_is_open) = create_signal(cx, false);
    let selection_indicator = move || {
        if is_open() {
            "./up-arrow.png"
        } else {
            "./down-arrow.png"
        }
    };

    view! {
        cx,
        <div class="select-container">
            <span class="select-icon-label">
                <img class="select-icon" src={select_icon.src} style={select_icon.style} />
                <label class="sub-title-font select-label" for={id.clone()}>{label}</label>
            </span>
            <div class="normal-font select">
                <select id={id.clone()}>
                    {
                        options.into_iter()
                            .map(|option| {
                                view! {
                                    cx,
                                    <option value={option.id}>{option.label}</option>
                                }
                            })
                            .collect::<Vec<_>>()
                    }
                    <span class="focus"></span>
                </select>
                <img class="select-chevron" src=selection_indicator />
            </div>            
        </div>
    }
}
use leptos::{IntoView, Scope, view, use_context, WriteSignal, component};
use leptos_router::A;
use crate::common::state::user_type::UserType;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let _set_user_type = use_context::<WriteSignal<UserType>>(cx).unwrap();
    println!("hello!!!");
    view! {
        cx,
        <div class="home">                      
            <div class="panel-col home-item-dev">
                <A href="dev" class="home-item-link">
                    <img class="home-icon" src="./programmer.png" />
                    <div class="title-font home-item-content">
                    "I'm a Developer"
                    </div>                    
                </A>
            </div>            
                        
            <div class="panel-col home-item-emp">      
                <A href="employer" class="home-item-link">          
                    <img class="home-icon" src="./businessman.png" />
                    <div class="title-font home-item-content">
                    "I'm an Employer"
                    </div>                
                </A>
            </div>            
        </div>
    }
}
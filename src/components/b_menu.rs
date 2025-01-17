use leptos::*;
use leptos::attr::{Attr, AttributeKey};
use leptos::prelude::*;
use leptos::html::*;
use leptos::text_prop::TextProp;

#[component]
pub fn BAsideMenu(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <aside class=format!("menu {}", class.get())>{children()}</aside> }
}

#[component]
pub fn BMenu(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <div class=format!("menu {}", class.get())>{children()}</div> }
}

#[component]
pub fn BMenuLabel(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <div class=format!("menu-label {}", class.get())>{children()}</div> }
}

#[component]
pub fn BMenuList(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <ul class=format!("menu-list {}", class.get())>{children()}</ul> }
}

#[component]
pub fn BMenuLink(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] href: TextProp,
) -> impl IntoView {
    //{Attr{0:format!{"{}","exact"},1:format!{"{}",true}}}
    //active_class="is-active"
    view! {
        <li>
            <a class={class.get()} href=href.get()  >
                {children()}
            </a>
        </li>
    }
}

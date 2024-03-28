use leptos::*;

#[component]
pub fn BAsideMenu(children: Children, #[prop(default = "")] class: &'static str) -> impl IntoView {
    view! { <aside class=format!("menu {}", class)>{children()}</aside> }
}

#[component]
pub fn BMenu(children: Children, #[prop(default = "")] class: &'static str) -> impl IntoView {
    view! { <div class=format!("menu {}", class)>{children()}</div> }
}

#[component]
pub fn BMenuLabel(children: Children, #[prop(default = "")] class: &'static str) -> impl IntoView {
    view! { <div class=format!("menu-label {}", class)>{children()}</div> }
}

#[component]
pub fn BMenuList(children: Children, #[prop(default = "")] class: &'static str) -> impl IntoView {
    view! { <ul class=format!("menu-list {}", class)>{children()}</ul> }
}

#[component]
pub fn BMenuLink(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] href: TextProp,
) -> impl IntoView {
    view! { <li><a active_class="is-active" class=class exact=true href=href>{children()}</a></li> }
}

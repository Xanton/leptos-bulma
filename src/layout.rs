use leptos::*;
use leptos::html::*;
use leptos::prelude::{Children, ClassAttribute};
use leptos::text_prop::TextProp;

#[component]
pub fn BSection(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <section class=format!("section {}", class.get())>{children()}</section> }
}

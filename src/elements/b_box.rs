use leptos::*;
use leptos::prelude::{Children, ClassAttribute};
use leptos::text_prop::TextProp;
use leptos::html::*;

#[component]
pub fn BBox(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <div class=format!("box {}", class.get())>{children()}</div> }
}

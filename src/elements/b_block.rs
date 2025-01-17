use leptos::*;
use leptos::html::*;
use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[component]
pub fn BBlock(children: Children, #[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! {
        <div class=format!("block {}", class.get())>
            {children()}
        </div>
    }
}

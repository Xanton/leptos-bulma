use leptos::*;
use leptos_bulma::elements::BBox;
use leptos::prelude::*;

#[component]
pub fn BasicBox() -> impl IntoView {
    view! { <BBox class="has-text-centered">"Hello, World!"</BBox> }
}

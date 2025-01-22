use leptos::*;
use leptos_bulma::elements::BProgress;
//use leptos::prelude::*;

#[component]
pub fn ProgressIndeterminate() -> impl IntoView {
    view! { <BProgress max=100 /> }
}

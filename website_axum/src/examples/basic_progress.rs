use leptos::*;
use leptos_bulma::elements::BProgress;
//use leptos::prelude::*;

#[component]
pub fn BasicProgress() -> impl IntoView {
    view! {
        <BProgress max=100 value=25>
            "25%"
        </BProgress>
    }
}

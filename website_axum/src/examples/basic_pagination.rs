use leptos::*;
use leptos_bulma::components::BPagination;
use leptos::prelude::*;

#[component]
pub fn BasicPagination() -> impl IntoView {
    view! { <BPagination class="is-centered" count=10 list_size=4/> }
}

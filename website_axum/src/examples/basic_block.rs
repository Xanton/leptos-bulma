use leptos::*;
use leptos_bulma::elements::BBlock;
//use leptos::prelude::*;


#[component]
pub fn BasicBlock() -> impl IntoView {
    view! { <BBlock class="has-text-centered">"Hello, World!"</BBlock> }
}

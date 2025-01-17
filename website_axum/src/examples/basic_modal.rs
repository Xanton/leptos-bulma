use leptos::*;
use leptos_bulma::components::{BModal, BModalContent};
use leptos_bulma::elements::BBox;
use leptos::prelude::*;

#[component]
pub fn BasicModal() -> impl IntoView {
    let modal_is_active = RwSignal::new(false);

    view! {
        <a class="button block" on:click=move |_| modal_is_active.set(true)>
            "Open modal"
        </a>

        <BModal is_active=modal_is_active>
            <BModalContent>
                <BBox class="has-text-centered">"Hello, World!"</BBox>
            </BModalContent>
        </BModal>
    }
}

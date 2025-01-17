use leptos::*;

use crate::components::PageTitle;
use leptos::prelude::*;

/// 404 - Not Found
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <PageTitle text="Error 404: Page Not Found"/>

        <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>
    }
}

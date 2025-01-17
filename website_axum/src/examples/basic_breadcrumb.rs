use leptos::*;
use leptos_bulma::components::{BBreadcrumb, BBreadcrumbItem};
use leptos::prelude::*;

#[component]
pub fn BasicBreadcrumb() -> impl IntoView {
    view! {
        <BBreadcrumb>
            <BBreadcrumbItem href_P="#">"Leptos Bulma"</BBreadcrumbItem>
            <BBreadcrumbItem href_P="#">"Components"</BBreadcrumbItem>
            <BBreadcrumbItem href_P="#" is_active=true>
                "Breadcrumb"
            </BBreadcrumbItem>
        </BBreadcrumb>
    }
}

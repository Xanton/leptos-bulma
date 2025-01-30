use leptos::*;
use leptos_bulma::components::{BBreadcrumb, BBreadcrumbItem};
//use leptos::prelude::*;

#[component]
pub fn BasicBreadcrumb() -> impl IntoView {
    view! {
        <BBreadcrumb>
            <BBreadcrumbItem href_p="#">"Leptos Bulma"</BBreadcrumbItem>
            <BBreadcrumbItem href_p="#">"Components"</BBreadcrumbItem>
            <BBreadcrumbItem href_p="#" is_active=true>
                "Breadcrumb"
            </BBreadcrumbItem>
        </BBreadcrumb>
    }
}

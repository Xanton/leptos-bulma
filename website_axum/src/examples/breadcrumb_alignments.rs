use leptos::*;
use leptos_bulma::components::{BBreadcrumb, BBreadcrumbItem};
use leptos_bulma::enums::BAlignment;
use leptos::prelude::*;

const BREADCRUMB_ALIGNMENT_OPTIONS: [BAlignment; 3] =
    [BAlignment::Centered, BAlignment::Default, BAlignment::Right];

#[component]
pub fn BreadcrumbAlignments() -> impl IntoView {
    view! {
        <For each=move || BREADCRUMB_ALIGNMENT_OPTIONS key=|balignment| *balignment let:balignment>
            <BBreadcrumb alignment=balignment>
                <BBreadcrumbItem href_P="#">"Is"</BBreadcrumbItem>
                <BBreadcrumbItem href_P="#">"aligned"</BBreadcrumbItem>
                <BBreadcrumbItem href_P="#" is_active=true>
                    {String::from(balignment)}
                </BBreadcrumbItem>
            </BBreadcrumb>
        </For>
    }
}

use leptos::*;
use leptos_bulma::components::{BBreadcrumb, BBreadcrumbItem};
use leptos_bulma::enums::BBreadcrumbSeparator;
use leptos::prelude::*;

const BREADCRUMB_SEPARATOR_OPTIONS: [BBreadcrumbSeparator; 5] = [
    BBreadcrumbSeparator::Arrow,
    BBreadcrumbSeparator::Bullet,
    BBreadcrumbSeparator::Default,
    BBreadcrumbSeparator::Dot,
    BBreadcrumbSeparator::Succeeds,
];

#[component]
pub fn BreadcrumbSeparators() -> impl IntoView {
    view! {
        <For each=move || BREADCRUMB_SEPARATOR_OPTIONS key=|bseparator| *bseparator let:bseparator>
            <BBreadcrumb separator=bseparator>
                <BBreadcrumbItem href_P="#">"Is"</BBreadcrumbItem>
                <BBreadcrumbItem href_P="#">"separated"</BBreadcrumbItem>
                <BBreadcrumbItem href_P="#">"by"</BBreadcrumbItem>
                <BBreadcrumbItem href_P="#" is_active=true>
                    {String::from(bseparator)}
                </BBreadcrumbItem>
            </BBreadcrumb>
        </For>
    }
}

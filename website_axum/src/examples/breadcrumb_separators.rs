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
                <BBreadcrumbItem href_p="#">"Is"</BBreadcrumbItem>
                <BBreadcrumbItem href_p="#">"separated"</BBreadcrumbItem>
                <BBreadcrumbItem href_p="#">"by"</BBreadcrumbItem>
                <BBreadcrumbItem href_p="#" is_active=true>
                    {String::from(bseparator)}
                </BBreadcrumbItem>
            </BBreadcrumb>
        </For>
    }
}

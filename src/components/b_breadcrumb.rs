use leptos::*;
use leptos::children::*;
use leptos::text_prop::TextProp;
use leptos::prelude::*;

use crate::enums::{BAlignment, BBreadcrumbSeparator, BSize};

#[component]
pub fn BBreadcrumb(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BAlignment::Default.into(), into)] alignment: Signal<BAlignment>,
    #[prop(default = BBreadcrumbSeparator::Default.into(), into)] separator: Signal<
        BBreadcrumbSeparator,
    >,
    #[prop(default = BSize::Default.into(), into)] size: Signal<BSize>,
) -> impl IntoView {
    let breadcrumb_class_list = move || {
        let mut class_list = "breadcrumb".to_owned();

        if alignment.get() != BAlignment::Default {
            class_list += &format!(" is-{}", String::from(alignment.get()))
        };

        if separator.get() != BBreadcrumbSeparator::Default {
            class_list += &format!(" has-{}-separator", String::from(separator.get()))
        };

        size.get().add_to_class_list(&mut class_list);

        if !class.get().is_empty() {
            class_list += &format!(" {}", class.get());
        }

        class_list
    };

    view! {
        <nav class=breadcrumb_class_list>
            <ul>{children()}</ul>
        </nav>
    }
}

#[component]
pub fn BBreadcrumbItem(
    children: Children,
    #[prop(optional, into)] is_active: Signal<bool>,
    #[prop(optional, into)] href_p: TextProp,
) -> impl IntoView {
    view! {
        <li class:is-active=is_active>
            <a href=href_p.get()>{children()}</a>
        </li>
    }
}

use leptos::*;

use crate::enums::{BColor, BSize};
use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[component]
pub fn BTag(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BColor::Default.into(), into)] color: Signal<BColor>,
    #[prop(default = BSize::Default.into(), into)] size: Signal<BSize>,
    #[prop(optional, into)] is_dark: Signal<bool>,
    #[prop(optional, into)] is_delete: Signal<bool>,
    #[prop(optional, into)] is_hoverable: Signal<bool>,
    #[prop(optional, into)] is_light: Signal<bool>,
    #[prop(optional, into)] is_rounded: Signal<bool>,
) -> impl IntoView {
    let tag_class_list = move || {
        let mut class_list = "tag".to_owned();

        color.get().add_to_class_list(&mut class_list);
        size.get().add_to_class_list(&mut class_list);

        if is_dark.get() {
            class_list += " is-dark";
        }

        if is_delete.get() {
            class_list += " is-delete";
        }

        if is_hoverable.get() {
            class_list += " is-hoverable";
        }

        if is_light.get() {
            class_list += " is-light";
        }

        if is_rounded.get() {
            class_list += " is-rounded";
        }

        if !class.get().is_empty() {
            class_list += &format!(" {}", class.get());
        }

        class_list
    };

    view! { <span class=tag_class_list>{children.map(|c| c())}</span> }
}

#[component]
pub fn BTags(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BSize::Default.into(), into)] size: Signal<BSize>,
    #[prop(optional, into)] has_addons: Signal<bool>,
) -> impl IntoView {
    let tags_class_list = move || {
        let mut class_list = "tags".to_owned();

        if size.get() != BSize::Default {
            class_list += &format!(" are-{}", String::from(size.get()))
        };

        if has_addons.get() {
            class_list += " has-addons";
        }

        if !class.get().is_empty() {
            class_list += &format!(" {}", class.get());
        }

        class_list
    };

    view! { <div class=tags_class_list>{children()}</div> }
}

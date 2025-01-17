use leptos::*;
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use crate::enums::{BColor, BSize, BState};
use leptos::html::*;


fn get_button_class_list(
    class: Oco<'static, str>,
    color: BColor,
    size: BSize,
    state: BState,
    is_dark: bool,
    is_fullwidth: bool,
    is_inverted: bool,
    is_light: bool,
    is_outlined: bool,
    is_responsive: bool,
    is_rounded: bool,
) -> String {
    let mut class_list = "button".to_owned();

    color.add_to_class_list(&mut class_list);
    size.add_to_class_list(&mut class_list);

    if ![BState::Default, BState::Disabled].contains(&state) {
        class_list += &format!(" is-{}", String::from(state))
    };

    if is_dark {
        class_list += " is-dark";
    }

    if is_fullwidth {
        class_list += " is-fullwidth";
    }

    if is_inverted {
        class_list += " is-inverted";
    }

    if is_light {
        class_list += " is-light";
    }

    if is_outlined {
        class_list += " is-outlined";
    }

    if is_responsive {
        class_list += " is-responsive";
    }

    if is_rounded {
        class_list += " is-rounded";
    }

    if !class.is_empty() {
        class_list += &format!(" {}", class);
    }

    class_list
}

#[component]
pub fn BAButton(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BColor::Default.into(), into)] color: Signal<BColor>,
    #[prop(default = BSize::Default.into(), into)] size: Signal<BSize>,
    #[prop(default = BState::Default.into(), into)] state: Signal<BState>,
    #[prop(optional, into)] is_dark: Signal<bool>,
    #[prop(optional, into)] is_fullwidth: Signal<bool>,
    #[prop(optional, into)] is_inverted: Signal<bool>,
    #[prop(optional, into)] is_light: Signal<bool>,
    #[prop(optional, into)] is_outlined: Signal<bool>,
    #[prop(optional, into)] is_responsive: Signal<bool>,
    #[prop(optional, into)] is_rounded: Signal<bool>,
    #[prop(optional, into)] href: TextProp,
    #[prop(optional, into)] target: TextProp,
    #[prop(optional, into)] title: TextProp,
) -> impl IntoView {
    let button_class_list = move || {
        get_button_class_list(
            class.get(),
            color.get(),
            size.get(),
            state.get(),
            is_dark.get(),
            is_fullwidth.get(),
            is_inverted.get(),
            is_light.get(),
            is_outlined.get(),
            is_responsive.get(),
            is_rounded.get(),
        )
    };
//disabled=move || {state.get() == BState::Disabled}
    view! {
        <a class=button_class_list href=href.get() target=target.get() title=title.get() >
            {children()}
        </a>
    }
}

#[component]
pub fn BButton(
    #[prop(optional, into)] button_type: Option<String>,
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BColor::Default.into(), into)] color: Signal<BColor>,
    #[prop(default = BSize::Default.into(), into)] size: Signal<BSize>,
    #[prop(default = BState::Default.into(), into)] state: Signal<BState>,
    #[prop(optional, into)] is_dark: Signal<bool>,
    #[prop(optional, into)] is_fullwidth: Signal<bool>,
    #[prop(optional, into)] is_inverted: Signal<bool>,
    #[prop(optional, into)] is_light: Signal<bool>,
    #[prop(optional, into)] is_outlined: Signal<bool>,
    #[prop(optional, into)] is_responsive: Signal<bool>,
    #[prop(optional, into)] is_rounded: Signal<bool>,
    #[prop(optional, into)] title: TextProp,
) -> impl IntoView {
    let button_class_list = move || {
        get_button_class_list(
            class.get(),
            color.get(),
            size.get(),
            state.get(),
            is_dark.get(),
            is_fullwidth.get(),
            is_inverted.get(),
            is_light.get(),
            is_outlined.get(),
            is_responsive.get(),
            is_rounded.get(),
        )
    };

    view! {
        <button class=button_class_list type=button_type title=title.get() disabled=move || state.get() == BState::Disabled>
            {children()}
        </button>
    }
}

#[component]
pub fn BButtons(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BSize::Default.into(), into)] size: Signal<BSize>,
    #[prop(optional, into)] has_addons: Signal<bool>,
) -> impl IntoView {
    let buttons_class_list = move || {
        let mut class_list = "buttons".to_owned();

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

    view! { <div class=buttons_class_list>{children()}</div> }
}

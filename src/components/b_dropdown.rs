use leptos::html::*;
use leptos::*;
use leptos::prelude::*;
use leptos::text_prop::TextProp;
use crate::elements::BButton;

#[component]
pub fn BDropdown<F, IV>(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] is_right: Signal<bool>,
    #[prop(optional, into)] is_up: Signal<bool>,
    #[prop(optional, into)] is_hoverable: Signal<bool>,
    trigger: F,
) -> impl IntoView
where
    F: Fn() -> IV + 'static + std::marker::Send,
    IV: IntoView + 'static,
{
    let node_ref:NodeRef<Div> = NodeRef::new();
    let is_active = RwSignal::new(false);

    let _ = leptos_use::on_click_outside(node_ref, move |_| {
        is_active.set(false);
    });

    let toggle_dropdown = move |_| {
        if is_hoverable.get() {
            is_active.set(false);
        } else {
            is_active.update(|value| *value = !*value);
        }
    };

    view! {
        <div
            node_ref=node_ref
            class=format!("dropdown {}", class.get())
            class:is-active=is_active
            class:is-hoverable=is_hoverable
            class:is-right=is_right
            class:is-up=is_up
        >
            <div class="dropdown-trigger">
                <BButton on:click=toggle_dropdown>{trigger()}</BButton>
            </div>

            <div class="dropdown-menu">
                <div class="dropdown-content">{children()}</div>
            </div>
        </div>
    }
}

#[component]
pub fn BDropdownDivider() -> impl IntoView {
    view! { <hr class="dropdown-divider"/> }
}

#[component]
pub fn BDropdownItem(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional)] href: Option<&'static str>,
) -> impl IntoView
where
{
    view! {
        <a class=format!("dropdown-item {}", class.get()) href=href>
            {children()}
        </a>
    }
}

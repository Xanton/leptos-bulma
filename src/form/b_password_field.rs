use leptos::*;

use crate::EventFn;

use super::{BControl, BField, BHelp, BLabel};
use leptos::prelude::*;

#[cfg(feature = "icondata-fa")]
#[component]
fn VisibilityIcon(is_visible: RwSignal<bool>) -> impl IntoView {
    use crate::elements::BIcon;

    let visibility_icon = RwSignal::new(icondata_fa::FaEyeSlashSolid);

    Effect::new(move |_| {
        visibility_icon.set(if is_visible.get() {
            icondata_fa::FaEyeSolid
        } else {
            icondata_fa::FaEyeSlashSolid
        })
    });

    view! { <BIcon is_scaled=false icon=visibility_icon/> }
}

#[cfg(not(feature = "icondata-fa"))]
#[component]
fn VisibilityIcon(is_visible: RwSignal<bool>) -> impl IntoView {
    let text_decoration = RwSignal::new("line-through");

    Effect::new(move |_| {
        text_decoration.set(if is_visible.get() {
            "none"
        } else {
            "line-through"
        })
    });

    view! { <span style:text-decoration=text_decoration>"👁"</span> }
}

#[allow(unused_variables)]
#[component]
pub fn BPasswordField(
    #[prop(optional)] node_ref: NodeRef<leptos::html::Input>,
    #[prop(optional, into)] error: Signal<Option<String>>,
    #[prop(optional)] id: &'static str,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] name: &'static str,
    #[prop(optional)] placeholder: &'static str,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] on_change: Option<EventFn>,
    #[prop(optional, into)] on_input: Option<EventFn>,
) -> impl IntoView {
    let error_text = RwSignal::new(None);
    let is_visible = RwSignal::new(false);

    Effect::new(move |_| {
        error_text.set(error.get().map(|e| e.trim().to_owned()));
    });

    let has_error = move || error_text.get().is_some();

    let input_class = move || {
        if has_error() {
            "input is-danger"
        } else {
            "input"
        }
    };

    let button_class = move || {
        if has_error() {
            "button is-danger is-outlined"
        } else {
            "button"
        }
    };

    let input_type = move || {
        if is_visible.get() {
            "text"
        } else {
            "password"
        }
    };

    let input_view = view! {
        <input node_ref=node_ref class=input_class id=id type=input_type name=name placeholder=placeholder value=value/>
    };
    if on_change.is_some() {
        node_ref.on_load(|element| { let _ = element.on(ev::change, on_change.unwrap().into_inner());});
    }
    if on_input.is_some() {
        node_ref.on_load(|element| { let _ = element.on(ev::input, on_input.unwrap().into_inner());});
    }

    view! {
        <BField>
            <Show when=move || label.is_some()>
                <BLabel for_id=id>{label.unwrap()}</BLabel>
            </Show>

            <BField class="has-addons">
                <BControl class="is-expanded">{input_view}</BControl>

                <BControl>
                    <a class=button_class on:click=move |_| { is_visible.update(|value| *value = !*value) }>
                        <VisibilityIcon is_visible=is_visible/>
                    </a>
                </BControl>
            </BField>

            <Show when=has_error>
                <BHelp class="is-danger">{move || error_text.get()}</BHelp>
            </Show>
        </BField>
    }
}

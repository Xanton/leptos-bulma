use leptos::*;

use crate::EventFn;

use super::{BControl, BField, BHelp, BLabel};
use leptos::prelude::*;

#[allow(unused_variables)]
#[component]
pub fn BSelectField(
    #[prop(optional)] node_ref: NodeRef<leptos::html::Select>,
    #[prop(optional, into)] error: Signal<Option<String>>,
    #[prop(optional, into)] id: &'static str,
    #[prop(optional)] label: Option<&'static str>,
    #[prop(optional)] name: &'static str,
    #[prop(optional, into)] options: Signal<Vec<(String, String)>>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] on_change: Option<EventFn>,
) -> impl IntoView {
    let error_text = RwSignal::new(None);

    Effect::new(move |_| {
        error_text.set(error.get().map(|e| e.trim().to_owned()));
    });

    let has_error = move || error_text.get().is_some();

    let select_class = move || {
        if has_error() {
            "select is-danger"
        } else {
            "select"
        }
    };

    let options_view = move || {
        let mut options_vec = vec![];

        for option in options.get() {
            let selected = if option.1.clone() == value.get() {
                Some("selected")
            } else {
                None
            };

            options_vec.push(
                view! {
                    <option value=option.1.clone() selected=selected>
                        {option.0.clone()}
                    </option>
                }
                .into_view(),
            );
        }

        options_vec
    };

    let select_view = view! {
        <select node_ref=node_ref id=id name=name>
            {options_view}
        </select>
    };
    if on_change.is_some() {
        node_ref.on_load(|element| { let _ = element.on(ev::change, on_change.unwrap().into_inner());});
    }
    //.optional_event(ev::change, on_change.map(EventFn::into_inner));

    view! {
        <BField>
            <Show when=move || label.is_some()>
                <BLabel for_id=id>{label.unwrap()}</BLabel>
            </Show>

            <BControl class="is-expanded">
                <div class=select_class>{select_view}</div>
            </BControl>

            <Show when=has_error>
                <BHelp class="is-danger">{move || error_text.get()}</BHelp>
            </Show>
        </BField>
    }
}

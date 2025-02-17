use leptos::*;

use crate::EventFn;

use super::{BCheckbox, BCheckboxProps, BControl, BField, BHelp};
use leptos::prelude::*;
#[component]
pub fn BCheckboxField(
    #[prop(optional)] node_ref: NodeRef<leptos::html::Input>,
    #[prop(optional, into)] error: Signal<Option<String>>,
    #[prop(optional, into)] id: Option<&'static str>,
    label: &'static str,
    #[prop(optional, into)] name: Option<&'static str>,
    #[prop(default = "true".into(), into)] value: Signal<String>,
    #[prop(optional, into)] is_checked: Signal<bool>,
    #[prop(optional, into)] on_change: Option<EventFn>,
) -> impl IntoView {
    let error_text = RwSignal::new(None);

    Effect::new(move |_| {
        error_text.set(error.get().map(|e| e.trim().to_owned()));
    });

    let has_error = move || error_text.get().is_some();

    let b_checkbox = BCheckbox(BCheckboxProps {
        node_ref,
        class: "".into(),
        id,
        label,
        name,
        value,
        is_checked,
        on_change,
        attributes: vec![],
    });

    view! {
        <BField>
            <BControl>{b_checkbox}</BControl>

            <Show when=has_error>
                <BHelp class="is-danger">{move || error_text.get()}</BHelp>
            </Show>
        </BField>
    }
}

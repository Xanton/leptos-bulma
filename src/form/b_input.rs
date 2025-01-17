use leptos::*;
use leptos::attr::Attribute;
use crate::EventFn;
use leptos::prelude::*;

#[allow(unused_variables)]
#[component]
pub fn BInput(
    #[prop(optional)] node_ref: NodeRef<leptos::html::Input>,
    #[prop(optional, into)] class: Option<&'static str>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(default = "text")] input_type: &'static str,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(optional, into)] on_change: Option<EventFn>,
    #[prop(optional, into)] on_input: Option<EventFn>,
    #[prop(attrs, optional)] attributes: Vec<(&'static str, String)>,
) -> impl IntoView {
    let mut b_input = view! {
        <input
            node_ref=node_ref
            class=format!("input {}", class.unwrap_or_default())
            type=input_type
            id=id
            name=name
            placeholder=placeholder
            value=value
        />
    };
    if on_change.is_some() {
        node_ref.on_load(|element| { let _ = element.on(ev::change, on_change.unwrap().into_inner());});
    }
    if on_input.is_some() {
        node_ref.on_load(|element| { let _ = element.on(ev::input, on_input.unwrap().into_inner());});
    }

    //for (attr_name, attr_value) in attributes {
    //    b_input = b_input.attr(attr_name, attr_value);
    //}

    b_input
}

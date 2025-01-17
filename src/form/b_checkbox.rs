use leptos::*;
use leptos::attr::Attribute;
use crate::EventFn;
use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[allow(unused_variables)]
#[component]
pub fn BCheckbox(
    #[prop(optional)] node_ref: NodeRef<leptos::html::Input>,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] id: Option<&'static str>,
    label: &'static str,
    #[prop(optional, into)] name: Option<&'static str>,
    #[prop(default = "true".into(), into)] value: Signal<String>,
    #[prop(optional, into)] is_checked: Signal<bool>,
    #[prop(optional, into)] on_change: Option<EventFn>,
    #[prop(attrs, optional)] attributes: Vec<(&'static str, String)>,
) -> impl IntoView {
    let mut b_input =
        view! { <input node_ref=node_ref id=id name=name type="checkbox" value=value checked=is_checked/> };
    //.optional_event(ev::change, on_change.map(EventFn::into_inner));
    if on_change.is_some() {
        node_ref.on_load(|element| { let _ = element.on(ev::change, on_change.unwrap().into_inner());});
    }

    //for (attr_name, attr_value) in attributes {
    //    let b_input = b_input.attr(attr_name, attr_value);
    //}

    view! { <label class=format!("checkbox {}", class.get())>{b_input} " " {label}</label> }
}

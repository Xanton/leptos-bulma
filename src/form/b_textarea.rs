use leptos::*;
use leptos::attr::Attr;
use leptos::prelude::*;
//use leptos::server_fn::serde::__private::de::Content::String;

#[component]
pub fn BTextarea(
    #[prop(optional, into)] class: Option<&'static str>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional)] placeholder: Option<&'static str>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(attrs, optional)] attributes: Vec<(&'static str, String)>,
) -> impl IntoView {
    //let mut b_textarea =

    //let mut dynamicAttribute = String::new();
    //for (attr_name, attr_value) in attributes {
    //    dynamicAttribute = dynamicAttribute + format!("{}={} ", attr_name, attr_value).as_str();
    //}
    //{dynamicAttribute}
    view! {
        <textarea class=class id=id name=name prop:value=value placeholder=placeholder/>
    }

    //for (attr_name, attr_value) in attributes {
    //    b_textarea = b_textarea.attr(attr_name, attr_value);
    //}


    //b_textarea
}

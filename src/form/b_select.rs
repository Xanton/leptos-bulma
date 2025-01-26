use leptos::*;
//use leptos::attr::Attribute;
use leptos::html::*;
use leptos::prelude::*;
#[allow(unused_variables)]
#[component]
pub fn BSelect(
    #[prop(optional)] node_ref: NodeRef<leptos::html::Select>,
    #[prop(optional)] id: Option<&'static str>,
    #[prop(optional)] name: Option<&'static str>,
    #[prop(optional, into)] options: Signal<Vec<(String, String)>>,
    #[prop(optional, into)] value: Signal<String>,
    #[prop(attrs, optional)] attributes: Vec<(&'static str, String)>,
) -> impl IntoView {
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

    let mut b_select= view! {
        <div class="select">
            <select node_ref=node_ref id=id name=name>
                {options_view}
            </select>
        </div>
    };

    //for (attr_name, attr_value) in attributes {
    //    b_select = b_select.attr(attr_name, attr_value);
    //}

    b_select
}

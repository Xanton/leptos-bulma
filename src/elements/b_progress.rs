use leptos::*;

use crate::enums::{BColor, BSize};
use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[component]
pub fn BProgress(
    #[prop(optional)] children: Option<Children>,
    #[prop(optional, into)] class: TextProp,
    #[prop(default = BColor::Default.into(), into)] color: Signal<BColor>,
    #[prop(default = BSize::Default.into(), into)] size: Signal<BSize>,
    #[prop(into)] max: Signal<usize>,
    #[prop(optional, into)] value: MaybeProp<usize>,
) -> impl IntoView {
    let mut class_list = "progress".to_owned();

    color.get().add_to_class_list(&mut class_list);

    size.get().add_to_class_list(&mut class_list);

    if !class.get().is_empty() {
        class_list += &format!(" {}", class.get());
    }

    view! {
        <progress class=class_list max=max value=value.get()>
            {children.map(|c| c())}
        </progress>
    }
}

use leptos::*;
use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[component]
pub fn BTitle(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] id: TextProp,
    #[prop(optional)] is: Option<i8>,
) -> impl IntoView {
    let title_class = move || {
        let mut t_class = format!("title {}", class.get());

        if let Some(is) = is {
            t_class += &format!(" is-{}", is);
        }

        t_class
    };

    view! {
        <div class=title_class id=id.get()>
            {children()}
        </div>
    }
}

#[component]
pub fn BSubtitle(
    children: Children,
    #[prop(optional, into)] class: TextProp,
    #[prop(optional)] is: Option<i8>,
) -> impl IntoView {
    let subtitle_class = move || {
        let mut s_class = format!("subtitle {}", class.get());

        if let Some(is) = is {
            s_class += &format!(" is-{}", is);
        }

        s_class
    };

    view! { <div class=subtitle_class>{children()}</div> }
}

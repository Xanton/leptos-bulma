use leptos::html::*;
use leptos::*;
use leptos::prelude::*;
use leptos_bulma::elements::BBlock;
use leptos_meta::Title;
use web_sys::HtmlElement;
use leptos::text_prop::TextProp;
use leptos_use::UseColorModeReturn;
use crate::app::use_app_color_mode;
use crate::i18n::{t, use_i18n};

#[component]
pub fn PageTitle(#[prop(into)] text: TextProp) -> impl IntoView {
    view! { <Title text=format!("{} | Leptos Bulma", text.get())/> }
}

use leptos_use::{ColorMode};




#[component]
pub fn CodeBlock(
    children: Children,
    #[prop(optional, into)] node_ref :NodeRef<Code>,
    #[prop(default = "plaintext")] language: &'static str,
) -> impl IntoView {
    view! {
        <pre class="block">
            <code node_ref=node_ref class=format!("language-{language}")>
                {children()}
            </code>
        </pre>
    }
}

#[component]
pub fn GoToBulmaIo(path: &'static str) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <BBlock>
            {t!(i18n, additionally_you_can_check_bulma_official_documentation)} ": "
            <a href=format!("https://bulma.io/documentation/{path}") target="_blank">
                {format!("bulma.io/documentation/{path}")}
            </a>
        </BBlock>
    }
}

#[component]
pub fn GoToDocsRs(path: &'static str) -> impl IntoView {
    let i18n = use_i18n();
    let version = env!("CARGO_PKG_VERSION");

    view! {
        <BBlock>
            {t!(i18n, to_find_more_information_you_can_go_to)}": "
            <a href=format!("https://docs.rs/leptos-bulma/{version}/leptos_bulma/{path}.html") target="_blank">
                {format!("docs.rs/leptos-bulma/{version}/leptos_bulma/{path}.html")}
            </a>
        </BBlock>
    }
}

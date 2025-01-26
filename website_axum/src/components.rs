use leptos::html::*;
use leptos::*;
use leptos::prelude::*;
use leptos_bulma::elements::BBlock;
use leptos_meta::Title;
//use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::HtmlElement;
use leptos::text_prop::TextProp;
use leptos_use::UseColorModeReturn;
use crate::app::use_app_color_mode;
use crate::i18n::{t, use_i18n};

#[component]
pub fn PageTitle(#[prop(into)] text: TextProp) -> impl IntoView {
    view! { <Title text=format!("{} | Leptos Bulma", text.get())/> }
}

use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use leptos_use::{ColorMode};




#[component]
pub fn CodeBlock(
    children: Children,
    #[prop(optional, into)] node_ref :NodeRef<Code>,
    #[prop(default = "plaintext")] language: &'static str,
) -> impl IntoView {


    /* let sourceCode=children().to_html().clone();
    let ts = ThemeSet::load_defaults();
    let themeLight= &ts.themes["base16-ocean.light"];
    let themeDark = &ts.themes["base16-ocean.dark"];

        let hlLight = get_hl(sourceCode.clone(),language,themeLight);
        let hlDark = get_hl(sourceCode.clone(),language,themeDark);
      if let Some(element) = node_ref.get() {
           if let Some(el) = element.dyn_ref::<HtmlElement>() {
               if mode.get()==ColorMode::Light{
                   el.set_inner_html(hlLight.as_str());
               } else {
                   el.set_inner_html(hlDark.as_str());
               }
           }
       }
   */

    /*Effect::new(move |colormode| {
        let colormode=mode.get();
        if let Some(element) = node_ref.get() {
            if let Some(el) = element.dyn_ref::<HtmlElement>() {
                if colormode==ColorMode::Light{
                    el.set_inner_html(hlLight.as_str());
                } else {
                    el.set_inner_html(hlDark.as_str());
                }
            }
        }
    });*/

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

use leptos::html::*;
use leptos::*;
use leptos::prelude::*;
use leptos_bulma::elements::BBlock;
use leptos_meta::Title;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::HtmlElement;
use leptos::text_prop::TextProp;
use leptos_use::{ColorMode, UseColorModeReturn};
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use crate::app::use_app_color_mode;
use crate::i18n::{t, use_i18n};

#[component]
pub fn PageTitle(#[prop(into)] text: TextProp) -> impl IntoView {
    view! { <Title text=format!("{} | Leptos Bulma", text.get())/> }
}



fn highlightCode(data: String, theme: &Theme) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let syntax =ss.find_syntax_by_extension("rs").unwrap();
    highlighted_html_for_string(format!("{}",data).as_str(),&ss,&syntax,&theme).unwrap()
}

#[component]
pub fn CodeBlock(
    children: Children,
    #[prop(default = "plaintext")] language: &'static str,
) -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_app_color_mode();

    let node_ref_d:NodeRef<Code> = NodeRef::new();

    Effect::new(move |_| {
        if let Some(element) = node_ref_d.get() {
            if let Some(el) = element.dyn_ref::<HtmlElement>() {
                let ts = ThemeSet::load_defaults();
                let themeDark = &ts.themes["base16-ocean.dark"];
                el.set_inner_html(highlightCode(el.inner_text(),themeDark).as_str());
            }
        }
    });


    view! {
        <pre class="block">
            {
                if mode.get()==ColorMode::Light{
                    let node_ref_l:NodeRef<Code> = NodeRef::new();

                    Effect::new(move |_| {
                        if let Some(element) = node_ref_l.get() {
                            if let Some(el) = element.dyn_ref::<HtmlElement>() {
                                let ts = ThemeSet::load_defaults();
                                let themeLight = &ts.themes["base16-ocean.light"];
                                el.set_inner_html(highlightCode(el.inner_text(), &themeLight).as_str());
                            }
                        }
                    });
                    view! {
                        <code node_ref=node_ref_l class=format!("language-{language}")>
                            {children()}
                        </code>
                    }
                }else{
                    view! {
                        <code node_ref=node_ref_d class=format!("language-{language}")>
                            {children()}
                        </code>
                    }
                }
            }
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

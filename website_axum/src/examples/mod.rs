use leptos::*;
use leptos::attr::Name;
use leptos::prelude::*;
use leptos::html::*;
use leptos_meta::*;
use leptos_router::*;
use leptos::task::*;
use server_fn::{
    client::{browser::BrowserClient, Client},
    codec::{
        FromReq, FromRes, GetUrl, IntoReq, IntoRes, MultipartData,
        MultipartFormData, Postcard, Rkyv, SerdeLite, StreamingText,
        TextStream,
    },
    request::{browser::BrowserRequest, ClientReq, Req},
    response::{browser::BrowserResponse, ClientRes, Res},
};
use std::path::Path;
use leptos::web_sys::HtmlElement;
use leptos_use::UseColorModeReturn;

use crate::components::CodeBlock;

mod basic_block;
mod basic_box;
mod basic_breadcrumb;
mod basic_buttons;
mod basic_columns;
mod basic_dropdown;
mod basic_icons;
mod basic_modal;
mod basic_navbar;
mod basic_notification;
mod basic_pagination;
mod basic_progress;
mod basic_section;
mod basic_table;
mod basic_tags;
mod basic_title;
mod breadcrumb_alignments;
mod breadcrumb_separators;
mod breadcrumb_sizes;
mod button_colors;
mod button_sizes;
mod button_states;
mod general_form;
mod icon_packages;
mod icon_sizes;
mod notification_colors;
mod notification_light_colors;
mod progress_colors;
mod progress_indeterminate;
mod progress_sizes;
mod tag_addons;
mod tag_colors;
mod tag_sizes;

pub use basic_block::BasicBlock;
pub use basic_box::BasicBox;
pub use basic_breadcrumb::BasicBreadcrumb;
pub use basic_buttons::BasicButtons;
pub use basic_columns::BasicColumns;
pub use basic_dropdown::BasicDropdown;
pub use basic_icons::BasicIcons;
pub use basic_modal::BasicModal;
pub use basic_navbar::BasicNavbar;
pub use basic_notification::BasicNotification;
pub use basic_pagination::BasicPagination;
pub use basic_progress::BasicProgress;
pub use basic_section::BasicSection;
pub use basic_table::BasicTable;
pub use basic_tags::BasicTags;
pub use basic_title::BasicTitle;
pub use breadcrumb_alignments::BreadcrumbAlignments;
pub use breadcrumb_separators::BreadcrumbSeparators;
pub use breadcrumb_sizes::BreadcrumbSizes;
pub use button_colors::ButtonColors;
pub use button_sizes::ButtonSizes;
pub use button_states::ButtonStates;
pub use general_form::GeneralForm;
pub use icon_packages::IconPackages;
pub use icon_sizes::IconSizes;
pub use notification_colors::NotificationColors;
pub use notification_light_colors::NotificationLightColors;
pub use progress_colors::ProgressColors;
pub use progress_indeterminate::ProgressIndeterminate;
pub use progress_sizes::ProgressSizes;
pub use tag_addons::TagAddons;
pub use tag_colors::TagColors;
pub use tag_sizes::TagSizes;



use leptos_use::{ColorMode};
use crate::app::use_app_color_mode;
use wasm_bindgen::prelude::*;

/*#[server(
    // this server function will be exposed at /api2/custom_path
    prefix = "/api",
    endpoint = "getCode",
    // it will take its arguments as a URL-encoded GET request (useful for caching)
    input = GetUrl,
    // it will return its output using SerdeLite
    // (this needs to be enabled with the `serde-lite` feature on the `server_fn` crate
    output = SerdeLite,
)]
    pub async fn get_code_example(input: String) -> Result<(String,String), ServerFnError> {
    println!("get code");
    //use regex::Regex;
    //let re = Regex::new(r"^(([a-zA-Z]:|\\)\\)?(((\.)|(\.\.)|([^\\/:*?"|<>. ](([^\\/:*?"|<>. ])|([^\\/:*?"|<>]*[^\\/:*?"|<>. ]))?))\\)*[^\\/:*?"|<>. ](([^\\/:*?"|<>. ])|([^\\/:*?"|<>]*[^\\/:*?"|<>. ]))?$").unwrap();
    //re
    let code_example_path = format!("./src/examples/{input}.rs");
    //use std::path::{self, Path};
    //let absolute = path::absolute(".")?;
    println!("absolutabsolutee: {}", code_example_path);
    let data = std::fs::read_to_string(code_example_path);
    if data.is_ok() {
        let ss = SyntaxSet::load_defaults_newlines();
        let syntax = ss.find_syntax_by_extension("rs");
        let ts = ThemeSet::load_defaults();
        let themeLight= &ts.themes["base16-ocean.light"];
        let themeDark = &ts.themes["base16-ocean.dark"];
        let code = data.unwrap();
        let hlL = highlighted_html_for_string(code.clone().as_str(), &ss, &(syntax.unwrap()), themeLight);
        let hlD = highlighted_html_for_string(code.clone().as_str(), &ss, &(syntax.unwrap()), themeDark);
        Ok((hlL.unwrap(),hlD.unwrap()))
    }else{
        Err(ServerFnError::ServerError(data.err().unwrap().to_string()))
    }
    //let html = highlighted_html_for_file(code_example_path, &ss, theme);
}*/

#[component]
pub fn RustCodeExample(name: &'static str) -> impl IntoView {

    let UseColorModeReturn { mode, set_mode, .. } = use_app_color_mode();
    let light_path = format!("/highlight/light/{}.rs.html",name);
    let dark_path = format!("/highlight/dark/{}.rs.html",name);


    view! {
        <div style = move || { match mode.get(){
                    ColorMode::Dark => "display:none",
                    ColorMode::Light => "display:",
                    _ => "display:none"
                }
        }>
        //<CodeBlock language="rs">
            <object data=light_path height="auto" width="100%" type="text/html"/>
        //</CodeBlock>
        </div>
        <div style = move || { match mode.get(){
                    ColorMode::Dark => "display:",
                    ColorMode::Light => "display:none",
                    _ => "display:"
                    }
        }>
        //<CodeBlock language="rs">
            <object data=dark_path height="auto" width="100%" type="text/html"/>
        //</CodeBlock>
        </div>
        /*<Suspense fallback=move || {
            view! { <CodeBlock>"Loading..."</CodeBlock> }
        }>
            {
                    let resource = Resource::new(|| (),move |_| async move {
                    let tmp=get_code_example(format!("{}",name)).await;
                    if tmp.is_ok(){
                        tmp.unwrap()
                    }
                    else {
                        let error =tmp.err().unwrap().to_string();
                        (format!("{}",error.clone()),format!("{}",error.clone()))
                    }
                });
                move || Suspend::new(async move {
                let data = resource.await;
                let node_ref:NodeRef<Code> = NodeRef::new();

                Effect::new(move |colormode| {
                    let colormode=mode.get();
                    if let Some(element) = node_ref.get() {
                        if let Some(el) = element.dyn_ref::<HtmlElement>() {
                            if colormode==ColorMode::Light{
                                el.set_inner_html(data.0.as_str());
                            } else {
                                el.set_inner_html(data.1.as_str());
                            }
                        }
                    }
                });
                view! {

                }
            })}
        </Suspense>*/
    }
}

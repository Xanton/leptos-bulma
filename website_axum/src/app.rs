use std::sync::LazyLock;

use leptos::prelude::*;
use leptos_meta::MetaTags;
use leptos_meta::*;
use leptos_router::{components::{FlatRoutes, ProtectedRoute, Route, Router, Routes}, hooks::use_params, params::Params, path, ParamSegment, SsrMode, StaticSegment};

use leptos_use::{
    use_color_mode_with_options, use_cookie, UseColorModeOptions, UseColorModeReturn,
};

use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use std::sync::atomic::{AtomicBool, Ordering};
//use thiserror::Error;

use leptos_i18n::Locale;
use crate::pages::components::BreadcrumbPage;
use crate::i18n::provide_i18n_context;
use crate::layout::Layout;
use crate::pages::*;


pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <App/>
        </html>
    }
}

/*
#[cfg(feature = "ssr")]
static IS_ADMIN: AtomicBool = AtomicBool::new(true);

#[server]
pub async fn is_admin() -> Result<bool, ServerFnError> {
    Ok(IS_ADMIN.load(Ordering::Relaxed))
}

#[server]
pub async fn set_is_admin(is_admin: bool) -> Result<(), ServerFnError> {
    IS_ADMIN.store(is_admin, Ordering::Relaxed);
    Ok(())
}
*/

pub fn use_app_color_mode() -> UseColorModeReturn {
    use_color_mode_with_options(
        UseColorModeOptions::default()
            .cookie_enabled(true)
            .cookie_name("pref_color_mode")
            .attribute("data-theme")
            .emit_auto(true)
            .transition_enabled(true),
    )
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    let ct=provide_meta_context();
    //provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();
    //let toggle_admin = ServerAction::<SetIsAdmin>::new();
    //let is_admin =
    //    Resource::new(move || toggle_admin.version().get(), |_| is_admin());


    let i18n = provide_i18n_context();

    //let (locale_cookie, _) = use_cookie::<String, FromToStringCodec>("i18n_pref_locale");


    view! {
        <body>
        <Stylesheet id="leptos" href="/pkg/ssr_modes.css" />

        // sets the favicon
        <Link rel="icon" href="/images/favicon.png" />

        // sets the document title
        <Title text="Leptos Bulma - A Leptos component library based on Bulma CSS framework" />

        <div class="loading-overlay" class:is-done=true></div>

        // content for this welcome page
        <Router>//trailing_slash=TrailingSlash::Redirect
            <Layout>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/columns") view=ColumnsPage ssr=SsrMode::Async/>
                    <Route path=path!("/components") view=ComponentsPage ssr=SsrMode::Async/>
                    <Route path=path!("/components/breadcrumb") view=components::BreadcrumbPage ssr=SsrMode::Async/>
                    <Route path=path!("/elements") view=ElementsPage ssr=SsrMode::Async/>
                    <Route path=path!("/elements/button") view=elements::ButtonPage ssr=SsrMode::Async/>
                    <Route path=path!("/elements/icon") view=elements::IconPage ssr=SsrMode::Async/>
                    <Route path=path!("/elements/notification") view=elements::NotificationPage ssr=SsrMode::Async/>
                    <Route path=path!("/elements/progress") view=elements::ProgressPage ssr=SsrMode::Async/>
                    <Route path=path!("/elements/tag") view=elements::TagPage ssr=SsrMode::Async/>
                    <Route path=path!("/form") view=FormPage ssr=SsrMode::Async/>
                    <Route path=path!("/guides") view=GuidesPage ssr=SsrMode::Async/>
                    <Route path=path!("/layout") view=LayoutPage ssr=SsrMode::Async/>
                    <Route path=path!("/") view=HomePage ssr=SsrMode::Async/>
                    //<Route path=path!("/*") view=NotFoundPage ssr=SsrMode::Async/>
                </Routes>
            </Layout>
        </Router>
        // add syntax highlighting here: 
        <script src="/highlight/highlight.min.js"></script>
        <link rel="stylesheet" href="/highlight/styles/default.css"/>
        <script>hljs.highlightAll();</script>
        </body>
    }
}



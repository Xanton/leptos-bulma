use std::sync::LazyLock;

use leptos::prelude::*;
use leptos_meta::MetaTags;
use leptos_meta::*;
use leptos_router::{components::{FlatRoutes, ProtectedRoute, Route, Router, Routes}, hooks::use_params, params::Params, path, ParamSegment, SsrMode, StaticSegment};
use leptos_bulma::columns::{BColumn, BColumns};
use leptos_bulma::components::{
    BDropdown, BDropdownItem//, BNavbar, BNavbarBrand, BNavbarBurger, BNavbarEnd, BNavbarItem,
    //BNavbarItemDropdown, BNavbarMenu, BNavbarStart,
};
use leptos_bulma::elements::{BButton, BButtons, BIcon};
use leptos_bulma::enums::{BSize, BState};
use leptos_bulma::icons::icondata_fa;

//use crate::app::use_app_color_mode;

use crate::i18n::{t, use_i18n, Locale};


//use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use std::sync::atomic::{AtomicBool, Ordering};
use leptos_use::UseColorModeReturn;
//use thiserror::Error;

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

        <body>
            <App/>
                </body>
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
            .transition_enabled(true).cookie_enabled(true),
    )
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    let ct=provide_meta_context();
    //provide_meta_context();
    let fallback = || NotFoundPage.into_view();
    //let toggle_admin = ServerAction::<SetIsAdmin>::new();
    //let is_admin =
    //    Resource::new(move || toggle_admin.version().get(), |_| is_admin());

    let UseColorModeReturn { mode, set_mode, .. } = use_app_color_mode();

    let i18n = provide_i18n_context();

    //let (locale_cookie, _) = use_cookie::<String, FromToStringCodec>("i18n_pref_locale");

    let i18n = use_i18n();

    view! {
        <Stylesheet id="leptos" href="/pkg/ssr_modes.css" />

        // sets the favicon
        <Link rel="icon" href="/images/favicon.png" />

        // sets the document title
        <Title text="Leptos Bulma - A Leptos component library based on Bulma CSS framework" />

        <div class="loading-overlay" class:is-done=true></div>

        // content for this welcome page
        <Router>//trailing_slash=TrailingSlash::Redirect
            <Layout>
                <Routes fallback=fallback>
                    <Route path=path!("/columns") view=ColumnsPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/components") view=ComponentsPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/components/breadcrumb") view=components::BreadcrumbPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/elements") view=ElementsPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/elements/button") view=elements::ButtonPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/elements/icon") view=elements::IconPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/elements/notification") view=elements::NotificationPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/elements/progress") view=elements::ProgressPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/elements/tag") view=elements::TagPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/form") view=FormPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/guides") view=GuidesPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/layout") view=LayoutPage ssr=SsrMode::InOrder/>
                    <Route path=path!("/") view=HomePage ssr=SsrMode::InOrder/>
                </Routes>
            </Layout>
        </Router>

        <footer class="footer">
            <div class="content container">
                <BColumns>
                    <BColumn>
                        <div class="is-flex is-align-items-center is-justify-content-center">
                            {t!(i18n, this_website_was_made_with)}
                            <a class="mx-3" href="https://leptos.dev" target="_blank" title="Go to Leptos">
                                <ImageColorModes
                                    dark_src="/images/leptos-logo-light.svg"
                                    light_src="/images/leptos-logo.svg"
                                    alt="Leptos"
                                    width=100
                                />
                            </a>& <a class="mx-3" href="https://bulma.io/" target="_blank" title="Go to Bulma">
                                <ImageColorModes
                                    dark_src="/images/bulma-logo-light.svg"
                                    light_src="/images/bulma-logo.svg"
                                    alt="Bulma"
                                    width=100
                                />
                            </a>
                        </div>
                        <div class="mt-3 is-flex is-align-items-center is-justify-content-center">
                            {t!(i18n, and_you_can_see_the_source_code_at)}
                            <a
                                class="mx-3"
                                href="https://github.com/javierEd/leptos-bulma/blob/main/website"
                                target="_blank"
                                title="Go to GitHub"
                            >
                                <ImageColorModes
                                    dark_src="/images/github-logo-light.svg"
                                    light_src="/images/github-logo.svg"
                                    alt="GitHub"
                                    width=100
                                />
                            </a>
                        </div>
                    </BColumn>

                    <BColumn is="narrow has-text-right">
                        <BDropdown
                            class="mb-4"
                            is_right=true
                            is_up=true
                            trigger=move || {
                                view! { <span class="has-text-weight-bold">{t!(i18n, change_language)} " ▲"</span> }
                            }
                        >

                            <BDropdownItem on:click=move |_| i18n.set_locale(Locale::en)>"English"</BDropdownItem>
                            <BDropdownItem on:click=move |_| i18n.set_locale(Locale::es)>"Español"</BDropdownItem>
                        </BDropdown>

                        <BButtons has_addons=true>
                            <For
                                each=color_mode_options
                                key=|(_, _, cmode)| cmode.clone()
                                children=move |(title, icon, cmode)| {
                                    let mode_clone = cmode.clone();
                                    let assign_button_state = move || {
                                        let color_mode = mode.get();
                                        if color_mode == mode_clone { BState::Active } else { BState::Default }
                                    };
                                    let button_state = RwSignal::new(assign_button_state());
                                    Effect::new(move |_| {
                                        button_state.set(assign_button_state());
                                    });
                                    view! {
                                        <BButton
                                            title=title
                                            on:click=move |_| set_mode.set(cmode.clone())
                                            state=button_state
                                        >
                                            <BIcon is_scaled=false icon=icon />
                                        </BButton>
                                    }
                                }
                            />
                        </BButtons>
                    </BColumn>
                </BColumns>
            </div>
        </footer>
    }
}

  /*


*/

use icondata_core::IconData;
use leptos_use::{
    use_color_mode_with_options, use_cookie_with_options, use_debounce_fn, use_event_listener,
    use_interval, use_intl_number_format, use_locales, use_preferred_dark, use_timestamp,
    use_window, ColorMode, UseColorModeOptions, UseCookieOptions,
    UseIntervalReturn, UseIntlNumberFormatOptions,
};

#[component]
fn ImageColorModes(
    dark_src: &'static str,
    light_src: &'static str,
    alt: &'static str,
    width: i8,
) -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } = use_app_color_mode();
    view! {
        <picture>

            <img
                src=move || { match mode.get(){
                    ColorMode::Dark => dark_src,
                    ColorMode::Light => light_src,
                    _ => dark_src
                }}
                alt=alt
                width=width
            />
        </picture>
    }
}


fn color_mode_options() -> [(&'static str, &'static IconData, ColorMode); 3] {
    [
        ("System theme", icondata_fa::FaDesktopSolid, ColorMode::Auto),
        ("Light theme", icondata_fa::FaSunSolid, ColorMode::Light),
        ("Dark theme", icondata_fa::FaMoonSolid, ColorMode::Dark),
    ]
}

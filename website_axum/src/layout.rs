use icondata_core::IconData;
use leptos::*;
use leptos_bulma::columns::{BColumn, BColumns};
use leptos_bulma::components::{
    BDropdown, BDropdownItem, BNavbar, BNavbarBrand, BNavbarBurger, BNavbarEnd, BNavbarItem,
    BNavbarItemDropdown, BNavbarMenu, BNavbarStart,
};
use leptos_bulma::elements::{BButton, BButtons, BIcon};
use leptos_bulma::enums::{BSize, BState};
use leptos_bulma::icons::icondata_fa;
use leptos_use::{
    use_color_mode_with_options, use_cookie_with_options, use_debounce_fn, use_event_listener,
    use_interval, use_intl_number_format, use_locales, use_preferred_dark, use_timestamp,
    use_window, ColorMode, UseColorModeOptions, UseColorModeReturn, UseCookieOptions,
    UseIntervalReturn, UseIntlNumberFormatOptions,
};
use leptos::prelude::*;

use crate::app::use_app_color_mode;
use crate::i18n::{t, use_i18n, Locale};

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let i18n = use_i18n();
    let UseColorModeReturn { mode, set_mode, .. } = use_app_color_mode();
    let burger_is_active =  RwSignal::new(false);

    view! {
        // add syntax highlighting here:
        //<script src="/highlight/highlight.min.js"></script>
        //<link rel="stylesheet" href= move ||{if mode.get()==ColorMode::Light{
        //    "/highlight/styles/1c-light.min.css"
        //}else{
        //    "/highlight/styles/dark.min.css"
        //}}/>
        <BNavbar class="has-shadow">
            <BNavbarBrand>
                <BNavbarItem class="media mb-0 is-align-items-center" href="/">
                    <div class="media-left">
                        <figure class="image is-48x48">
                            <img class="is-rounded" src="/images/favicon.png" />
                        </figure>
                    </div>
                    <div class="media-content">
                        <div class="title is-5">"Leptos Bulma"</div>
                    </div>
                </BNavbarItem>

                <BNavbarBurger is_active=burger_is_active />
            </BNavbarBrand>

            <BNavbarMenu is_active=burger_is_active>
                <BNavbarStart>
                    <BNavbarItem href="/">{t!(i18n, home)}</BNavbarItem>
                    <BNavbarItemDropdown is_hoverable=true href="/guides" trigger=move || t!(i18n, guides)>
                        <BNavbarItem href="/guides#how-to-install-ssr">
                            {t!(i18n, how_to_install_in_leptos_ssr)}
                        </BNavbarItem>
                        <BNavbarItem href="/guides#how-to-install-csr">
                            {t!(i18n, how_to_install_in_leptos_csr)}
                        </BNavbarItem>
                        <BNavbarItem href="/guides#customization">{t!(i18n, customization)}</BNavbarItem>
                    </BNavbarItemDropdown>
                    <BNavbarItemDropdown is_hoverable=true href="/elements" trigger=move || t!(i18n, elements)>
                        <BNavbarItem href="/elements/button">{t!(i18n, button)}</BNavbarItem>
                        <BNavbarItem href="/elements/icon">{t!(i18n, icon)}</BNavbarItem>
                        <BNavbarItem href="/elements/notification">{t!(i18n, notification)}</BNavbarItem>
                        <BNavbarItem href="/elements/progress">{t!(i18n, progress)}</BNavbarItem>
                        <BNavbarItem href="/elements/tag">{t!(i18n, tag)}</BNavbarItem>
                    </BNavbarItemDropdown>
                    <BNavbarItemDropdown is_hoverable=true href="/components" trigger=move || t!(i18n, components)>
                        <BNavbarItem href="/components/breadcrumb">{t!(i18n, breadcrumb)}</BNavbarItem>
                    </BNavbarItemDropdown>
                    <BNavbarItem href="/form">{t!(i18n, form)}</BNavbarItem>
                    <BNavbarItem href="/columns">{t!(i18n, columns)}</BNavbarItem>
                    <BNavbarItem href="/layout">{t!(i18n, layout)}</BNavbarItem>
                </BNavbarStart>

                <BNavbarEnd>
                    <BNavbarItem href="https://github.com/javierEd/leptos-bulma" target="_blank" title="GitHub">
                        <BIcon icon=icondata_fa::FaGithubBrands size=BSize::Large is_scaled=true />
                    </BNavbarItem>
                </BNavbarEnd>
            </BNavbarMenu>
        </BNavbar>

        <main class="container">
            <div class="m-5">{children()}</div>
        </main>
    }
}

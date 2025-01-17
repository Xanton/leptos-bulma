use leptos::*;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{GeneralForm, RustCodeExample};
use crate::i18n::{t, use_i18n, t_string};
use leptos::prelude::*;

#[component]
pub fn FormPage() -> impl IntoView {
    let i18n = use_i18n();

    let title = t_string!(i18n, form);
    view! {
        <PageTitle text={title}/>
        //<PageTitle text=t!(i18n, form)/>

        <h2 class="title is-3">{t!(i18n, form)}</h2>

        <section class="section">
            <h3 class="title is-4">"General"</h3>

            <p class="block">"Example:"</p>

            <RustCodeExample name="general_form"/>

            <p class="block">"See it in action:"</p>

            <GeneralForm/>

            <GoToDocsRs path="form/index"/>
        </section>

        <GoToBulmaIo path="form"/>
    }
}

use leptos::*;
use leptos::prelude::*;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{BasicColumns, RustCodeExample};
use crate::i18n::{t, use_i18n,t_string};

#[component]
pub fn ColumnsPage() -> impl IntoView {
    let i18n = use_i18n();

    let title = t_string!(i18n, columns);
    view! {
        <PageTitle text={title}/>
        //<PageTitle text=t!(i18n, columns)/>

        <h2 class="title is-3">{t!(i18n, columns)}</h2>

        <section class="section">
            <h3 class="title is-4">"Basic"</h3>

            <p class="block">"Example:"</p>

            <RustCodeExample name="basic_columns"/>

            <p class="block">"See it in action:"</p>

            <BasicColumns/>

            <GoToDocsRs path="columns/fn.BColumns"/>
        </section>

        <GoToBulmaIo path="columns"/>
    }
}

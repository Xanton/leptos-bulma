use leptos::*;
use leptos_bulma::elements::{BBlock, BTitle};
use leptos_bulma::layout::BSection;

use crate::components::{GoToBulmaIo, GoToDocsRs, PageTitle};
use crate::examples::{BasicSection, RustCodeExample};
use crate::i18n::{t, use_i18n, t_string};
//use leptos::prelude::*;

#[component]
pub fn LayoutPage() -> impl IntoView {
    let i18n = use_i18n();

    let title = t_string!(i18n, layout);
    view! {
        <PageTitle text={title}/>

        <BTitle is=3>{t!(i18n, layout)}</BTitle>

        <BSection>
            <BTitle is=4>"Section"</BTitle>

            <BBlock>"Example:"</BBlock>

            <RustCodeExample name="basic_section"/>

            <BBlock>"See it in action:"</BBlock>

            <BasicSection/>

            <GoToDocsRs path="layout/fn.BSection"/>
        </BSection>

        <GoToBulmaIo path="layout"/>
    }
}

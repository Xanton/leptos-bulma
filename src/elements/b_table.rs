use std::hash::Hash;

use leptos::*;
use leptos::html::*;
use leptos::prelude::*;
use leptos::text_prop::TextProp;

#[component]
pub fn BTable(#[prop(optional, into)] class: TextProp, children: Children) -> impl IntoView {
    view! { <table class=format!("table {}", class.get())>{children()}</table> }
}

#[component]
pub fn BTbody<IF, I, T, EF, N, KF, K>(
    #[prop(optional, into)] class: TextProp,
    each_row: IF,
    key: KF,
    children: EF,
) -> impl IntoView
where
    IF: Fn() -> I + 'static + std::marker::Send,
    I: IntoIterator<Item = T> + std::marker::Send + 'static,
    EF: Fn(T) -> N + 'static + std::marker::Send + std::clone::Clone,
    N: IntoView + 'static,
    KF: Fn(&T) -> K + 'static + std::marker::Send + std::clone::Clone,
    K: Eq + Hash + 'static,
    T: 'static + std::marker::Send,
{
    view! {
        <tbody class=class.get()>
            <For each=each_row key=key let:data>
                <tr>{children(data)}</tr>
            </For>
        </tbody>
    }
}

#[component]
pub fn BTfoot<IF, I, T, EF, N, KF, K>(
    #[prop(optional, into)] class: TextProp,
    each_cell: IF,
    key: KF,
    children: EF,
) -> impl IntoView
where
    IF: Fn() -> I + 'static,
    I: IntoIterator<Item = T> + 'static,
    EF: Fn(T) -> N + 'static + std::marker::Send + std::clone::Clone,
    N: IntoView + 'static,
    KF: Fn(&T) -> K + 'static + std::clone::Clone + std::marker::Send,
    K: Eq + Hash + 'static,
    T: 'static, I: Send, IF: Send, T: Send
{
    view! {
        <tfoot class=class.get()>
            <tr>
                <For each=each_cell key=key let:data>

                    <th>{children(data)}</th>
                </For>
            </tr>
        </tfoot>
    }
}

#[component]
pub fn BThead<IF, I, T, EF, N, KF, K>(
    #[prop(optional, into)] class: TextProp,
    each_cell: IF,
    key: KF,
    children: EF,
) -> impl IntoView
where
    IF: Fn() -> I + 'static + std::marker::Send,
    I: IntoIterator<Item = T> + std::marker::Send + 'static,
    EF: Fn(T) -> N + 'static + std::marker::Send + std::clone::Clone,
    N: IntoView + 'static,
    KF: Fn(&T) -> K + 'static + std::clone::Clone + std::marker::Send,
    K: Eq + Hash + 'static,
    T: 'static + std::marker::Send, KF: Send
{
    view! {
        <thead class=class.get()>
            <tr>
                <For each=each_cell key=key let:data>

                    <th>{children(data)}</th>
                </For>
            </tr>
        </thead>
    }
}

use leptos::ev::SubmitEvent;
use leptos::html::{Input, Select};
use leptos::*;
use leptos_bulma::elements::BButton;
use leptos_bulma::form::{
    BCheckboxField, BFile, BPasswordField, BSelectField, BTextField, BTextareaField,
};
use leptos::prelude::*;

fn validate_required(value: Option<String>, error: RwSignal<Option<String>>) -> bool {
    let value_is_empty = value.unwrap_or_default().trim().is_empty();

    error.set(if value_is_empty {
        Some("Can't be blank".to_owned())
    } else {
        None
    });

    !value_is_empty
}

fn validate_required_checkbox(node_ref: NodeRef<Input>, error: RwSignal<Option<String>>) -> bool {
    validate_required(
        node_ref.get().and_then(|el| {
            if el.checked() {
                Some("checked".to_owned())
            } else {
                None
            }
        }),
        error,
    )
}

fn validate_required_input(node_ref: NodeRef<Input>, error: RwSignal<Option<String>>) -> bool {
    validate_required(node_ref.get().map(|el| el.value()), error)
}

fn validate_required_select(node_ref: NodeRef<Select>, error: RwSignal<Option<String>>) -> bool {
    validate_required(node_ref.get().map(|el| el.value()), error)
}

#[component]
pub fn GeneralForm() -> impl IntoView {
    let node_ref_username:NodeRef<Input> = NodeRef::new();
    let node_ref_first_name:NodeRef<Input> = NodeRef::new();
    let node_ref_slug:NodeRef<Input> = NodeRef::new();
    let node_ref_email:NodeRef<Input> = NodeRef::new();
    let node_ref_password:NodeRef<Input> = NodeRef::new();
    let node_ref_country:NodeRef<Select> = NodeRef::new();
    let node_ref_accept_terms:NodeRef<Input> = NodeRef::new();
    let error_username = RwSignal::new(None);
    let error_first_name = RwSignal::new(None);
    let error_slug = RwSignal::new(None);
    let error_email = RwSignal::new(None);
    let error_password = RwSignal::new(None);
    let error_country = RwSignal::new(None);
    let error_accept_terms = RwSignal::new(None);
    let value_slug = RwSignal::new("".to_owned());

    let first_name_on_input = move |event| {
        value_slug.set(slug::slugify(event_target_value(&event)));
        validate_required_input(node_ref_first_name, error_first_name);
    };

    let form_on_submit = move |event: SubmitEvent| {
        event.prevent_default();

        let is_valid = validate_required_input(node_ref_username, error_username)
            & validate_required_input(node_ref_first_name, error_first_name)
            & validate_required_input(node_ref_slug, error_slug)
            & validate_required_input(node_ref_email, error_email)
            & validate_required_input(node_ref_password, error_password)
            & validate_required_select(node_ref_country, error_country)
            & validate_required_checkbox(node_ref_accept_terms, error_accept_terms);

        if is_valid {
            let _ = window().alert_with_message("Just an example, nothing is sent.");
        }
    };

    view! {
        <form on:submit=form_on_submit class="block" autocomplete="off" novalidate="true">
            <BTextField
                node_ref=node_ref_username
                label="* Username"
                error=error_username
                addon_left=move || view! { <a class="button is-static">"@"</a> }
                on_input=move |_| {
                    validate_required_input(node_ref_username, error_username);
                }
            />

            <BTextField
                node_ref=node_ref_first_name
                label="* First name"
                error=error_first_name
                on_input=first_name_on_input
            />

            <BTextField
                node_ref=node_ref_slug
                label="* Slug"
                value=value_slug
                error=error_slug
                on_input=move |_| {
                    validate_required_input(node_ref_slug, error_slug);
                }
            />

            <BTextField
                node_ref=node_ref_email
                label="* Email"
                input_type="email"
                error=error_email
                on_input=move |_| {
                    validate_required_input(node_ref_email, error_email);
                }
            />

            <BPasswordField
                node_ref=node_ref_password
                label="* Password"
                error=error_password
                on_input=move |_| {
                    validate_required_input(node_ref_password, error_password);
                }
            />

            <BTextareaField label="Bio"/>
            <BSelectField
                node_ref=node_ref_country
                label="* Country"
                options=vec![
                    ("Select".to_owned(), "".to_owned()),
                    ("Agentina".to_owned(), "AR".to_owned()),
                    ("Panamá".to_owned(), "PA".to_owned()),
                    ("Trinidad and Tobago".to_owned(), "TT".to_owned()),
                    ("United States".to_owned(), "US".to_owned()),
                    ("Venezuela".to_owned(), "VE".to_owned()),
                ]

                error=error_country
                on_change=move |_| {
                    validate_required_select(node_ref_country, error_country);
                }
            />

            <BFile label="Avatar image" accept="image/gif,image/jpeg,image/png"/>
            <BCheckboxField
                node_ref=node_ref_accept_terms
                label="I accept the terms and conditions"
                error=error_accept_terms
                on_change=move |_| {
                    validate_required_checkbox(node_ref_accept_terms, error_accept_terms);
                }
            />

            <BButton class="is-primary">"Submit"</BButton>
        </form>
    }
}

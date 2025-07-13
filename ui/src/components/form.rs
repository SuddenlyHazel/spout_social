use dioxus::prelude::*;

const FORM_CSS: Asset = asset!("/assets/styling/components/form.css");

#[component]
pub fn Form(onsubmit: Option<EventHandler<FormEvent>>, children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FORM_CSS }

        form {
            class: "form",
            onsubmit: move |evt| {
                evt.prevent_default();
                if let Some(handler) = onsubmit {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn FormGroup(children: Element) -> Element {
    rsx! {
        div {
            class: "form-group",
            {children}
        }
    }
}

#[component]
pub fn Label(r#for: Option<String>, required: Option<bool>, children: Element) -> Element {
    let required = required.unwrap_or(false);
    let class = if required {
        "form-label form-label--required"
    } else {
        "form-label"
    };

    rsx! {
        label {
            r#for: r#for,
            class: "{class}",
            {children}
        }
    }
}

#[component]
pub fn Input(
    id: Option<String>,
    r#type: Option<String>,
    placeholder: Option<String>,
    value: Option<String>,
    maxlength: Option<String>,
    accept: Option<String>,
    disabled: Option<bool>,
    error: Option<bool>,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
) -> Element {
    let input_type = r#type.unwrap_or_else(|| "text".to_string());
    let disabled = disabled.unwrap_or(false);
    let error = error.unwrap_or(false);

    let mut class = "form-input".to_string();
    if error {
        class.push_str(" form-input--error");
    }

    rsx! {
        input {
            id: id,
            class: "{class}",
            r#type: "{input_type}",
            placeholder: placeholder,
            value: value,
            maxlength: maxlength,
            accept: accept,
            disabled: disabled,
            oninput: move |evt| {
                if let Some(handler) = oninput {
                    handler.call(evt);
                }
            },
            onchange: move |evt| {
                if let Some(handler) = onchange {
                    handler.call(evt);
                }
            },
        }
    }
}

#[component]
pub fn Textarea(
    id: Option<String>,
    placeholder: Option<String>,
    value: Option<String>,
    maxlength: Option<String>,
    rows: Option<String>,
    disabled: Option<bool>,
    error: Option<bool>,
    oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let disabled = disabled.unwrap_or(false);
    let error = error.unwrap_or(false);

    let mut class = "form-textarea".to_string();
    if error {
        class.push_str(" form-textarea--error");
    }

    rsx! {
        textarea {
            id: id,
            class: "{class}",
            placeholder: placeholder,
            value: value,
            maxlength: maxlength,
            rows: rows,
            disabled: disabled,
            oninput: move |evt| {
                if let Some(handler) = oninput {
                    handler.call(evt);
                }
            },
        }
    }
}

#[component]
pub fn Hint(children: Element) -> Element {
    rsx! {
        div {
            class: "form-hint",
            {children}
        }
    }
}

#[component]
pub fn FormError(children: Element) -> Element {
    rsx! {
        div {
            class: "form-error",
            {children}
        }
    }
}

#[component]
pub fn FormActions(children: Element) -> Element {
    rsx! {
        div {
            class: "form-actions",
            {children}
        }
    }
}

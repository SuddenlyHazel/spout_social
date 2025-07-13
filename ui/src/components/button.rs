use dioxus::prelude::*;

const BUTTON_CSS: Asset = asset!("/assets/styling/components/button.css");

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
    Success,
    Info,
    Purple,
    Nav,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[component]
pub fn Button(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    disabled: Option<bool>,
    loading: Option<bool>,
    full_width: Option<bool>,
    flex: Option<String>, // "1", "auto", etc.
    active: Option<bool>, // for nav buttons
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or(ButtonVariant::Primary);
    let size = size.unwrap_or(ButtonSize::Medium);
    let disabled = disabled.unwrap_or(false);
    let loading = loading.unwrap_or(false);
    let full_width = full_width.unwrap_or(false);
    let active = active.unwrap_or(false);

    let mut class_list = vec!["btn".to_string()];

    // Add variant class
    match variant {
        ButtonVariant::Primary => class_list.push("btn--primary".to_string()),
        ButtonVariant::Secondary => class_list.push("btn--secondary".to_string()),
        ButtonVariant::Outline => class_list.push("btn--outline".to_string()),
        ButtonVariant::Ghost => class_list.push("btn--ghost".to_string()),
        ButtonVariant::Success => class_list.push("btn--success".to_string()),
        ButtonVariant::Info => class_list.push("btn--info".to_string()),
        ButtonVariant::Purple => class_list.push("btn--purple".to_string()),
        ButtonVariant::Nav => {
            class_list.push("btn--nav".to_string());
            if active {
                class_list.push("btn--nav-active".to_string());
            }
        }
    }

    // Add size class
    match size {
        ButtonSize::Small => class_list.push("btn--sm".to_string()),
        ButtonSize::Medium => {} // default, no class needed
        ButtonSize::Large => class_list.push("btn--lg".to_string()),
        ButtonSize::ExtraLarge => class_list.push("btn--xl".to_string()),
    }

    // Add modifier classes
    if full_width {
        class_list.push("btn--full".to_string());
    }

    if loading {
        class_list.push("btn--loading".to_string());
    }

    if let Some(flex_value) = flex {
        match flex_value.as_str() {
            "1" => class_list.push("btn--flex-1".to_string()),
            "auto" => class_list.push("btn--flex-auto".to_string()),
            _ => {}
        }
    }

    let class_string = class_list.join(" ");

    rsx! {
        document::Link { rel: "stylesheet", href: BUTTON_CSS }

        button {
            class: "{class_string}",
            disabled: disabled || loading,
            onclick: move |evt| {
                if let Some(handler) = onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}

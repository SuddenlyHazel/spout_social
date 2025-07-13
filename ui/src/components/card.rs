use dioxus::prelude::*;

const CARD_CSS: Asset = asset!("/assets/styling/components/card.css");

#[derive(Clone, PartialEq)]
pub enum CardVariant {
    Default,
    Post,
    Group,
    Dm,
    Info,
    Fingerprint,
    Dashed,
}

#[component]
pub fn Card(
    variant: Option<CardVariant>,
    interactive: Option<bool>,
    compact: Option<bool>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or(CardVariant::Default);
    let interactive = interactive.unwrap_or(false);
    let compact = compact.unwrap_or(false);

    let mut class_list = vec!["card".to_string()];

    // Add variant class
    match variant {
        CardVariant::Default => {}
        CardVariant::Post => class_list.push("card--post".to_string()),
        CardVariant::Group => class_list.push("card--group".to_string()),
        CardVariant::Dm => class_list.push("card--dm".to_string()),
        CardVariant::Info => class_list.push("card--info".to_string()),
        CardVariant::Fingerprint => class_list.push("card--fingerprint".to_string()),
        CardVariant::Dashed => class_list.push("card--dashed".to_string()),
    }

    // Add modifier classes
    if interactive {
        class_list.push("card--interactive".to_string());
    }

    if compact {
        class_list.push("card--compact".to_string());
    }

    let class_string = class_list.join(" ");

    rsx! {
        document::Link { rel: "stylesheet", href: CARD_CSS }

        div {
            class: "{class_string}",
            onclick: move |evt| {
                if let Some(handler) = onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}

#[component]
pub fn CardHeader(children: Element) -> Element {
    rsx! {
        div {
            class: "card-header",
            {children}
        }
    }
}

#[component]
pub fn CardHeaderDm(children: Element) -> Element {
    rsx! {
        div {
            class: "card-header--dm",
            {children}
        }
    }
}

#[component]
pub fn CardContent(children: Element) -> Element {
    rsx! {
        div {
            class: "card-content",
            {children}
        }
    }
}

#[component]
pub fn CardActions(children: Element) -> Element {
    rsx! {
        div {
            class: "card-actions",
            {children}
        }
    }
}

#[component]
pub fn CardFooter(children: Element) -> Element {
    rsx! {
        div {
            class: "card-footer",
            {children}
        }
    }
}

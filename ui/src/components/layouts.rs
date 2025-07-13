use crate::components::utils::ClassBuilder;
use dioxus::prelude::*;

const BASE_CSS: Asset = asset!("/assets/styling/base.css");
const TOKENS_CSS: Asset = asset!("/assets/styling/tokens.css");

#[component]
pub fn OnboardingLayout(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BASE_CSS }
        document::Link { rel: "stylesheet", href: TOKENS_CSS }

        div {
            class: "onboarding-layout",
            style: "
                margin: 0;
                padding: var(--space-40) var(--space-20);
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                min-height: 100vh;
                background-color: var(--color-black);
                color: var(--color-text-primary);
                font-family: var(--font-family-sans);
                font-size: var(--font-size-base);
                line-height: var(--line-height-normal);
            ",
            {children}
        }
    }
}

#[component]
pub fn OnboardingSection(
    max_width: Option<String>,
    centered: Option<bool>,
    children: Element,
) -> Element {
    let max_width = max_width.unwrap_or_else(|| "var(--container-sm)".to_string());
    let centered = centered.unwrap_or(true);

    let class = ClassBuilder::new("onboarding-section")
        .add_if("onboarding-section--centered", centered)
        .build();

    let align_items = if centered { "center" } else { "flex-start" };
    let text_align = if centered { "center" } else { "left" };

    rsx! {
        div {
            class: "{class}",
            style: "
                width: 100%;
                max-width: {max_width};
                display: flex;
                flex-direction: column;
                align-items: {align_items};
                text-align: {text_align};
            ",
            {children}
        }
    }
}

#[component]
pub fn OnboardingHeader(
    title: String,
    subtitle: Option<String>,
    gradient: Option<String>,
    children: Option<Element>,
) -> Element {
    let gradient = gradient.unwrap_or_else(|| "var(--gradient-primary)".to_string());

    rsx! {
        div {
            class: "onboarding-header",
            style: "margin-bottom: var(--space-40);",
            h1 {
                class: "onboarding-title",
                style: "
                    font-size: var(--font-size-9xl);
                    font-weight: var(--font-weight-bold);
                    margin: 0 0 var(--space-10) 0;
                    text-align: center;
                    background: {gradient};
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                ",
                "{title}"
            }
            if let Some(subtitle_text) = subtitle {
                p {
                    class: "onboarding-subtitle",
                    style: "
                        font-size: var(--font-size-3xl);
                        line-height: var(--line-height-loose);
                        color: var(--color-text-muted);
                        text-align: center;
                        margin: 0 0 var(--space-32) 0;
                    ",
                    "{subtitle_text}"
                }
            }
            if let Some(extra_content) = children {
                {extra_content}
            }
        }
    }
}

#[component]
pub fn OnboardingActions(children: Element) -> Element {
    rsx! {
        div {
            class: "onboarding-actions",
            style: "
                display: flex;
                flex-direction: column;
                gap: var(--space-12);
                width: 100%;
                max-width: 400px;
            ",
            {children}
        }
    }
}

#[component]
pub fn AppLayout(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BASE_CSS }
        document::Link { rel: "stylesheet", href: TOKENS_CSS }

        div {
            class: "app-layout",
            style: "
                display: flex;
                flex-direction: column;
                height: 100vh;
                background-color: var(--color-light-bg);
                font-family: var(--font-family-sans);
                font-size: var(--font-size-base);
                line-height: var(--line-height-normal);
            ",
            {children}
        }
    }
}

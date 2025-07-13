use dioxus::prelude::*;

const BASE_CSS: Asset = asset!("/assets/styling/base.css");

#[component]
pub fn OnboardingLayout(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BASE_CSS }

        div {
            style: "
                margin: 0;
                padding: 40px 20px;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                min-height: 100vh;
                background-color: #0f1116;
                color: white;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                font-size: 14px;
                line-height: 1.4;
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
    let max_width = max_width.unwrap_or_else(|| "600px".to_string());
    let centered = centered.unwrap_or(true);

    let align_items = if centered { "center" } else { "flex-start" };
    let text_align = if centered { "center" } else { "left" };

    rsx! {
        div {
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
    let gradient =
        gradient.unwrap_or_else(|| "linear-gradient(135deg, #3cc4dc, #00a8d6)".to_string());

    rsx! {
        div {
            style: "margin-bottom: 40px;",
            h1 {
                style: "
                    font-size: 2.5rem;
                    font-weight: 700;
                    margin: 0 0 10px 0;
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
                    style: "
                        font-size: 1.1rem;
                        line-height: 1.6;
                        color: #b0b0b0;
                        text-align: center;
                        margin: 0 0 30px 0;
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
            {children}
        }
    }
}

#[component]
pub fn OnboardingButton(
    variant: String, // "primary" or "secondary"
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let class_name = match variant.as_str() {
        "primary" => "onboarding-button onboarding-button--primary",
        "secondary" => "onboarding-button onboarding-button--secondary",
        _ => "onboarding-button onboarding-button--primary",
    };

    rsx! {
        button {
            class: "{class_name}",
            onclick: move |evt| onclick.call(evt),
            {children}
        }
    }
}

#[component]
pub fn AppLayout(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BASE_CSS }

        div {
            style: "
                display: flex;
                flex-direction: column;
                height: 100vh;
                background-color: #f8fafc;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                font-size: 14px;
                line-height: 1.4;
            ",
            {children}
        }
    }
}

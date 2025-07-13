use crate::{
    Button, ButtonVariant, OnboardingHeader, OnboardingLayout, OnboardingSection, PageInfo,
};
use dioxus::prelude::*;

#[component]
pub fn ImportHandle(on_back: EventHandler<()>) -> Element {
    rsx! {
        OnboardingLayout {
            OnboardingSection {
                max_width: Some("500px".to_string()),
                OnboardingHeader {
                    title: "Import Handle".to_string(),
                    subtitle: Some("Bring your existing social identity into Spout Social".to_string()),
                }

                PageInfo {
                    div {
                        style: "
                            text-align: center;
                            padding: var(--space-32);
                            background-color: rgba(255, 255, 255, 0.05);
                            border-radius: var(--radius-lg);
                            margin-bottom: var(--space-24);
                        ",
                        div {
                            style: "
                                font-size: var(--font-size-6xl);
                                margin-bottom: var(--space-16);
                            ",
                            "🚧"
                        }
                        h3 {
                            style: "
                                color: var(--color-text-primary);
                                margin-bottom: var(--space-12);
                                font-size: var(--font-size-4xl);
                            ",
                            "This feature is coming soon!"
                        }
                        p {
                            style: "
                                color: var(--color-text-muted);
                                line-height: var(--line-height-relaxed);
                                max-width: 400px;
                                margin: 0 auto;
                            ",
                            "Handle importing will allow you to bring your existing social identity into Spout Social. "
                            "This feature is currently in development."
                        }
                    }
                }

                Button {
                    variant: ButtonVariant::Secondary,
                    full_width: true,
                    onclick: move |_| on_back.call(()),
                    "← Back to Home"
                }
            }
        }
    }
}

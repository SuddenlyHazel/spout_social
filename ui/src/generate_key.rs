use crate::{
    Button, ButtonVariant, Card, CardContent, CardHeader, CardVariant, Page, PageContent,
    PageHeader, PageHeaderStyle, PageSection,
};
use dioxus::prelude::*;

#[component]
pub fn GenerateKey(
    name: String,
    handle: String,
    image_file: Option<String>,
    bio: String,
    handle_id: String,
    key_value: String,
    fingerprint: String,
    is_generating: bool,
    on_back: EventHandler<()>,
    on_regenerate: EventHandler<()>,
    on_accept: EventHandler<()>,
) -> Element {
    rsx! {
        Page {
            PageSection {
                PageHeader {
                    style: PageHeaderStyle::Primary,
                    title: "Your Unique Identity".to_string(),
                    subtitle: Some("This is your cryptographic fingerprint - a unique visual representation of your identity key.".to_string()),
                }

                PageContent {
                    div {
                        style: "
                            display: flex;
                            flex-direction: column;
                            gap: var(--space-8);
                            width: 100%;
                            max-width: var(--container-md);
                        ",

                        Card {
                            variant: CardVariant::Fingerprint,
                            CardHeader {
                                h3 {
                                    style: "
                                        font-size: var(--font-size-4xl);
                                        font-weight: var(--font-weight-semibold);
                                        color: var(--color-text-primary);
                                        margin-bottom: var(--space-4);
                                        text-align: center;
                                    ",
                                    "Visual Fingerprint"
                                }
                            }
                            CardContent {
                                pre {
                                    style: "
                                        font-family: var(--font-family-mono);
                                        font-size: var(--font-size-sm);
                                        color: var(--color-primary-start);
                                        text-align: center;
                                        margin: var(--space-6) 0;
                                        line-height: var(--line-height-snug);
                                        white-space: pre;
                                    ",
                                    "{fingerprint}"
                                }
                                div {
                                    style: "
                                        font-size: var(--font-size-md);
                                        color: var(--color-text-muted);
                                        text-align: center;
                                        line-height: var(--line-height-relaxed);
                                    ",
                                    "This pattern is unique to your key and can help you verify your identity."
                                }
                            }
                        }

                        Card {
                            variant: CardVariant::Info,
                            CardHeader {
                                h3 {
                                    style: "
                                        font-size: var(--font-size-4xl);
                                        font-weight: var(--font-weight-semibold);
                                        color: var(--color-text-primary);
                                        margin-bottom: var(--space-4);
                                    ",
                                    "Handle Details"
                                }
                            }
                            CardContent {
                                div {
                                    style: "display: flex; flex-direction: column; gap: var(--space-4);",

                                    div {
                                        style: "
                                            display: flex;
                                            justify-content: space-between;
                                            align-items: center;
                                            padding: var(--space-2) 0;
                                        ",
                                        span {
                                            style: "
                                                font-weight: var(--font-weight-medium);
                                                color: var(--color-text-secondary);
                                            ",
                                            "Username:"
                                        }
                                        span {
                                            style: "
                                                font-family: var(--font-family-mono);
                                                color: var(--color-text-primary);
                                            ",
                                            "@{handle}"
                                        }
                                    }

                                    div {
                                        style: "
                                            display: flex;
                                            justify-content: space-between;
                                            align-items: center;
                                            padding: var(--space-2) 0;
                                        ",
                                        span {
                                            style: "
                                                font-weight: var(--font-weight-medium);
                                                color: var(--color-text-secondary);
                                            ",
                                            "Display Name:"
                                        }
                                        span {
                                            style: "color: var(--color-text-primary);",
                                            "{name}"
                                        }
                                    }

                                    if !bio.is_empty() {
                                        div {
                                            style: "
                                                display: flex;
                                                justify-content: space-between;
                                                align-items: flex-start;
                                                padding: var(--space-2) 0;
                                            ",
                                            span {
                                                style: "
                                                    font-weight: var(--font-weight-medium);
                                                    color: var(--color-text-secondary);
                                                    margin-right: var(--space-4);
                                                ",
                                                "Bio:"
                                            }
                                            span {
                                                style: "
                                                    color: var(--color-text-primary);
                                                    text-align: right;
                                                    flex: 1;
                                                ",
                                                "{bio}"
                                            }
                                        }
                                    }

                                    div {
                                        style: "
                                            display: flex;
                                            justify-content: space-between;
                                            align-items: center;
                                            padding: var(--space-2) 0;
                                        ",
                                        span {
                                            style: "
                                                font-weight: var(--font-weight-medium);
                                                color: var(--color-text-secondary);
                                            ",
                                            "Handle ID:"
                                        }
                                        span {
                                            style: "
                                                font-family: var(--font-family-mono);
                                                color: var(--color-text-primary);
                                                font-size: var(--font-size-sm);
                                            ",
                                            "{handle_id}"
                                        }
                                    }

                                    div {
                                        style: "
                                            display: flex;
                                            justify-content: space-between;
                                            align-items: center;
                                            padding: var(--space-2) 0;
                                        ",
                                        span {
                                            style: "
                                                font-weight: var(--font-weight-medium);
                                                color: var(--color-text-secondary);
                                            ",
                                            "Key Value:"
                                        }
                                        span {
                                            style: "
                                                font-family: var(--font-family-mono);
                                                color: var(--color-text-primary);
                                                font-size: var(--font-size-sm);
                                                word-break: break-all;
                                                max-width: 200px;
                                                text-align: right;
                                            ",
                                            "{key_value}"
                                        }
                                    }
                                }
                            }
                        }

                        div {
                            style: "
                                background-color: var(--color-dark-bg);
                                border: 1px solid var(--color-border-medium);
                                border-radius: var(--radius-lg);
                                padding: var(--space-4) var(--space-5);
                                text-align: center;
                                color: var(--color-text-muted);
                                font-size: var(--font-size-lg);
                                line-height: var(--line-height-relaxed);
                            ",
                            "💡 Don't like this pattern? You can generate a new one!"
                            br {}
                            "Each key is completely unique and secure."
                        }

                        div {
                            style: "
                                display: flex;
                                gap: var(--space-4);
                                width: 100%;
                                flex-wrap: wrap;
                            ",

                            Button {
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| on_back.call(()),
                                "← Back to Profile"
                            }

                            Button {
                                variant: ButtonVariant::Secondary,
                                disabled: Some(is_generating),
                                loading: Some(is_generating),
                                onclick: move |_| on_regenerate.call(()),
                                if is_generating {
                                    "🔄 Generating..."
                                } else {
                                    "🎲 Generate New Key"
                                }
                            }

                            Button {
                                variant: ButtonVariant::Primary,
                                disabled: Some(is_generating),
                                onclick: move |_| on_accept.call(()),
                                "✨ Accept This Identity"
                            }
                        }
                    }
                }
            }
        }
    }
}

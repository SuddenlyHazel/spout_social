use crate::{CreateHandle, ImportHandle};
use dioxus::prelude::*;

const LANDING_CSS: Asset = asset!("/assets/styling/landing.css");

#[component]
pub fn Landing() -> Element {
    let mut current_view = use_signal(|| "landing");

    match current_view() {
        "import" => rsx! {
            ImportHandle {
                on_back: move |_| current_view.set("landing")
            }
        },
        "create" => rsx! {
            CreateHandle {
                on_back: move |_| current_view.set("landing")
            }
        },
        _ => rsx! {
            document::Link { rel: "stylesheet", href: LANDING_CSS }

            div {
                id: "landing",

                div {
                    id: "welcome-section",
                    h1 { "Welcome to Spout Social!" }
                    p {
                        class: "subtitle",
                        "You're ready to start your social journey. Get started by creating a new handle or importing an existing one."
                    }

                    a {
                        href: "https://example.com/docs",
                        class: "docs-link",
                        "📖 How does this thing work?"
                    }
                }

                div {
                    id: "action-buttons",

                    button {
                        class: "action-btn primary",
                        onclick: move |_| {
                            current_view.set("create");
                        },
                        "🆕 Create Handle"
                    }

                    button {
                        class: "action-btn secondary",
                        onclick: move |_| {
                            current_view.set("import");
                        },
                        "📥 Import Handle"
                    }
                }
            }
        },
    }
}

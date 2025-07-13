use dioxus::prelude::*;

const IMPORT_CSS: Asset = asset!("/assets/styling/import_handle.css");

#[component]
pub fn ImportHandle(on_back: EventHandler<()>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: IMPORT_CSS }

        div {
            id: "import-handle",

            div {
                id: "import-section",
                h1 { "Import Handle" }

                div {
                    class: "placeholder-content",
                    "🚧 This feature is coming soon!"
                    p {
                        class: "placeholder-text",
                        "Handle importing will allow you to bring your existing social identity into Spout Social. "
                        "This feature is currently in development."
                    }
                }

                button {
                    class: "back-btn",
                    onclick: move |_| on_back.call(()),
                    "← Back to Home"
                }
            }
        }
    }
}

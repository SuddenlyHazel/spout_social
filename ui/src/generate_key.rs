use dioxus::prelude::*;

const GENERATE_KEY_CSS: Asset = asset!("/assets/styling/generate_key.css");

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
        document::Link { rel: "stylesheet", href: GENERATE_KEY_CSS }

        div {
            id: "generate-key",

            div {
                id: "key-section",
                h1 { "Your Unique Identity" }
                p {
                    class: "subtitle",
                    "This is your cryptographic fingerprint - a unique visual representation of your identity key."
                }

                div {
                    class: "key-container",

                    div {
                        class: "fingerprint-display",
                        h3 { "Visual Fingerprint" }
                        pre {
                            class: "fingerprint-art",
                            "{fingerprint}"
                        }
                        div {
                            class: "fingerprint-note",
                            "This pattern is unique to your key and can help you verify your identity."
                        }
                    }

                    div {
                        class: "key-info",
                        h3 { "Handle Details" }
                        div {
                            class: "info-row",
                            span { class: "label", "Username:" }
                            span { class: "value mono", "@{handle}" }
                        }
                        div {
                            class: "info-row",
                            span { class: "label", "Display Name:" }
                            span { class: "value", "{name}" }
                        }
                        if !bio.is_empty() {
                            div {
                                class: "info-row",
                                span { class: "label", "Bio:" }
                                span { class: "value", "{bio}" }
                            }
                        }
                        div {
                            class: "info-row",
                            span { class: "label", "Handle ID:" }
                            span { class: "value mono", "{handle_id}" }
                        }
                        div {
                            class: "info-row",
                            span { class: "label", "Key Value:" }
                            span { class: "value mono key-value", "{key_value}" }
                        }
                    }
                }

                div {
                    class: "generation-info",
                    "💡 Don't like this pattern? You can generate a new one!"
                    br {}
                    "Each key is completely unique and secure."
                }

                div {
                    class: "action-buttons",

                    button {
                        class: "back-btn",
                        onclick: move |_| on_back.call(()),
                        "← Back to Profile"
                    }

                    button {
                        class: "regenerate-btn",
                        disabled: is_generating,
                        onclick: move |_| on_regenerate.call(()),
                        if is_generating {
                            "🔄 Generating..."
                        } else {
                            "🎲 Generate New Key"
                        }
                    }

                    button {
                        class: "accept-btn",
                        disabled: is_generating,
                        onclick: move |_| on_accept.call(()),
                        "✨ Accept This Identity"
                    }
                }
            }
        }
    }
}

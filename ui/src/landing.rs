use crate::{CreateHandle, GenerateKey, ImportHandle, MainView};
use dioxus::prelude::*;

const LANDING_CSS: Asset = asset!("/assets/styling/landing.css");

#[component]
pub fn Landing() -> Element {
    let mut current_view = use_signal(|| "landing");
    let mut profile_data = use_signal(|| None::<(String, String, Option<String>, String)>);

    match current_view() {
        "import" => rsx! {
            ImportHandle {
                on_back: move |_| current_view.set("landing")
            }
        },
        "create" => rsx! {
            CreateHandle {
                on_back: move |_| current_view.set("landing"),
                on_submit: move |(name, handle, image, bio)| {
                    profile_data.set(Some((name, handle, image, bio)));
                    current_view.set("generate-key");
                },
                initial_name: profile_data().map(|(name, _, _, _)| name),
                initial_handle: profile_data().map(|(_, handle, _, _)| handle),
                initial_image_file: profile_data().map(|(_, _, image, _)| image),
                initial_bio: profile_data().map(|(_, _, _, bio)| bio),
            }
        },
        "generate-key" => {
            if let Some((name, handle, image_file, bio)) = profile_data() {
                rsx! {
                                                                                    GenerateKey {
                                                                                        name: name,
                                                                                        handle: handle.clone(),
                                                                                        image_file: image_file,
                                                                                        bio: bio,
                                                                                        handle_id: handle, // Use the handle from form
                                                                                        key_value: "spout_key_abc123def456...".to_string(), // TODO: Get from app-sim
                                                                                        fingerprint: r#"╭─────────────────────╮
├ ꠨ ꠩   ◎ █ ◎   ꠩ ꠨ ├
├   ┼ ◫ ❖ ✱ ❖ ◫ ┼   ├
├ ◠ ▓ █ ◎ █ ◎ █ ▓ ◠ ├
├   ┼ ◫ ❖ █ ❖ ◫ ┼   ├
├ ◠ ▓ █ ◎ █ ◎ █ ▓ ◠ ├
├   ┼ ◫ ❖ █ ❖ ◫ ┼   ├
├ ꠨ ꠩   ◎ █ ◎   ꠩ ꠨ ├
╰─────────────────────╯"#.to_string(), // TODO: Get from app-sim
                                                                                        is_generating: false, // TODO: Get from app-sim
                                                                                        on_back: move |_| current_view.set("create"),
                                                                                        on_regenerate: move |_| {
                                                                                            // TODO: Call app-sim to regenerate key
                                                                                            println!("Regenerate key requested");
                                                                                        },
                                                                                        on_accept: move |_| {
                                                                                            // TODO: Call app-sim to accept and save handle
                                                                                            println!("Handle accepted");
                                                                                            current_view.set("main");
                                                                                        },
                                                                                    }
                                                                                }
            } else {
                // Fallback if no profile data
                rsx! {
                    div { "Error: No profile data" }
                }
            }
        }
        "main" => {
            if let Some((name, handle, _, _)) = profile_data() {
                rsx! {
                    MainView {
                        user_handle: handle,
                        user_display_name: name,
                    }
                }
            } else {
                // Fallback if no profile data
                rsx! {
                    div { "Error: No profile data for main view" }
                }
            }
        }
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

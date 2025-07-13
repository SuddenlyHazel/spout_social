use crate::{
    Button, ButtonVariant, CreateHandle, GenerateKey, ImportHandle, MainView, OnboardingActions,
    OnboardingHeader, OnboardingLayout, OnboardingSection,
};
use dioxus::prelude::*;

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
            OnboardingLayout {
                OnboardingSection {
                    OnboardingHeader {
                        title: "Welcome to Spout Social!".to_string(),
                        subtitle: Some("You're ready to start your social journey. Get started by creating a new handle or importing an existing one.".to_string()),
                        gradient: Some("var(--gradient-text-mixed)".to_string()),
                        a {
                            href: "https://example.com/docs",
                            style: "
                                display: inline-block;
                                color: var(--color-primary-start);
                                text-decoration: none;
                                font-size: var(--font-size-3xl);
                                padding: var(--space-8) var(--space-16);
                                border: 1px solid var(--color-primary-start);
                                border-radius: var(--radius-md);
                                transition: var(--transition-normal);
                                margin-top: var(--space-32);
                            ",
                            class: "docs-link-hover",

                            "📖 How does this thing work?"
                        }
                    }

                    OnboardingActions {
                        Button {
                            variant: ButtonVariant::Primary,
                            full_width: true,
                            onclick: move |_| {
                                current_view.set("create");
                            },
                            "🆕 Create Handle"
                        }

                        Button {
                            variant: ButtonVariant::Secondary,
                            full_width: true,
                            onclick: move |_| {
                                current_view.set("import");
                            },
                            "📥 Import Handle"
                        }
                    }
                }
            }
        },
    }
}

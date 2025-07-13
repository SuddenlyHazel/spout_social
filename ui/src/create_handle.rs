use dioxus::prelude::*;

const CREATE_CSS: Asset = asset!("/assets/styling/create_handle.css");

#[component]
pub fn CreateHandle(on_back: EventHandler<()>) -> Element {
    let mut name = use_signal(|| String::new());
    let mut image_file = use_signal(|| None::<String>);
    let mut bio = use_signal(|| String::new());
    let mut is_creating = use_signal(|| false);

    let handle_create = move |_| {
        let name_val = name().trim().to_string();
        let image_val = image_file().unwrap_or_default();
        let bio_val = bio().trim().to_string();

        if name_val.is_empty() {
            return;
        }

        is_creating.set(true);

        // TODO: Create actual handle with these values
        println!(
            "Creating handle with name: {}, image: {}, bio: {}",
            name_val, image_val, bio_val
        );

        // For now, just simulate creation delay
        // In the future, this would create the Handle struct and save it
        is_creating.set(false);
    };

    let is_valid = !name().trim().is_empty();

    rsx! {
        document::Link { rel: "stylesheet", href: CREATE_CSS }

        div {
            id: "create-handle",

            div {
                id: "create-section",
                h1 { "Create Your Handle" }
                p {
                    class: "subtitle",
                    "Your handle is your unique identity in Spout Social. Choose wisely!"
                }

                form {
                    class: "handle-form",
                    onsubmit: handle_create,

                    div {
                        class: "form-group",
                        label {
                            r#for: "name",
                            class: "form-label required",
                            "Display Name"
                        }
                        input {
                            id: "name",
                            class: "form-input",
                            r#type: "text",
                            placeholder: "Enter your display name",
                            value: "{name}",
                            maxlength: "64",
                            oninput: move |evt| name.set(evt.value()),
                        }
                        div {
                            class: "form-hint",
                            "This is how others will see you (max 64 characters)"
                        }
                    }

                    div {
                        class: "form-group",
                        label {
                            r#for: "image",
                            class: "form-label",
                            "Profile Image"
                        }
                        input {
                            id: "image",
                            class: "form-input",
                            r#type: "file",
                            accept: "image/*",
                            onchange: move |evt| {
                                if let Some(file_engine) = &evt.files() {
                                    if let Some(file) = file_engine.files().get(0) {
                                        image_file.set(Some(file.clone()));
                                    }
                                }
                            },
                        }
                        div {
                            class: "form-hint",
                            "Optional: Upload an image file (JPG, PNG, GIF, etc.)"
                        }
                    }

                    div {
                        class: "form-group",
                        label {
                            r#for: "bio",
                            class: "form-label",
                            "Bio"
                        }
                        textarea {
                            id: "bio",
                            class: "form-textarea",
                            placeholder: "Tell us about yourself...",
                            value: "{bio}",
                            maxlength: "512",
                            rows: "4",
                            oninput: move |evt| bio.set(evt.value()),
                        }
                        div {
                            class: "form-hint",
                            "Optional: A short description about you (max 512 characters)"
                        }
                    }

                    div {
                        class: "form-actions",
                        button {
                            r#type: "button",
                            class: "back-btn",
                            onclick: move |_| on_back.call(()),
                            "← Back"
                        }

                        button {
                            r#type: "submit",
                            class: if is_valid { "create-btn enabled" } else { "create-btn disabled" },
                            disabled: !is_valid || is_creating(),
                            if is_creating() {
                                "Creating..."
                            } else {
                                "🚀 Create Handle"
                            }
                        }
                    }
                }

                if !name().trim().is_empty() {
                    div {
                        class: "preview-section",
                        h3 { "Preview" }
                        div {
                            class: "handle-preview",
                            if image_file().is_some() {
                                div {
                                    class: "preview-image-placeholder",
                                    "🖼️ Image selected"
                                }
                            }
                            div {
                                class: "preview-info",
                                div { class: "preview-name", "{name}" }
                                if !bio().trim().is_empty() {
                                    div { class: "preview-bio", "{bio}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

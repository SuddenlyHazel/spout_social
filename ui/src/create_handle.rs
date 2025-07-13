use dioxus::prelude::*;

const CREATE_CSS: Asset = asset!("/assets/styling/create_handle.css");

#[component]
pub fn CreateHandle(
    on_back: EventHandler<()>,
    on_submit: EventHandler<(String, String, Option<String>, String)>,
    initial_name: Option<String>,
    initial_handle: Option<String>,
    initial_image_file: Option<Option<String>>,
    initial_bio: Option<String>,
) -> Element {
    let mut name = use_signal(|| initial_name.unwrap_or_default());
    let mut handle = use_signal(|| initial_handle.unwrap_or_default());
    let mut image_file = use_signal(|| initial_image_file.unwrap_or_default());
    let mut bio = use_signal(|| initial_bio.unwrap_or_default());
    let mut is_creating = use_signal(|| false);

    let handle_create = move |_| {
        let name_val = name().trim().to_string();
        let handle_val = handle().trim().to_string();
        let image_val = image_file();
        let bio_val = bio().trim().to_string();

        if name_val.is_empty() || handle_val.is_empty() {
            return;
        }

        is_creating.set(true);

        // Call the parent's submit handler
        on_submit.call((name_val, handle_val, image_val, bio_val));
        is_creating.set(false);
    };

    let is_valid = !name().trim().is_empty() && !handle().trim().is_empty();

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
                            r#for: "handle",
                            class: "form-label required",
                            "Username"
                        }
                        input {
                            id: "handle",
                            class: "form-input",
                            r#type: "text",
                            placeholder: "your_username",
                            value: "{handle}",
                            maxlength: "32",
                            oninput: move |evt| {
                                let new_handle = evt.value();
                                handle.set(new_handle.clone());
                                // Auto-populate display name if it's empty or matches previous handle
                                if name().is_empty() || name() == handle() {
                                    name.set(new_handle);
                                }
                            },
                        }
                        div {
                            class: "form-hint",
                            "Your unique username (like @username, max 32 characters)"
                        }
                    }

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
                            placeholder: "How others will see you",
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
            }
        }
    }
}

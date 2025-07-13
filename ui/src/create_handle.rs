use crate::{
    Button, ButtonVariant, Form, FormActions, FormGroup, Hint, Input, Label, Page, PageContent,
    PageHeader, PageHeaderStyle, PageSection, Textarea,
};
use dioxus::prelude::*;

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

    let mut submit_form = move || {
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

    let handle_form_submit = move |_: FormEvent| {
        submit_form();
    };

    let handle_button_click = move |_| {
        submit_form();
    };

    let is_valid = !name().trim().is_empty() && !handle().trim().is_empty();

    rsx! {
        Page {
            PageSection {
                PageHeader {
                    style: PageHeaderStyle::Primary,
                    title: "Create Your Handle".to_string(),
                    subtitle: Some("Your handle is your unique identity in Spout Social. Choose wisely!".to_string()),
                }

                PageContent {
                    Form {
                        onsubmit: handle_form_submit,

                        FormGroup {
                            Label {
                                r#for: Some("handle".to_string()),
                                required: Some(true),
                                "Username"
                            }
                            Input {
                                id: Some("handle".to_string()),
                                r#type: Some("text".to_string()),
                                placeholder: Some("your_username".to_string()),
                                value: Some(handle().to_string()),
                                maxlength: Some("32".to_string()),
                                oninput: move |evt: FormEvent| {
                                    let new_handle = evt.value();
                                    let old_handle = handle();
                                    handle.set(new_handle.clone());
                                    // Auto-populate display name if it's empty or matches previous handle
                                    if name().is_empty() || name() == old_handle {
                                        name.set(new_handle);
                                    }
                                },
                            }
                            Hint {
                                "Your unique username (like @username, max 32 characters)"
                            }
                        }

                        FormGroup {
                            Label {
                                r#for: Some("name".to_string()),
                                required: Some(true),
                                "Display Name"
                            }
                            Input {
                                id: Some("name".to_string()),
                                r#type: Some("text".to_string()),
                                placeholder: Some("How others will see you".to_string()),
                                value: Some(name().to_string()),
                                maxlength: Some("64".to_string()),
                                oninput: move |evt: FormEvent| name.set(evt.value()),
                            }
                            Hint {
                                "This is how others will see you (max 64 characters)"
                            }
                        }

                        FormGroup {
                            Label {
                                r#for: Some("image".to_string()),
                                "Profile Image"
                            }
                            Input {
                                id: Some("image".to_string()),
                                r#type: Some("file".to_string()),
                                accept: Some("image/*".to_string()),
                                onchange: move |evt: FormEvent| {
                                    if let Some(file_engine) = &evt.files() {
                                        if let Some(file) = file_engine.files().get(0) {
                                            image_file.set(Some(file.clone()));
                                        }
                                    }
                                },
                            }
                            Hint {
                                "Optional: Upload an image file (JPG, PNG, GIF, etc.)"
                            }
                        }

                        FormGroup {
                            Label {
                                r#for: Some("bio".to_string()),
                                "Bio"
                            }
                            Textarea {
                                id: Some("bio".to_string()),
                                placeholder: Some("Tell us about yourself...".to_string()),
                                value: Some(bio().to_string()),
                                maxlength: Some("512".to_string()),
                                rows: Some("4".to_string()),
                                oninput: move |evt: FormEvent| bio.set(evt.value()),
                            }
                            Hint {
                                "Optional: A short description about you (max 512 characters)"
                            }
                        }

                        FormActions {
                            Button {
                                variant: ButtonVariant::Ghost,
                                onclick: move |_| on_back.call(()),
                                "← Back"
                            }

                            Button {
                                variant: ButtonVariant::Primary,
                                disabled: Some(!is_valid || is_creating()),
                                loading: Some(is_creating()),
                                onclick: handle_button_click,
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
}

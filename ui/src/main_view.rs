use dioxus::prelude::*;

const MAIN_VIEW_CSS: Asset = asset!("/assets/styling/main_view.css");

#[component]
pub fn MainView(user_handle: String, user_display_name: String) -> Element {
    let mut current_tab = use_signal(|| "feed");

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_VIEW_CSS }

        div {
            id: "main-view",

            // Header with user info and navigation
            header {
                id: "main-header",
                div {
                    class: "user-info",
                    span { class: "display-name", "{user_display_name}" }
                    span { class: "handle", "@{user_handle}" }
                }
                nav {
                    class: "main-nav",
                    button {
                        class: if current_tab() == "feed" { "nav-btn active" } else { "nav-btn" },
                        onclick: move |_| current_tab.set("feed"),
                        "📰 Feed"
                    }
                    button {
                        class: if current_tab() == "groups" { "nav-btn active" } else { "nav-btn" },
                        onclick: move |_| current_tab.set("groups"),
                        "🏘️ Groups"
                    }
                    button {
                        class: if current_tab() == "dms" { "nav-btn active" } else { "nav-btn" },
                        onclick: move |_| current_tab.set("dms"),
                        "💬 DMs"
                    }
                }
            }

            // Main content area
            main {
                id: "main-content",
                match current_tab() {
                    "feed" => rsx! { FeedView {} },
                    "groups" => rsx! { GroupsView {} },
                    "dms" => rsx! { DMsView {} },
                    _ => rsx! { FeedView {} }
                }
            }
        }
    }
}

#[component]
fn FeedView() -> Element {
    rsx! {
        div {
            class: "feed-container",

            div {
                class: "feed-header",
                h2 { "Your Feed" }
                p { class: "feed-subtitle", "Latest posts from all your groups" }
            }

            div {
                class: "posts-list",

                // Sample posts - these would come from props/state in real app
                div {
                    class: "post-card",
                    div {
                        class: "post-header",
                        span { class: "post-author", "@techie_sarah" }
                        span { class: "post-group", "in #rust-lang" }
                        span { class: "post-time", "2h ago" }
                    }
                    div {
                        class: "post-content",
                        h3 { "New async features in Rust 1.75" }
                        p { "Just discovered some amazing new async improvements..." }
                    }
                    div {
                        class: "post-actions",
                        button { class: "action-btn", "👍 12" }
                        button { class: "action-btn", "💬 8" }
                        button { class: "action-btn", "🔄 3" }
                    }
                }

                div {
                    class: "post-card",
                    div {
                        class: "post-header",
                        span { class: "post-author", "@crypto_dev" }
                        span { class: "post-group", "in #blockchain" }
                        span { class: "post-time", "4h ago" }
                    }
                    div {
                        class: "post-content",
                        h3 { "Building decentralized identity systems" }
                        p { "Working on a new approach to digital identity..." }
                    }
                    div {
                        class: "post-actions",
                        button { class: "action-btn", "👍 25" }
                        button { class: "action-btn", "💬 15" }
                        button { class: "action-btn", "🔄 7" }
                    }
                }

                div {
                    class: "load-more",
                    button { class: "load-more-btn", "Load More Posts" }
                }
            }
        }
    }
}

#[component]
fn GroupsView() -> Element {
    rsx! {
        div {
            class: "groups-container",

            div {
                class: "groups-header",
                h2 { "Your Groups" }
                p { class: "groups-subtitle", "Communities you're part of" }
                button { class: "create-group-btn", "➕ Create Group" }
            }

            div {
                class: "groups-list",

                // Sample groups - these would come from props/state in real app
                div {
                    class: "group-card",
                    div {
                        class: "group-info",
                        h3 { class: "group-name", "#rust-lang" }
                        p { class: "group-description", "Rust programming language discussion" }
                        div {
                            class: "group-stats",
                            span { "1,234 members" }
                            span { "•" }
                            span { "23 online" }
                        }
                    }
                    div {
                        class: "group-actions",
                        button { class: "group-btn posts-btn", "📝 Posts" }
                        button { class: "group-btn chat-btn", "💬 Chat" }
                    }
                }

                div {
                    class: "group-card",
                    div {
                        class: "group-info",
                        h3 { class: "group-name", "#blockchain" }
                        p { class: "group-description", "Blockchain technology and cryptocurrency" }
                        div {
                            class: "group-stats",
                            span { "856 members" }
                            span { "•" }
                            span { "12 online" }
                        }
                    }
                    div {
                        class: "group-actions",
                        button { class: "group-btn posts-btn", "📝 Posts" }
                        button { class: "group-btn chat-btn", "💬 Chat" }
                    }
                }

                div {
                    class: "group-card",
                    div {
                        class: "group-info",
                        h3 { class: "group-name", "#web-dev" }
                        p { class: "group-description", "Web development and modern frameworks" }
                        div {
                            class: "group-stats",
                            span { "2,103 members" }
                            span { "•" }
                            span { "45 online" }
                        }
                    }
                    div {
                        class: "group-actions",
                        button { class: "group-btn posts-btn", "📝 Posts" }
                        button { class: "group-btn chat-btn", "💬 Chat" }
                    }
                }
            }
        }
    }
}

#[component]
fn DMsView() -> Element {
    rsx! {
        div {
            class: "dms-container",

            div {
                class: "dms-header",
                h2 { "Direct Messages" }
                p { class: "dms-subtitle", "Private conversations" }
                button { class: "new-dm-btn", "➕ New Message" }
            }

            div {
                class: "dms-list",

                // Sample DMs - these would come from props/state in real app
                div {
                    class: "dm-card",
                    div {
                        class: "dm-avatar",
                        "👤"
                    }
                    div {
                        class: "dm-info",
                        div {
                            class: "dm-header",
                            span { class: "dm-name", "Sarah Chen" }
                            span { class: "dm-handle", "@techie_sarah" }
                            span { class: "dm-time", "5m ago" }
                        }
                        div {
                            class: "dm-preview",
                            "Thanks for the help with that async issue!"
                        }
                    }
                    div {
                        class: "dm-status",
                        span { class: "unread-count", "2" }
                    }
                }

                div {
                    class: "dm-card",
                    div {
                        class: "dm-avatar",
                        "👤"
                    }
                    div {
                        class: "dm-info",
                        div {
                            class: "dm-header",
                            span { class: "dm-name", "Alex Rivera" }
                            span { class: "dm-handle", "@crypto_dev" }
                            span { class: "dm-time", "2h ago" }
                        }
                        div {
                            class: "dm-preview",
                            "Are you going to the blockchain meetup?"
                        }
                    }
                    div {
                        class: "dm-status",
                        span { class: "read-indicator", "✓" }
                    }
                }

                div {
                    class: "dm-card",
                    div {
                        class: "dm-avatar",
                        "👤"
                    }
                    div {
                        class: "dm-info",
                        div {
                            class: "dm-header",
                            span { class: "dm-name", "Jordan Kim" }
                            span { class: "dm-handle", "@webdev_jordan" }
                            span { class: "dm-time", "1d ago" }
                        }
                        div {
                            class: "dm-preview",
                            "Check out this new React framework I found"
                        }
                    }
                    div {
                        class: "dm-status",
                        span { class: "read-indicator", "✓" }
                    }
                }
            }
        }
    }
}

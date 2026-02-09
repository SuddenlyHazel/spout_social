use crate::{
    AppLayout, Button, ButtonVariant, Card, CardContent, CardHeader, CardVariant, MainContent,
    MainHeader,
};
use dioxus::prelude::*;

#[component]
pub fn MainView(user_handle: String, user_display_name: String) -> Element {
    let mut current_tab = use_signal(|| "feed");

    rsx! {
        AppLayout {
            MainHeader {
                user_handle: user_handle.clone(),
                user_display_name: user_display_name.clone(),
                Button {
                    variant: ButtonVariant::Nav,
                    active: Some(current_tab() == "feed"),
                    onclick: move |_| current_tab.set("feed"),
                    "📰 Feed"
                }
                Button {
                    variant: ButtonVariant::Nav,
                    active: Some(current_tab() == "groups"),
                    onclick: move |_| current_tab.set("groups"),
                    "🏘️ Groups"
                }
                Button {
                    variant: ButtonVariant::Nav,
                    active: Some(current_tab() == "dms"),
                    onclick: move |_| current_tab.set("dms"),
                    "💬 DMs"
                }
            }

            MainContent {
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
            style: "
                max-width: 100%;
                display: flex;
                flex-direction: column;
                gap: var(--space-4);
            ",

            div {
                style: "
                    text-align: center;
                    padding: var(--space-4) var(--space-2);
                ",
                h2 {
                    style: "
                        font-size: var(--font-size-4xl);
                        font-weight: var(--font-weight-semibold);
                        color: var(--color-text-dark);
                        margin-bottom: var(--space-2);
                    ",
                    "Your Feed"
                }
                p {
                    style: "
                        color: var(--color-gray-600);
                        font-size: var(--font-size-md);
                    ",
                    "Latest posts from all your groups"
                }
            }

            div {
                style: "
                    display: flex;
                    flex-direction: column;
                    gap: var(--space-4);
                ",

                Card {
                    variant: CardVariant::Post,
                    interactive: true,
                    CardHeader {
                        div {
                            style: "
                                display: flex;
                                gap: var(--space-4);
                                align-items: center;
                                font-size: var(--font-size-sm);
                                width: 100%;
                            ",
                            span {
                                style: "
                                    font-weight: var(--font-weight-semibold);
                                    color: var(--color-info);
                                ",
                                "@techie_sarah"
                            }
                            span {
                                style: "color: var(--color-gray-500);",
                                "in #rust-lang"
                            }
                            span {
                                style: "
                                    color: var(--color-gray-400);
                                    margin-left: auto;
                                ",
                                "2h ago"
                            }
                        }
                    }
                    CardContent {
                        h3 {
                            style: "
                                font-size: var(--font-size-lg);
                                font-weight: var(--font-weight-semibold);
                                color: var(--color-text-dark);
                                margin-bottom: var(--space-2);
                                line-height: var(--line-height-snug);
                            ",
                            "New async features in Rust 1.75"
                        }
                        p {
                            style: "
                                color: var(--color-gray-600);
                                font-size: var(--font-size-base);
                                line-height: var(--line-height-normal);
                                margin-bottom: var(--space-4);
                            ",
                            "Just discovered some amazing new async improvements..."
                        }
                        div {
                            style: "
                                display: flex;
                                gap: var(--space-4);
                                padding-top: var(--space-4);
                                border-top: 1px solid var(--color-gray-200);
                            ",
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: crate::ButtonSize::Small,
                                "👍 12"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: crate::ButtonSize::Small,
                                "💬 8"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: crate::ButtonSize::Small,
                                "🔄 3"
                            }
                        }
                    }
                }

                Card {
                    variant: CardVariant::Post,
                    interactive: true,
                    CardHeader {
                        div {
                            style: "
                                display: flex;
                                gap: var(--space-4);
                                align-items: center;
                                font-size: var(--font-size-sm);
                                width: 100%;
                            ",
                            span {
                                style: "
                                    font-weight: var(--font-weight-semibold);
                                    color: var(--color-info);
                                ",
                                "@crypto_dev"
                            }
                            span {
                                style: "color: var(--color-gray-500);",
                                "in #blockchain"
                            }
                            span {
                                style: "
                                    color: var(--color-gray-400);
                                    margin-left: auto;
                                ",
                                "4h ago"
                            }
                        }
                    }
                    CardContent {
                        h3 {
                            style: "
                                font-size: var(--font-size-lg);
                                font-weight: var(--font-weight-semibold);
                                color: var(--color-text-dark);
                                margin-bottom: var(--space-2);
                                line-height: var(--line-height-snug);
                            ",
                            "Building decentralized identity systems"
                        }
                        p {
                            style: "
                                color: var(--color-gray-600);
                                font-size: var(--font-size-base);
                                line-height: var(--line-height-normal);
                                margin-bottom: var(--space-4);
                            ",
                            "Working on a new approach to digital identity..."
                        }
                        div {
                            style: "
                                display: flex;
                                gap: var(--space-4);
                                padding-top: var(--space-4);
                                border-top: 1px solid var(--color-gray-200);
                            ",
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: crate::ButtonSize::Small,
                                "👍 25"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: crate::ButtonSize::Small,
                                "💬 15"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: crate::ButtonSize::Small,
                                "🔄 7"
                            }
                        }
                    }
                }

                div {
                    style: "
                        text-align: center;
                        margin-top: var(--space-8);
                    ",
                    Button {
                        variant: ButtonVariant::Info,
                        "Load More Posts"
                    }
                }
            }
        }
    }
}

#[component]
fn GroupsView() -> Element {
    rsx! {
        div {
            style: "
                max-width: 100%;
                display: flex;
                flex-direction: column;
                gap: var(--space-4);
            ",

            div {
                style: "
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: var(--space-4) var(--space-2);
                    flex-wrap: wrap;
                    gap: var(--space-4);
                ",
                div {
                    h2 {
                        style: "
                            font-size: var(--font-size-4xl);
                            font-weight: var(--font-weight-semibold);
                            color: var(--color-text-dark);
                            margin-bottom: var(--space-2);
                        ",
                        "Your Groups"
                    }
                    p {
                        style: "
                            color: var(--color-gray-600);
                            font-size: var(--font-size-md);
                        ",
                        "Communities you're part of"
                    }
                }
                Button {
                    variant: ButtonVariant::Success,
                    size: crate::ButtonSize::Medium,
                    "➕ Create Group"
                }
            }

            div {
                style: "
                    display: flex;
                    flex-direction: column;
                    gap: var(--space-4);
                ",

                Card {
                    variant: CardVariant::Group,
                    interactive: true,
                    CardContent {
                        div {
                            style: "margin-bottom: var(--space-4);",
                            h3 {
                                style: "
                                    font-size: var(--font-size-xl);
                                    font-weight: var(--font-weight-semibold);
                                    color: var(--color-text-dark);
                                    margin-bottom: var(--space-2);
                                    font-family: var(--font-family-mono);
                                ",
                                "#rust-lang"
                            }
                            p {
                                style: "
                                    color: var(--color-gray-600);
                                    font-size: var(--font-size-base);
                                    margin-bottom: var(--space-3);
                                    line-height: var(--line-height-snug);
                                ",
                                "Rust programming language discussion"
                            }
                            div {
                                style: "
                                    display: flex;
                                    gap: var(--space-3);
                                    font-size: var(--font-size-sm);
                                    color: var(--color-gray-400);
                                    margin-bottom: var(--space-4);
                                ",
                                span { "1,234 members" }
                                span { "•" }
                                span { "23 online" }
                            }
                        }
                        div {
                            style: "
                                display: flex;
                                gap: var(--space-4);
                            ",
                            Button {
                                variant: ButtonVariant::Info,
                                size: crate::ButtonSize::Medium,
                                flex: Some("1".to_string()),
                                "📝 Posts"
                            }
                            Button {
                                variant: ButtonVariant::Success,
                                size: crate::ButtonSize::Medium,
                                flex: Some("1".to_string()),
                                "💬 Chat"
                            }
                        }
                    }
                }

                Card {
                    variant: CardVariant::Group,
                    interactive: true,
                    CardContent {
                        div {
                            style: "margin-bottom: var(--space-4);",
                            h3 {
                                style: "
                                    font-size: var(--font-size-xl);
                                    font-weight: var(--font-weight-semibold);
                                    color: var(--color-text-dark);
                                    margin-bottom: var(--space-2);
                                    font-family: var(--font-family-mono);
                                ",
                                "#blockchain"
                            }
                            p {
                                style: "
                                    color: var(--color-gray-600);
                                    font-size: var(--font-size-base);
                                    margin-bottom: var(--space-3);
                                    line-height: var(--line-height-snug);
                                ",
                                "Blockchain technology and cryptocurrency"
                            }
                            div {
                                style: "
                                    display: flex;
                                    gap: var(--space-3);
                                    font-size: var(--font-size-sm);
                                    color: var(--color-gray-400);
                                    margin-bottom: var(--space-4);
                                ",
                                span { "856 members" }
                                span { "•" }
                                span { "12 online" }
                            }
                        }
                        div {
                            style: "
                                display: flex;
                                gap: var(--space-4);
                            ",
                            Button {
                                variant: ButtonVariant::Info,
                                size: crate::ButtonSize::Medium,
                                flex: Some("1".to_string()),
                                "📝 Posts"
                            }
                            Button {
                                variant: ButtonVariant::Success,
                                size: crate::ButtonSize::Medium,
                                flex: Some("1".to_string()),
                                "💬 Chat"
                            }
                        }
                    }
                }

                Card {
                    variant: CardVariant::Group,
                    interactive: true,
                    CardContent {
                        div {
                            style: "margin-bottom: var(--space-4);",
                            h3 {
                                style: "
                                    font-size: var(--font-size-xl);
                                    font-weight: var(--font-weight-semibold);
                                    color: var(--color-text-dark);
                                    margin-bottom: var(--space-2);
                                    font-family: var(--font-family-mono);
                                ",
                                "#web-dev"
                            }
                            p {
                                style: "
                                    color: var(--color-gray-600);
                                    font-size: var(--font-size-base);
                                    margin-bottom: var(--space-3);
                                    line-height: var(--line-height-snug);
                                ",
                                "Web development and modern frameworks"
                            }
                            div {
                                style: "
                                    display: flex;
                                    gap: var(--space-3);
                                    font-size: var(--font-size-sm);
                                    color: var(--color-gray-400);
                                    margin-bottom: var(--space-4);
                                ",
                                span { "2,103 members" }
                                span { "•" }
                                span { "45 online" }
                            }
                        }
                        div {
                            style: "
                                display: flex;
                                gap: var(--space-4);
                            ",
                            Button {
                                variant: ButtonVariant::Info,
                                size: crate::ButtonSize::Medium,
                                flex: Some("1".to_string()),
                                "📝 Posts"
                            }
                            Button {
                                variant: ButtonVariant::Success,
                                size: crate::ButtonSize::Medium,
                                flex: Some("1".to_string()),
                                "💬 Chat"
                            }
                        }
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
            style: "
                max-width: 100%;
                display: flex;
                flex-direction: column;
                gap: var(--space-4);
            ",

            div {
                style: "
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: var(--space-4) var(--space-2);
                    flex-wrap: wrap;
                    gap: var(--space-4);
                ",
                div {
                    h2 {
                        style: "
                            font-size: var(--font-size-4xl);
                            font-weight: var(--font-weight-semibold);
                            color: var(--color-text-dark);
                            margin-bottom: var(--space-2);
                        ",
                        "Direct Messages"
                    }
                    p {
                        style: "
                            color: var(--color-gray-600);
                            font-size: var(--font-size-md);
                        ",
                        "Private conversations"
                    }
                }
                Button {
                    variant: ButtonVariant::Purple,
                    size: crate::ButtonSize::Medium,
                    "➕ New Message"
                }
            }

            div {
                style: "
                    display: flex;
                    flex-direction: column;
                    gap: var(--space-2);
                ",

                Card {
                    variant: CardVariant::Dm,
                    interactive: true,
                    div {
                        style: "
                            width: 36px;
                            height: 36px;
                            border-radius: var(--radius-full);
                            background-color: var(--color-gray-100);
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            font-size: var(--font-size-3xl);
                            flex-shrink: 0;
                        ",
                        "👤"
                    }
                    div {
                        style: "
                            flex: 1;
                            min-width: 0;
                        ",
                        div {
                            style: "
                                display: flex;
                                gap: var(--space-3);
                                align-items: center;
                                margin-bottom: var(--space-1);
                            ",
                            span {
                                style: "
                                    font-weight: var(--font-weight-semibold);
                                    color: var(--color-text-dark);
                                    font-size: var(--font-size-md);
                                ",
                                "Sarah Chen"
                            }
                            span {
                                style: "
                                    color: var(--color-gray-500);
                                    font-size: var(--font-size-sm);
                                    font-family: var(--font-family-mono);
                                ",
                                "@techie_sarah"
                            }
                            span {
                                style: "
                                    color: var(--color-gray-400);
                                    font-size: var(--font-size-xs);
                                    margin-left: auto;
                                ",
                                "5m ago"
                            }
                        }
                        div {
                            style: "
                                color: var(--color-gray-600);
                                font-size: var(--font-size-base);
                                white-space: nowrap;
                                overflow: hidden;
                                text-overflow: ellipsis;
                                line-height: var(--line-height-snug);
                            ",
                            "Thanks for the help with that async issue!"
                        }
                    }
                    div {
                        style: "
                            display: flex;
                            align-items: center;
                            flex-shrink: 0;
                        ",
                        span {
                            style: "
                                background-color: var(--color-error);
                                color: var(--color-white);
                                border-radius: var(--radius-full);
                                width: 18px;
                                height: 18px;
                                font-size: var(--font-size-xs);
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                font-weight: var(--font-weight-semibold);
                            ",
                            "2"
                        }
                    }
                }

                Card {
                    variant: CardVariant::Dm,
                    interactive: true,
                    div {
                        style: "
                            width: 36px;
                            height: 36px;
                            border-radius: var(--radius-full);
                            background-color: var(--color-gray-100);
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            font-size: var(--font-size-3xl);
                            flex-shrink: 0;
                        ",
                        "👤"
                    }
                    div {
                        style: "
                            flex: 1;
                            min-width: 0;
                        ",
                        div {
                            style: "
                                display: flex;
                                gap: var(--space-3);
                                align-items: center;
                                margin-bottom: var(--space-1);
                            ",
                            span {
                                style: "
                                    font-weight: var(--font-weight-semibold);
                                    color: var(--color-text-dark);
                                    font-size: var(--font-size-md);
                                ",
                                "Alex Rivera"
                            }
                            span {
                                style: "
                                    color: var(--color-gray-500);
                                    font-size: var(--font-size-sm);
                                    font-family: var(--font-family-mono);
                                ",
                                "@crypto_dev"
                            }
                            span {
                                style: "
                                    color: var(--color-gray-400);
                                    font-size: var(--font-size-xs);
                                    margin-left: auto;
                                ",
                                "2h ago"
                            }
                        }
                        div {
                            style: "
                                color: var(--color-gray-600);
                                font-size: var(--font-size-base);
                                white-space: nowrap;
                                overflow: hidden;
                                text-overflow: ellipsis;
                                line-height: var(--line-height-snug);
                            ",
                            "Are you going to the blockchain meetup?"
                        }
                    }
                    div {
                        style: "
                            display: flex;
                            align-items: center;
                            flex-shrink: 0;
                        ",
                        span {
                            style: "
                                color: var(--color-success);
                                font-size: var(--font-size-base);
                                font-weight: var(--font-weight-semibold);
                            ",
                            "✓"
                        }
                    }
                }

                Card {
                    variant: CardVariant::Dm,
                    interactive: true,
                    div {
                        style: "
                            width: 36px;
                            height: 36px;
                            border-radius: var(--radius-full);
                            background-color: var(--color-gray-100);
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            font-size: var(--font-size-3xl);
                            flex-shrink: 0;
                        ",
                        "👤"
                    }
                    div {
                        style: "
                            flex: 1;
                            min-width: 0;
                        ",
                        div {
                            style: "
                                display: flex;
                                gap: var(--space-3);
                                align-items: center;
                                margin-bottom: var(--space-1);
                            ",
                            span {
                                style: "
                                    font-weight: var(--font-weight-semibold);
                                    color: var(--color-text-dark);
                                    font-size: var(--font-size-md);
                                ",
                                "Jordan Kim"
                            }
                            span {
                                style: "
                                    color: var(--color-gray-500);
                                    font-size: var(--font-size-sm);
                                    font-family: var(--font-family-mono);
                                ",
                                "@webdev_jordan"
                            }
                            span {
                                style: "
                                    color: var(--color-gray-400);
                                    font-size: var(--font-size-xs);
                                    margin-left: auto;
                                ",
                                "1d ago"
                            }
                        }
                        div {
                            style: "
                                color: var(--color-gray-600);
                                font-size: var(--font-size-base);
                                white-space: nowrap;
                                overflow: hidden;
                                text-overflow: ellipsis;
                                line-height: var(--line-height-snug);
                            ",
                            "Check out this new React framework I found"
                        }
                    }
                    div {
                        style: "
                            display: flex;
                            align-items: center;
                            flex-shrink: 0;
                        ",
                        span {
                            style: "
                                color: var(--color-success);
                                font-size: var(--font-size-base);
                                font-weight: var(--font-weight-semibold);
                            ",
                            "✓"
                        }
                    }
                }
            }
        }
    }
}

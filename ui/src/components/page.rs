use dioxus::prelude::*;

const PAGE_CSS: Asset = asset!("/assets/styling/components/page.css");

#[derive(Clone, PartialEq)]
pub enum PageVariant {
    Dark,
    Light,
    Main,
}

#[derive(Clone, PartialEq)]
pub enum PageHeaderStyle {
    Primary,   // blue gradient
    Secondary, // red gradient
    Mixed,     // blue to red gradient
}

#[component]
pub fn Page(variant: Option<PageVariant>, children: Element) -> Element {
    let variant = variant.unwrap_or(PageVariant::Dark);

    let mut class_list = vec!["page".to_string()];

    match variant {
        PageVariant::Dark => class_list.push("page--dark".to_string()),
        PageVariant::Light => class_list.push("page--light".to_string()),
        PageVariant::Main => class_list.push("page--main".to_string()),
    }

    let class_string = class_list.join(" ");

    rsx! {
        document::Link { rel: "stylesheet", href: PAGE_CSS }

        div {
            class: "{class_string}",
            {children}
        }
    }
}

#[component]
pub fn PageSection(
    wide: Option<bool>,
    full: Option<bool>,
    centered: Option<bool>,
    children: Element,
) -> Element {
    let wide = wide.unwrap_or(false);
    let full = full.unwrap_or(false);
    let centered = centered.unwrap_or(false);

    let mut class_list = vec!["page-section".to_string()];

    if wide {
        class_list.push("page-section--wide".to_string());
    }
    if full {
        class_list.push("page-section--full".to_string());
    }
    if centered {
        class_list.push("page-section--centered".to_string());
    }

    let class_string = class_list.join(" ");

    rsx! {
        div {
            class: "{class_string}",
            {children}
        }
    }
}

#[component]
pub fn PageHeader(
    style: Option<PageHeaderStyle>,
    large: Option<bool>,
    compact: Option<bool>,
    title: String,
    subtitle: Option<String>,
    children: Option<Element>,
) -> Element {
    let style = style.unwrap_or(PageHeaderStyle::Primary);
    let large = large.unwrap_or(false);
    let compact = compact.unwrap_or(false);

    let mut header_class = vec!["page-header".to_string()];
    let mut title_class = vec![];

    if compact {
        header_class.push("page-header--compact".to_string());
    }

    match style {
        PageHeaderStyle::Primary => title_class.push("page-header--dark".to_string()),
        PageHeaderStyle::Secondary => title_class.push("page-header--secondary".to_string()),
        PageHeaderStyle::Mixed => title_class.push("page-header--mixed".to_string()),
    }

    if large {
        title_class.push("page-header--large".to_string());
    }

    let header_class_string = header_class.join(" ");
    let title_class_string = title_class.join(" ");

    rsx! {
        div {
            class: "{header_class_string} {title_class_string}",
            h1 { "{title}" }
            if let Some(subtitle_text) = subtitle {
                p {
                    class: if large { "page-subtitle page-subtitle--large" } else { "page-subtitle" },
                    "{subtitle_text}"
                }
            }
            if let Some(extra_content) = children {
                {extra_content}
            }
        }
    }
}

#[component]
pub fn PageContent(flex: Option<bool>, children: Element) -> Element {
    let flex = flex.unwrap_or(false);

    let class = if flex {
        "page-content page-content--flex"
    } else {
        "page-content"
    };

    rsx! {
        div {
            class: "{class}",
            {children}
        }
    }
}

#[component]
pub fn PageActions(
    column: Option<bool>,
    wide: Option<bool>,
    row_md: Option<bool>,
    children: Element,
) -> Element {
    let column = column.unwrap_or(false);
    let wide = wide.unwrap_or(false);
    let row_md = row_md.unwrap_or(false);

    let mut class_list = vec!["page-actions".to_string()];

    if column {
        class_list.push("page-actions--column".to_string());
    }
    if wide {
        class_list.push("page-actions--wide".to_string());
    }
    if row_md {
        class_list.push("page-actions--row-md".to_string());
    }

    let class_string = class_list.join(" ");

    rsx! {
        div {
            class: "{class_string}",
            {children}
        }
    }
}

#[component]
pub fn PageInfo(children: Element) -> Element {
    rsx! {
        div {
            class: "page-info",
            {children}
        }
    }
}

#[component]
pub fn MainHeader(user_handle: String, user_display_name: String, children: Element) -> Element {
    rsx! {
        header {
            class: "main-header",
            div {
                class: "user-info",
                span { class: "display-name", "{user_display_name}" }
                span { class: "handle", "@{user_handle}" }
            }
            nav {
                class: "main-nav",
                {children}
            }
        }
    }
}

#[component]
pub fn MainContent(children: Element) -> Element {
    rsx! {
        main {
            class: "main-content",
            {children}
        }
    }
}

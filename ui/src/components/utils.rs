/// Helper for building CSS class strings conditionally
pub fn build_classes(base: &str, variants: &[(&str, bool)]) -> String {
    let mut classes = vec![base];
    for (class, condition) in variants {
        if *condition {
            classes.push(class);
        }
    }
    classes.join(" ")
}

/// More ergonomic class builder with fluent interface
pub struct ClassBuilder {
    classes: Vec<String>,
}

impl ClassBuilder {
    pub fn new(base: &str) -> Self {
        Self {
            classes: vec![base.to_string()],
        }
    }

    pub fn add(mut self, class: &str) -> Self {
        self.classes.push(class.to_string());
        self
    }

    pub fn add_if(mut self, class: &str, condition: bool) -> Self {
        if condition {
            self.classes.push(class.to_string());
        }
        self
    }

    pub fn build(self) -> String {
        self.classes.join(" ")
    }
}

/// Common input types for type safety
#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Email,
    Password,
    File,
    Number,
    Tel,
    Url,
    Search,
    Hidden,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::File => "file",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
            InputType::Hidden => "hidden",
        }
    }
}

/// Flex value types for type safety
#[derive(Clone, PartialEq)]
pub enum FlexValue {
    Auto,
    None,
    Initial,
    Grow(u32), // flex-grow value
}

impl FlexValue {
    pub fn as_css_class(&self) -> &'static str {
        match self {
            FlexValue::Auto => "flex-auto",
            FlexValue::None => "flex-none",
            FlexValue::Initial => "flex-initial",
            FlexValue::Grow(1) => "flex-1",
            FlexValue::Grow(2) => "flex-2",
            FlexValue::Grow(3) => "flex-3",
            FlexValue::Grow(_) => "flex-grow", // fallback for custom values
        }
    }
}

/// Common breakpoint utilities for responsive design
#[derive(Clone, PartialEq)]
pub enum Breakpoint {
    Mobile,  // < 480px
    Tablet,  // 480px - 768px
    Desktop, // > 768px
}

/// Touch target size recommendations
#[derive(Clone, PartialEq)]
pub enum TouchTarget {
    Small,   // 36px - for secondary actions
    Default, // 44px - recommended minimum
    Large,   // 60px - for primary actions
}

impl TouchTarget {
    pub fn as_css_class(&self) -> &'static str {
        match self {
            TouchTarget::Small => "touch-sm",
            TouchTarget::Default => "touch-default",
            TouchTarget::Large => "touch-lg",
        }
    }
}

/// Event handler utilities
pub mod events {
    use dioxus::prelude::*;

    /// Prevents default and calls handler
    pub fn prevent_default_and_call<T>(event: Event<T>, handler: Option<EventHandler<Event<T>>>)
    where
        T: Clone + 'static,
    {
        event.prevent_default();
        if let Some(h) = handler {
            h.call(event);
        }
    }

    /// Stops propagation and calls handler
    pub fn stop_propagation_and_call<T>(event: Event<T>, handler: Option<EventHandler<Event<T>>>)
    where
        T: Clone + 'static,
    {
        event.stop_propagation();
        if let Some(h) = handler {
            h.call(event);
        }
    }

    /// Calls handler only if condition is met
    pub fn call_if<T>(event: Event<T>, handler: Option<EventHandler<Event<T>>>, condition: bool)
    where
        T: Clone + 'static,
    {
        if condition {
            if let Some(h) = handler {
                h.call(event);
            }
        }
    }
}

/// Accessibility utilities
pub mod a11y {
    /// Generate ARIA attributes for buttons
    pub fn button_aria(
        disabled: bool,
        loading: bool,
        expanded: Option<bool>,
        pressed: Option<bool>,
    ) -> Vec<(&'static str, String)> {
        let mut attrs = vec![];

        if disabled {
            attrs.push(("aria-disabled", "true".to_string()));
        }

        if loading {
            attrs.push(("aria-busy", "true".to_string()));
        }

        if let Some(exp) = expanded {
            attrs.push(("aria-expanded", exp.to_string()));
        }

        if let Some(press) = pressed {
            attrs.push(("aria-pressed", press.to_string()));
        }

        attrs
    }

    /// Generate ARIA attributes for form inputs
    pub fn input_aria(
        required: bool,
        invalid: bool,
        describedby: Option<&str>,
    ) -> Vec<(&'static str, String)> {
        let mut attrs = vec![];

        if required {
            attrs.push(("aria-required", "true".to_string()));
        }

        if invalid {
            attrs.push(("aria-invalid", "true".to_string()));
        }

        if let Some(desc) = describedby {
            attrs.push(("aria-describedby", desc.to_string()));
        }

        attrs
    }
}

/// Common prop patterns
pub mod props {

    /// Standard size variants across components
    #[derive(Clone, PartialEq)]
    pub enum Size {
        ExtraSmall,
        Small,
        Medium,
        Large,
        ExtraLarge,
    }

    impl Size {
        pub fn as_css_modifier(&self) -> &'static str {
            match self {
                Size::ExtraSmall => "--xs",
                Size::Small => "--sm",
                Size::Medium => "", // default, no modifier
                Size::Large => "--lg",
                Size::ExtraLarge => "--xl",
            }
        }
    }

    /// Standard spacing variants
    #[derive(Clone, PartialEq)]
    pub enum Spacing {
        None,
        Small,
        Medium,
        Large,
        ExtraLarge,
    }

    impl Spacing {
        pub fn as_css_class(&self) -> &'static str {
            match self {
                Spacing::None => "spacing-none",
                Spacing::Small => "spacing-sm",
                Spacing::Medium => "spacing-md",
                Spacing::Large => "spacing-lg",
                Spacing::ExtraLarge => "spacing-xl",
            }
        }
    }
}

/// Platform-specific utilities
pub mod platform {

    /// Check if running on mobile platform
    pub fn is_mobile() -> bool {
        // This would need platform-specific implementation
        // For now, assume based on user agent or screen size
        false // placeholder
    }

    /// Check if running on desktop
    pub fn is_desktop() -> bool {
        !is_mobile()
    }

    /// Get platform-specific class modifier
    pub fn platform_class() -> &'static str {
        if is_mobile() {
            "platform-mobile"
        } else {
            "platform-desktop"
        }
    }
}

/// CSS Custom Properties helpers
pub mod css {
    /// Generate CSS custom property reference
    pub fn var(name: &str) -> String {
        format!("var(--{})", name)
    }

    /// Generate CSS custom property with fallback
    pub fn var_with_fallback(name: &str, fallback: &str) -> String {
        format!("var(--{}, {})", name, fallback)
    }

    /// Common CSS custom properties from tokens
    pub mod tokens {
        use super::var;

        pub fn color_primary() -> String {
            var("color-primary-start")
        }

        pub fn color_text_primary() -> String {
            var("color-text-primary")
        }

        pub fn space(size: u32) -> String {
            var(&format!("space-{}", size))
        }

        pub fn font_size(size: &str) -> String {
            var(&format!("font-size-{}", size))
        }

        pub fn border_radius(size: &str) -> String {
            var(&format!("radius-{}", size))
        }
    }
}

/// Common validation patterns
pub mod validation {
    /// Check if email format is valid
    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.len() > 3
    }

    /// Check if password meets minimum requirements
    pub fn is_valid_password(password: &str) -> bool {
        password.len() >= 8
    }

    /// Check if required field is filled
    pub fn is_required_filled(value: &str) -> bool {
        !value.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_classes() {
        let result = build_classes("btn", &[("btn--primary", true), ("btn--disabled", false)]);
        assert_eq!(result, "btn btn--primary");
    }

    #[test]
    fn test_class_builder() {
        let result = ClassBuilder::new("card")
            .add("card--elevated")
            .add_if("card--interactive", true)
            .add_if("card--disabled", false)
            .build();
        assert_eq!(result, "card card--elevated card--interactive");
    }

    #[test]
    fn test_input_type() {
        assert_eq!(InputType::Email.as_str(), "email");
        assert_eq!(InputType::Password.as_str(), "password");
    }
}

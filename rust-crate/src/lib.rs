//! # {{app_name}}
//! 
//! {{description}}

pub struct Hello {
    greeting: String,
}

impl Hello {
    /// Creates a new Hello instance with the default greeting.
    pub fn new() -> Self {
        Self {
            greeting: "Hello, World!".to_string(),
        }
    }

    /// Sets a new greeting.
    pub fn set_greeting(&mut self, new_greeting: impl Into<String>) {
        self.greeting = new_greeting.into();
    }

    /// Gets the current greeting.
    pub fn get_greeting(&self) -> &str {
        &self.greeting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_greeting() {
        let hello = Hello::new();
        assert_eq!(hello.get_greeting(), "Hello, World!");
    }

    #[test]
    fn set_and_get_greeting() {
        let mut hello = Hello::new();
        hello.set_greeting("Hi, Rust!");
        assert_eq!(hello.get_greeting(), "Hi, Rust!");
    }
}

[package]
name = "{{project-name}}"
version = "0.1.0"
edition = "2021"
authors = ["{{author-name}} <{{author-email}}>"]
description = "{{description}}"
license = "MIT"

[dependencies]
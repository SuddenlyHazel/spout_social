mod drunken_bishop;

use bincode::Encode;
use drunken_bishop::{
    DrunkenBishop, DrunkenBishopConfig, generate_fingerprint, generate_fingerprint_from_bytes,
    generate_fingerprint_from_object, generate_fingerprint_with_border_overrides,
    generate_fingerprint_with_charset, generate_fingerprint_with_size,
};
use serde::Serialize;

#[derive(Serialize, Encode)]
struct Person {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

#[derive(Serialize, Encode)]
struct Config {
    debug: bool,
    max_connections: u64,
    allowed_ips: Vec<String>,
    settings: std::collections::HashMap<String, String>,
}

fn main() {
    println!("Drunken Bishop Demo");
    println!("===================");

    // Example fingerprint
    let fingerprint = "aabbccddeeff11223344556677889900";

    // Default settings
    println!("\nDefault settings:");
    println!("{}", generate_fingerprint(fingerprint));

    // Custom size
    println!("\nCustom size (6x3):");
    println!("{}", generate_fingerprint_with_size(fingerprint, 6, 3));

    // Custom charset
    println!("\nCustom charset:");
    println!(
        "{}",
        generate_fingerprint_with_charset(fingerprint, &["·", "•", "○", "●", "◐", "◑", "◒", "◓"])
    );

    // Smart border connections
    println!("\nSmart border connections:");
    println!(
        "{}",
        generate_fingerprint_with_border_overrides(
            fingerprint,
            &[("┼", "╤", "┤", "╧", "├"), ("├", "╤", "│", "╧", "╟")]
        )
    );

    // Multi-character sequences with diacritics
    println!("\nMulti-character sequences with diacritics:");
    println!(
        "{}",
        generate_fingerprint_with_charset(
            fingerprint,
            &["á", "é", "í", "ó", "ú", "ñ", "ü", "ç", "ø", "æ", "ß", "þ"]
        )
    );

    // Emoji and complex sequences
    println!("\nEmoji and complex sequences:");
    println!(
        "{}",
        generate_fingerprint_with_charset(
            fingerprint,
            &["🌟", "🎯", "🔥", "⚡", "💎", "🚀", "🌈", "🎨"]
        )
    );

    // Full custom configuration
    println!("\nFull custom config:");
    let config = DrunkenBishopConfig {
        width: 8,
        height: 5,
        charset: vec![
            "░".to_string(),
            "▒".to_string(),
            "▓".to_string(),
            "█".to_string(),
        ],
        end_char: '★',
        border: (
            "─".to_string(),
            "╮".to_string(),
            "│".to_string(),
            "╯".to_string(),
            "─".to_string(),
            "╰".to_string(),
            "│".to_string(),
            "╭".to_string(),
        ),
        border_overrides: vec![
            (
                "░".to_string(),
                "╤".to_string(),
                "┤".to_string(),
                "╧".to_string(),
                "├".to_string(),
            ),
            (
                "▒".to_string(),
                "╥".to_string(),
                "┤".to_string(),
                "╨".to_string(),
                "├".to_string(),
            ),
            (
                "▓".to_string(),
                "╦".to_string(),
                "┤".to_string(),
                "╩".to_string(),
                "├".to_string(),
            ),
            (
                "█".to_string(),
                "╬".to_string(),
                "│".to_string(),
                "╬".to_string(),
                "│".to_string(),
            ),
        ],
    };
    let bishop = DrunkenBishop::with_config(config);
    let grid = bishop.generate_grid(fingerprint);
    println!("{}", bishop.render_grid(&grid));

    // Object serialization demos
    println!("\nFrom serializable object (Person):");
    let person = Person {
        name: "Alice Johnson".to_string(),
        age: 29,
        email: "alice@example.com".to_string(),
        active: true,
    };
    if let Ok(result) = generate_fingerprint_from_object(&person) {
        println!("{}", result);
    }

    println!("\nFrom complex object (Config):");
    let mut settings = std::collections::HashMap::new();
    settings.insert("theme".to_string(), "dark".to_string());
    settings.insert("language".to_string(), "en".to_string());

    let config_obj = Config {
        debug: true,
        max_connections: 1000,
        allowed_ips: vec!["127.0.0.1".to_string(), "192.168.1.1".to_string()],
        settings,
    };
    if let Ok(result) = generate_fingerprint_from_object(&config_obj) {
        println!("{}", result);
    }

    println!("\nFrom raw bytes (UTF-8 text):");
    let text_bytes = "Hello, drunken bishop! 🍻".as_bytes();
    println!("{}", generate_fingerprint_from_bytes(text_bytes));

    println!("\nFrom raw bytes (random data):");
    let random_bytes = [0x42, 0xFF, 0x13, 0x37, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE];
    println!("{}", generate_fingerprint_from_bytes(&random_bytes));

    println!("\nFrom vector of numbers:");
    let numbers = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]; // Fibonacci
    if let Ok(result) = generate_fingerprint_from_object(&numbers) {
        println!("{}", result);
    }
}

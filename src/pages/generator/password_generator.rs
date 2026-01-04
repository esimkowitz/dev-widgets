#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaKey;
use rand::Rng;

use crate::{
    components::inputs::{NumberInput, SwitchInput, TextAreaForm},
    pages::{WidgetEntry, WidgetIcon},
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Password Generator",
    short_title: "Password",
    description: "Generate secure, customizable passwords",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<FaKey> = WidgetIcon { icon: FaKey };

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_+-=[]{}|;:,.<>?";
const AMBIGUOUS: &str = "0O1lI";

#[component]
pub fn PasswordGenerator() -> Element {
    let mut length = use_signal(|| 16usize);
    let mut use_uppercase = use_signal(|| true);
    let mut use_lowercase = use_signal(|| true);
    let mut use_numbers = use_signal(|| true);
    let mut use_symbols = use_signal(|| true);
    let mut exclude_ambiguous = use_signal(|| false);
    let mut quantity = use_signal(|| 1usize);
    let mut passwords = use_signal(Vec::<String>::new);

    let generate_passwords = move |_| {
        let mut charset = String::new();

        if *use_uppercase.read() {
            charset.push_str(UPPERCASE);
        }
        if *use_lowercase.read() {
            charset.push_str(LOWERCASE);
        }
        if *use_numbers.read() {
            charset.push_str(NUMBERS);
        }
        if *use_symbols.read() {
            charset.push_str(SYMBOLS);
        }

        if *exclude_ambiguous.read() {
            charset = charset
                .chars()
                .filter(|c| !AMBIGUOUS.contains(*c))
                .collect();
        }

        if charset.is_empty() {
            return;
        }

        let charset_chars: Vec<char> = charset.chars().collect();
        let mut rng = rand::thread_rng();
        let mut new_passwords = Vec::new();

        for _ in 0..*quantity.read() {
            let password: String = (0..*length.read())
                .map(|_| charset_chars[rng.gen_range(0..charset_chars.len())])
                .collect();
            new_passwords.push(password);
        }

        passwords.write().append(&mut new_passwords);
    };

    // Calculate entropy
    let charset_size = {
        let mut size = 0usize;
        if *use_uppercase.read() {
            size += 26;
        }
        if *use_lowercase.read() {
            size += 26;
        }
        if *use_numbers.read() {
            size += 10;
        }
        if *use_symbols.read() {
            size += SYMBOLS.len();
        }
        if *exclude_ambiguous.read() && size > 0 {
            size = size.saturating_sub(5); // Approximate ambiguous chars removed
        }
        size
    };
    let entropy = if charset_size > 0 {
        (*length.read() as f64) * (charset_size as f64).log2()
    } else {
        0.0
    };
    let entropy_label = if entropy >= 128.0 {
        "Very Strong"
    } else if entropy >= 80.0 {
        "Strong"
    } else if entropy >= 60.0 {
        "Moderate"
    } else if entropy >= 40.0 {
        "Weak"
    } else {
        "Very Weak"
    };

    let passwords_str = passwords.with(|p| p.join("\n"));

    rsx! {
        div { class: "password-generator",
            div { class: "params",
                NumberInput::<usize> {
                    label: "Password Length",
                    value: *length.read(),
                    onchange: move |value: usize| {
                        length.set(value.clamp(4, 128));
                    },
                }
                NumberInput::<usize> {
                    label: "Number of Passwords",
                    value: *quantity.read(),
                    onchange: move |value: usize| {
                        quantity.set(value.clamp(1, 100));
                    },
                }
                div { class: "switches",
                    SwitchInput {
                        label: "Uppercase (A-Z)",
                        checked: *use_uppercase.read(),
                        oninput: move |value| use_uppercase.set(value),
                    }
                    SwitchInput {
                        label: "Lowercase (a-z)",
                        checked: *use_lowercase.read(),
                        oninput: move |value| use_lowercase.set(value),
                    }
                    SwitchInput {
                        label: "Numbers (0-9)",
                        checked: *use_numbers.read(),
                        oninput: move |value| use_numbers.set(value),
                    }
                    SwitchInput {
                        label: "Symbols (!@#...)",
                        checked: *use_symbols.read(),
                        oninput: move |value| use_symbols.set(value),
                    }
                    SwitchInput {
                        label: "Exclude Ambiguous (0O1lI)",
                        checked: *exclude_ambiguous.read(),
                        oninput: move |value| exclude_ambiguous.set(value),
                    }
                }
            }

            div { class: "entropy-display", "Entropy: {entropy:.0} bits ({entropy_label})" }

            div { class: "buttons",
                button { class: "btn btn-info me-3", onclick: generate_passwords, "Generate" }
                button {
                    class: "btn btn-error",
                    onclick: move |_| passwords.write().clear(),
                    "Clear"
                }
            }

            TextAreaForm {
                label: "Generated Passwords",
                value: "{passwords_str}",
                readonly: true,
            }
        }
    }
}

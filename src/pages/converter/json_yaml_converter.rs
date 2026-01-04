#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaFileLines;
use std::fmt;

use crate::components::inputs::TextAreaForm;
use crate::pages::{WidgetEntry, WidgetIcon};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "JSON <> YAML Converter",
    short_title: "JSON <> YAML",
    description: "Convert between JSON and YAML file formats",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<FaFileLines> = WidgetIcon { icon: FaFileLines };

pub fn JsonYamlConverter() -> Element {
    use_context_provider(|| {
        Signal::new(ConverterValue {
            json_value: String::new(),
            yaml_value: String::new(),
        })
    });
    rsx! {
        div { class: "widget",
            converter_input { direction: Direction::Json }
            converter_input { direction: Direction::Yaml }
        }
    }
}

#[component]
fn converter_input(direction: Direction) -> Element {
    let mut value_context = use_context::<Signal<ConverterValue>>();

    let current_value = value_context.with(|value| match direction {
        Direction::Json => value.json_value.clone(),
        Direction::Yaml => value.yaml_value.clone(),
    });

    rsx! {
        TextAreaForm {
            label: match direction {
                Direction::Json => "JSON",
                Direction::Yaml => "YAML",
            },
            value: "{current_value}",
            oninput: move |event: Event<FormData>| {
                let input_value = event.value();
                match direction {
                    Direction::Json => {
                        let yaml_result = convert_json_to_yaml(&input_value);
                        value_context
                            .set(ConverterValue {
                                json_value: input_value,
                                yaml_value: yaml_result,
                            });
                    }
                    Direction::Yaml => {
                        let json_result = convert_yaml_to_json(&input_value);
                        value_context
                            .set(ConverterValue {
                                json_value: json_result,
                                yaml_value: input_value,
                            });
                    }
                };
            },
        }
    }
}

fn convert_json_to_yaml(json_str: &str) -> String {
    if json_str.trim().is_empty() {
        return String::new();
    }
    match serde_json::from_str::<serde_json::Value>(json_str) {
        Ok(value) => match serde_yaml::to_string(&value) {
            Ok(yaml) => yaml,
            Err(e) => format!("Error converting to YAML: {}", e),
        },
        Err(e) => format!("Invalid JSON: {}", e),
    }
}

fn convert_yaml_to_json(yaml_str: &str) -> String {
    if yaml_str.trim().is_empty() {
        return String::new();
    }
    match serde_yaml::from_str::<serde_yaml::Value>(yaml_str) {
        Ok(value) => match serde_json::to_string_pretty(&value) {
            Ok(json) => json,
            Err(e) => format!("Error converting to JSON: {}", e),
        },
        Err(e) => format!("Invalid YAML: {}", e),
    }
}

#[derive(Clone)]
struct ConverterValue {
    json_value: String,
    yaml_value: String,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Json,
    Yaml,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

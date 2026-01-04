#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons::FaAlignLeft;
use rand::thread_rng;
use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

use crate::{
    components::inputs::{NumberInput, SelectForm, SelectFormEnum, SwitchInput, TextAreaForm},
    pages::{WidgetEntry, WidgetIcon},
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Lorem Ipsum Generator",
    short_title: "Lorem Ipsum",
    description: "Generate placeholder text",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<FaAlignLeft> = WidgetIcon { icon: FaAlignLeft };

#[derive(
    Copy, Clone, Default, Debug, Display, EnumIter, EnumString, Hash, IntoStaticStr, PartialEq,
)]
enum LoremMode {
    #[default]
    Paragraphs,
    Sentences,
    Words,
}

impl SelectFormEnum for LoremMode {}

impl From<LoremMode> for String {
    fn from(mode: LoremMode) -> Self {
        mode.to_string()
    }
}

#[component]
pub fn LoremIpsum() -> Element {
    let mut mode = use_signal(|| LoremMode::Paragraphs);
    let mut count = use_signal(|| 3usize);
    let mut start_with_lorem = use_signal(|| true);
    let mut generated_text = use_signal(String::new);

    let generate = move |_| {
        let mode_val = *mode.read();
        let count_val = *count.read();
        let start_lorem = *start_with_lorem.read();
        let mut rng = thread_rng();

        let text = match mode_val {
            LoremMode::Paragraphs => {
                let paragraphs: Vec<String> = (0..count_val)
                    .map(|i| {
                        if i == 0 && start_lorem {
                            lipsum::lipsum_with_rng(&mut rng, 50)
                        } else {
                            lipsum::lipsum_words_with_rng(&mut rng, 50)
                        }
                    })
                    .collect();
                paragraphs.join("\n\n")
            }
            LoremMode::Sentences => {
                let word_count = count_val * 12;
                let text = if start_lorem {
                    lipsum::lipsum_with_rng(&mut rng, word_count)
                } else {
                    lipsum::lipsum_words_with_rng(&mut rng, word_count)
                };
                // Split into sentences and take the requested count
                let sentences: Vec<&str> = text.split(". ").take(count_val).collect();
                let mut result = sentences.join(". ");
                if !result.ends_with('.') {
                    result.push('.');
                }
                result
            }
            LoremMode::Words => {
                if start_lorem {
                    lipsum::lipsum_with_rng(&mut rng, count_val)
                } else {
                    lipsum::lipsum_words_with_rng(&mut rng, count_val)
                }
            }
        };
        generated_text.set(text);
    };

    rsx! {
        div { class: "widget",
            div { class: "widget-params",
                SelectForm::<LoremMode> {
                    label: "Mode",
                    value: *mode.read(),
                    oninput: move |value| mode.set(value),
                }
                NumberInput::<usize> {
                    label: "Count",
                    value: *count.read(),
                    onchange: move |value: usize| count.set(value.clamp(1, 50)),
                }
                div { class: "widget-buttons",
                    button {
                        class: "btn btn-info",
                        onclick: generate,
                        "Generate"
                    }
                    button {
                        class: "btn btn-error",
                        onclick: move |_| generated_text.set(String::new()),
                        "Clear"
                    }
                }
                div { class: "widget-switches",
                    SwitchInput {
                        label: "Start with \"Lorem ipsum...\"",
                        checked: *start_with_lorem.read(),
                        oninput: move |value| start_with_lorem.set(value),
                    }
                }
            }

            TextAreaForm {
                label: "Generated Text",
                value: "{generated_text}",
                readonly: true,
            }
        }
    }
}

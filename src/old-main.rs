use iced::widget::{Column, container, column, row, text, text_input};
use iced::{Element, Length, Sandbox, Settings};

use base64::{Engine as _, engine::{general_purpose}};

pub fn main() -> iced::Result {
    WidgetView::run(Settings::default())
}

struct WidgetView {
    widgets: Widgets,
}

impl Sandbox for WidgetView {
    type Message = Message;

    fn new() -> WidgetView {
        WidgetView {
            widgets: Widgets::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Widgets")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::WidgetMessage(widget_message) => {
                self.widgets.update(widget_message);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let WidgetView { widgets } = self;
        let content: Element<_> = column![
            widgets.view().map(Message::WidgetMessage),
        ]
        .max_width(540)
        .spacing(20)
        .padding(20)
        .into();

        container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
    }
}

#[derive(Debug, Clone)]
enum Message {
    WidgetMessage(WidgetMessage),
}

struct Widgets {
    widgets: Vec<Widget>,
}

impl Widgets {
    fn new() -> Self {
        Self {
            widgets: vec![
                Widget::NumberBaseConverter { value: 0 },
                Widget::Base64Converter { encoded_value: String::new(), decoded_value: String::new() },
            ],
        }
    }

    fn update(&mut self, message: WidgetMessage) {
        for widget in &mut self.widgets {
            widget.update(message.clone());
        }
    }

    fn view(&self) -> Element<WidgetMessage> {
        let mut widgets = Column::new();
        for widget in &self.widgets {
            widgets = widgets.push(widget.view());
        }
        widgets.into()
    }
}

enum Widget {
    NumberBaseConverter{
        value: i64,
    },
    Base64Converter {
        encoded_value: String,
        decoded_value: String,
    }
}

#[derive(Debug, Clone)]
enum WidgetMessage {
    NumberBaseConverterChanged(NumberBase),
    Base64ConverterChanged(Base64ConverterDirection),
}

impl<'a> Widget {
    fn container(title: &str) -> Column<'a, WidgetMessage> {
        column![text(title).size(50)].spacing(20)
    }

    fn update(&mut self, message: WidgetMessage) {
        match message {
            WidgetMessage::NumberBaseConverterChanged(number_base) => {
                if let Widget::NumberBaseConverter { value } = self {
                    fn parse_value(value: String, radix: u32) -> i64 {
                        match i64::from_str_radix(&value, radix) {
                            Ok(value) => value,
                            Err(_) => 0,
                        }
                    }

                    match number_base {
                        NumberBase::Binary(input_value) => {
                            *value = parse_value(input_value, 2);
                        },
                        NumberBase::Decimal(input_value) => {
                            *value = parse_value(input_value, 10);
                        },
                        NumberBase::Hexadecimal(input_value) => {
                            *value = parse_value(input_value, 16);
                        },
                        NumberBase::Octal(input_value) => {
                            *value = parse_value(input_value, 8);
                        },
                    }
                }
            },
            WidgetMessage::Base64ConverterChanged(input_value) => {
                if let Widget::Base64Converter { encoded_value, decoded_value } = self {
                    match input_value {
                        Base64ConverterDirection::Encode(input_value) => {
                            *decoded_value = input_value;
                            *encoded_value = general_purpose::STANDARD.encode(decoded_value);
                        },
                        Base64ConverterDirection::Decode(input_value) => {
                            *encoded_value = input_value;
                            let decode_val = general_purpose::STANDARD.decode(encoded_value).unwrap_or_default();
                            *decoded_value = String::from_utf8(decode_val).unwrap_or_default();
                        },
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<WidgetMessage> {
        match self {
            Widget::NumberBaseConverter { value } => Self::number_base_converter(*value),
            Widget::Base64Converter { encoded_value, decoded_value } => Self::base64_converter(encoded_value, decoded_value),
        }
        .into()
    }

    fn number_base_converter(
        value: i64,
    ) -> Column<'a, WidgetMessage> {
        let decimal_text_input = 
            text_input("0", format!("{}", value).as_str())
            .on_input(|value| WidgetMessage::NumberBaseConverterChanged(NumberBase::Decimal(value)));
        let binary_text_input = 
            text_input("0", format!("{:b}", value).as_str())
            .on_input(|value| WidgetMessage::NumberBaseConverterChanged(NumberBase::Binary(value)));
        let hexadecimal_text_input = 
            text_input("0", format!("{:x}", value).as_str())
            .on_input(|value| WidgetMessage::NumberBaseConverterChanged(NumberBase::Hexadecimal(value)));
        let octal_text_input = 
            text_input("0", format!("{:o}", value).as_str())
            .on_input(|value| WidgetMessage::NumberBaseConverterChanged(NumberBase::Octal(value)));

        let decimal_row = row![text("Decimal"), decimal_text_input];
        let binary_row = row![text("Binary"), binary_text_input];
        let hexadecimal_row = row![text("Hexadecimal"), hexadecimal_text_input];
        let octal_row = row![text("Octal"), octal_text_input];
        let layout_section: Element<_> = 
            column![decimal_row, hexadecimal_row, binary_row, octal_row].into();

        Self::container("Number base converter")
            .push(layout_section)
    }

    fn base64_converter(
        encoded_value: &String,
        decoded_value: &String
    ) -> Column<'a, WidgetMessage> {
        let base64_encode_input = 
            text_input("Encode", &decoded_value.as_str())
            .on_input(|value| WidgetMessage::Base64ConverterChanged(Base64ConverterDirection::Encode(value)));

        let base64_decode_input = 
            text_input("Decode", &encoded_value.as_str())
            .on_input(|value| WidgetMessage::Base64ConverterChanged(Base64ConverterDirection::Decode(value)));

        let base64_encode_row = row![text("Encode"), base64_encode_input];
        let base64_decode_row = row![text("Decode"), base64_decode_input];

        let layout_section: Element<_> = 
            column![base64_encode_row, base64_decode_row].into();
        
        Self::container("Base64 converter")
            .push(layout_section)
    }
}

#[derive(Debug, Clone)]
enum Base64ConverterDirection {
    Encode(String),
    Decode(String),
}

#[derive(Debug, Clone)]
enum NumberBase {
    Binary(String),
    Decimal(String),
    Hexadecimal(String),
    Octal(String),
}
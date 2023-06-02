use iced::widget::{Column, container, column, row, text, text_input};
use iced::{Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    WidgetView::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    WidgetMessage(WidgetMessage),
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

struct Widgets {
    widgets: Vec<Widget>,
}

impl Widgets {
    fn new() -> Self {
        Self {
            widgets: vec![Widget::NumberBaseConverter { value: 0 }],
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

#[derive(Debug, Clone)]
enum WidgetMessage {
    NumberBaseConverterChanged(NumberBase),
}

enum Widget {
    NumberBaseConverter{
        value: i64,
    }
}

impl<'a> Widget {
    fn container(title: &str) -> Column<'a, WidgetMessage> {
        column![text(title).size(50)].spacing(20)
    }

    fn update(&mut self, message: WidgetMessage) {
        match message {
            WidgetMessage::NumberBaseConverterChanged(number_base) => {
                let Widget::NumberBaseConverter { value } = self;

                fn parse_value(value: String, base: u32) -> i64 {
                    match i64::from_str_radix(&value, base) {
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
        }
    }

    fn view(&self) -> Element<WidgetMessage> {
        match self {
            Widget::NumberBaseConverter { value } => Self::number_base_converter(*value),
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
}

#[derive(Debug, Clone)]
enum NumberBase {
    Binary(String),
    Decimal(String),
    Hexadecimal(String),
    Octal(String),
}
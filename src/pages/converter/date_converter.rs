#![allow(non_snake_case)]
use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsClock;
use strum::IntoEnumIterator;
use time::{Month, OffsetDateTime, UtcOffset};
use time_tz::{system, timezones, OffsetDateTimeExt, TimeZone, Tz};

use crate::{
    components::inputs::{NumberInput, SelectForm, SelectFormEnum, TextInput},
    pages::{WidgetEntry, WidgetIcon},
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Date Converter",
    short_title: "Date",
    description: "Convert dates between formats",
    icon: move || ICON.icon(),
};

const ICON: WidgetIcon<BsClock> = WidgetIcon { icon: BsClock };

pub fn DateConverter() -> Element {
    let mut date_signal = use_signal(|| DateConverterState {
        time_zone: DcTimeZone::default(),
        time_utc: OffsetDateTime::now_utc(),
    });

    let local_datetime = date_signal.with(|date_state| date_state.local_datetime());
    let unix_time = date_signal.with(|date_state| date_state.time_utc.unix_timestamp());

    rsx! {
        div {
            class: "date-converter",
            SelectForm::<DcTimeZone> {
                label: "Time Zone",
                oninput: move |tz: DcTimeZone| {
                    date_signal.with_mut(|date_state| {
                        date_state.time_zone = tz;
                    });
                },
                value: date_signal.with(|date_state| date_state.time_zone),
            }
            TextInput {
                label: "Date",
                value: "{local_datetime}",
                readonly: true,
            }
            TextInput {
                label: "Unix Timestamp",
                value: "{unix_time}",
                onchange: move |event: Event<FormData>| {
                    if let Ok(unix_time) = event.parsed::<i64>() {
                        if let Ok(datetime) = OffsetDateTime::from_unix_timestamp(unix_time) {
                            date_signal.with_mut(|date_state| {
                                date_state.set_local_datetime(datetime);
                            });
                        }
                    }
                }
            }
            div {
                class: "selectors-wrapper",
                div {
                    class: "ymd selectors",
                    div {
                        class: "selectors-inner",
                        NumberInput::<i32> {
                            class: "year",
                            label: "Year",
                            value: local_datetime.year(),
                            onchange: move |year| {
                                date_signal.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.replace_year(year).unwrap_or(local_datetime));
                                });
                            }
                        }
                        NumberInput::<u8> {
                            class: "month",
                            label: "Month",
                            value: u8::from(local_datetime.month()),
                            onchange: move |month| {
                                date_signal.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.replace_month(Month::try_from(month).unwrap_or(local_datetime.month())).unwrap_or(local_datetime));
                                });
                            }
                        }
                        NumberInput::<u8> {
                            class: "day",
                            label: "Day",
                            value: local_datetime.day(),
                            onchange: move |day| {
                                date_signal.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.replace_day(day).unwrap_or(local_datetime));
                                });
                            }
                        }
                    }
                }
                div {
                    class: "hms selectors",
                    div {
                        class: "selectors-inner",
                        NumberInput::<u8> {
                            class: "hour",
                            label: "Hour",
                            value: local_datetime.hour(),
                            onchange: move |hour| {
                                date_signal.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.replace_hour(hour).unwrap_or(local_datetime));
                                });
                            }
                        }
                        NumberInput::<u8> {
                            class: "minute",
                            label: "Minute",
                            value: local_datetime.minute(),
                            onchange: move |minute| {
                                date_signal.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.replace_minute(minute).unwrap_or(local_datetime));
                                });
                            }
                        }
                        NumberInput::<u8> {
                            class: "second",
                            label: "Second",
                            value: local_datetime.second(),
                            onchange: move |second| {
                                date_signal.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.replace_second(second).unwrap_or(local_datetime));
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

struct DateConverterState {
    time_zone: DcTimeZone,
    time_utc: OffsetDateTime,
}

impl DateConverterState {
    fn local_datetime(&self) -> OffsetDateTime {
        self.time_utc.to_timezone(self.time_zone.inner())
    }

    fn set_local_datetime(&mut self, datetime: OffsetDateTime) {
        self.time_utc = datetime.to_offset(UtcOffset::UTC);
    }
}

#[derive(Debug, Clone, Copy, Eq)]
enum DcTimeZone {
    Base(&'static Tz),
}

impl Default for DcTimeZone {
    fn default() -> Self {
        Self::Base(match system::get_timezone() {
            Ok(tz) => tz,
            Err(err) => {
                log::warn!("Failed to get system timezone, defaulting to UTC {:?}", err);
                timezones::get_by_name("UTC").unwrap()
            },
        })
    }
}

impl Display for DcTimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner())
    }
}

impl FromStr for DcTimeZone {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match timezones::get_by_name(s) {
            Some(tz) => Ok(Self::Base(tz)),
            None => {
                log::error!("Failed to parse timezone: {}", s);
                Err(TzParseError)
            },
        }
    }

    type Err = TzParseError;
}

impl From<DcTimeZone> for String {
    fn from(val: DcTimeZone) -> Self {
        val.to_string()
    }
}

#[derive(Debug, Clone)]
struct TzParseError;

impl Display for TzParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "invalid timezone".fmt(f)
    }
}

impl PartialEq for DcTimeZone {
    fn eq(&self, other: &Self) -> bool {
        self.inner() == other.inner()
    }
}

impl IntoEnumIterator for DcTimeZone {
    fn iter() -> Self::Iterator {
        timezones::iter()
            .map(Self::Base)
            .collect::<Vec<_>>()
            .into_iter()
    }

    type Iterator = std::vec::IntoIter<Self>;
}

impl From<DcTimeZone> for &'static str {
    fn from(val: DcTimeZone) -> Self {
        val.inner().name()
    }
}

impl SelectFormEnum for DcTimeZone {}

impl DcTimeZone {
    fn inner(&self) -> &'static Tz {
        match self {
            Self::Base(tz) => tz,
        }
    }
}

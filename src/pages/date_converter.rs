use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Timelike, Utc};
use chrono_tz::{ParseError, Tz, TZ_VARIANTS};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsClock;
use strum::IntoEnumIterator;

use crate::{
    components::inputs::{NumberInput, SelectForm, SelectFormEnum, TextInput},
    pages::{WidgetEntry, WidgetIcon},
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Date Converter",
    short_title: "Date",
    description: "Convert dates between formats",
    path: "/date-converter",
    function: date_converter,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsClock> = WidgetIcon { icon: BsClock };

pub fn date_converter(cx: Scope) -> Element {
    let date_state = use_ref(cx, || DateConverterState {
        time_zone: DcTimeZone::default(),
        time: Utc::now().naive_utc(),
    });

    let local_datetime = date_state.with(|date_state| date_state.local_datetime());
    let unix_time = date_state.with(|date_state| date_state.time.timestamp());

    cx.render(rsx! {
        div {
            class: "date-converter",
            SelectForm::<DcTimeZone> {
                label: "Time Zone",
                oninput: move |tz: DcTimeZone| {
                    date_state.with_mut(|date_state| {
                        date_state.time_zone = tz;
                    });
                },
                value: date_state.with(|date_state| date_state.time_zone),
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
                    let new_unix_time = event.value.clone();
                    if let Ok(unix_time) = i64::from_str(&new_unix_time) {
                        if let chrono::LocalResult::Single(datetime) = Utc.timestamp_opt(unix_time, 0) {
                            date_state.with_mut(|date_state| {
                                date_state.time = datetime.naive_utc();
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
                                date_state.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.with_year(year).unwrap_or(local_datetime));
                                });
                            }
                        }
                        NumberInput::<u32> {
                            class: "month",
                            label: "Month",
                            value: local_datetime.month(),
                            onchange: move |month| {
                                date_state.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.with_month(month).unwrap_or(local_datetime));
                                });
                            }
                        }
                        NumberInput::<u32> {
                            class: "day",
                            label: "Day",
                            value: local_datetime.day(),
                            onchange: move |day| {
                                date_state.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.with_day(day).unwrap_or(local_datetime));
                                });
                            }
                        }
                    }
                }
                div {
                    class: "hms selectors",
                    div {
                        class: "selectors-inner",
                        NumberInput::<u32> {
                            class: "hour",
                            label: "Hour",
                            value: local_datetime.hour(),
                            onchange: move |hour| {
                                date_state.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.with_hour(hour).unwrap_or(local_datetime));
                                });
                            }
                        }
                        NumberInput::<u32> {
                            class: "minute",
                            label: "Minute",
                            value: local_datetime.minute(),
                            onchange: move |minute| {
                                date_state.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.with_minute(minute).unwrap_or(local_datetime));
                                });
                            }
                        }
                        NumberInput::<u32> {
                            class: "second",
                            label: "Second",
                            value: local_datetime.second(),
                            onchange: move |second| {
                                date_state.with_mut(|date_state| {
                                    date_state.set_local_datetime(local_datetime.with_second(second).unwrap_or(local_datetime));
                                });
                            }
                        }
                    }
                }
            }
        }
    })
}

#[derive(Clone, Copy)]
struct DateConverterState {
    time_zone: DcTimeZone,
    time: NaiveDateTime,
}

impl DateConverterState {
    fn local_datetime(&self) -> DateTime<Tz> {
        self.time_zone.inner().from_utc_datetime(&self.time)
    }

    fn set_local_datetime(&mut self, datetime: DateTime<Tz>) {
        self.time = datetime.naive_utc();
    }
}

#[derive(Copy, Clone, Debug)]
enum DcTimeZone {
    Base(Tz),
}

impl Default for DcTimeZone {
    fn default() -> Self {
        Self::Base(Tz::UTC)
    }
}

impl Display for DcTimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl FromStr for DcTimeZone {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Base(Tz::from_str(s)?))
    }

    type Err = ParseError;
}

impl PartialEq for DcTimeZone {
    fn eq(&self, other: &Self) -> bool {
        self.inner() == other.inner()
    }
}

impl IntoEnumIterator for DcTimeZone {
    fn iter() -> Self::Iterator {
        TZ_VARIANTS
            .iter()
            .map(|tz| Self::Base(*tz))
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
    fn inner(&self) -> Tz {
        match self {
            Self::Base(tz) => *tz,
        }
    }
}

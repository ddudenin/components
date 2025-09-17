//! Defines the [`DatePicker`] component and its subcomponents, which allowing users to enter or select a date value

use crate::{
    calendar::{
        Calendar, CalendarGrid, CalendarHeader, CalendarNavigation, CalendarNextMonthButton,
        CalendarPreviousMonthButton, CalendarSelectMonth, CalendarSelectYear,
    },
    popover::{PopoverContent, PopoverRoot, PopoverTrigger},
    use_effect,
};

use dioxus::prelude::*;
use time::{macros::format_description, Date, UtcDateTime};

/// The value of the [`DatePicker`] component.
#[derive(Copy, Clone)]
pub struct DatePickerValue {
    /// A dates range value or single day
    is_range: bool,
    /// Current date value
    value: DateValue,
}

impl DatePickerValue {
    /// Create empty value for a single day
    pub fn day_none() -> Self {
        Self {
            is_range: false,
            value: DateValue::Empty,
        }
    }

    /// Create empty value for a date range
    pub fn range_none() -> Self {
        Self {
            is_range: true,
            value: DateValue::Empty,
        }
    }

    /// Create new value by given date
    pub fn new(is_range: bool, date: Date) -> Self {
        let value = if is_range {
            DateValue::Range {
                start: date,
                end: None,
            }
        } else {
            DateValue::Single { date }
        };

        Self { is_range, value }
    }

    /// Create new value by given date range
    pub fn new_range(start: Date, end: Date) -> Self {
        Self {
            is_range: true,
            value: DateValue::Range {
                start,
                end: Some(end),
            },
        }
    }

    /// Has initial value
    pub fn is_empty(&self) -> bool {
        self.value == DateValue::Empty
    }

    fn set_date(&self, date: Option<Date>) -> Self {
        match date {
            Some(date) => match self.value {
                DateValue::Range { start, end } => {
                    if end.is_some() {
                        Self::new(self.is_range, date)
                    } else {
                        let value = if date < start {
                            DateValue::Range {
                                start: date,
                                end: Some(start),
                            }
                        } else {
                            DateValue::Range {
                                start,
                                end: Some(date),
                            }
                        };
                        Self { value, ..*self }
                    }                    
                }
                _ => Self::new(self.is_range, date),
            },
            None => Self {
                value: DateValue::Empty,
                ..*self
            },
        }
    }

    fn date(&self) -> Option<Date> {
        match self.value {
            DateValue::Single { date } => Some(date),
            DateValue::Range { start, end } => {
                if end.is_some() {
                    return end;
                }

                Some(start)
            }
            DateValue::Empty => None,
        }
    }

    fn is_selected(&self) -> bool {
        match self.value {
            DateValue::Range { end, .. } => end.is_some(),
            _ => true,
        }
    }

    fn placeholder(&self, sep: &'static str) -> String {
        let capacity = if self.is_range { 2 } else { 1 };
        vec!["YYYY-MM-DD"; capacity].join(sep)
    }
}

impl std::fmt::Display for DatePickerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// The value type of the [`DatePicker`] component.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DateValue {
    /// A single value for the date picker
    Single {
        /// The selected date
        date: Date,
    },
    /// A dates range value for the date picker
    Range {
        /// The first range date
        start: Date,
        /// The last range date
        end: Option<Date>,
    },
    /// None value for the date picker
    Empty,
}

impl std::fmt::Display for DateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateValue::Single { date } => write!(f, "{date}"),
            DateValue::Range { start, end } => {
                let end_str = match end {
                    Some(date) => date.to_string(),
                    None => String::default(),
                };
                write!(f, "{start} - {end_str}")
            }
            DateValue::Empty => write!(f, ""),
        }
    }
}

/// The context provided by the [`DatePicker`] component to its children.
#[derive(Copy, Clone)]
struct DatePickerContext {
    // State
    value: ReadOnlySignal<DatePickerValue>,
    on_value_change: Callback<DatePickerValue>,
    open: Signal<bool>,

    // Configuration
    disabled: ReadOnlySignal<bool>,
    separator: &'static str,
}

impl DatePickerContext {
    fn set_date(&mut self, date: Option<Date>) {
        let value = (self.value)().set_date(date);
        self.on_value_change.call(value)
    }
}

/// The props for the [`DatePicker`] component.
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// The controlled value of the date picker
    pub value: ReadOnlySignal<DatePickerValue>,

    /// Callback when value changes
    #[props(default)]
    pub on_value_change: Callback<DatePickerValue>,

    /// The selected date
    #[props(default)]
    pub selected_date: Signal<Option<Date>>,

    /// Whether the date picker is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Separator between range value
    #[props(default = " - ")]
    pub separator: &'static str,

    /// Additional attributes to extend the date picker element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the date picker element
    pub children: Element,
}

/// # DatePicker
///
/// The [`DatePicker`] component provides an accessible date input interface.
///
/// ## Example
/// ```rust
/// ```
///
/// # Styling
///
/// The [`DatePicker`] component defines the following data attributes you can use to control styling:
/// - `data-disabled`: Indicates if the DatePicker is disabled. Possible values are `true` or `false`.
#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let open = use_signal(|| false);

    // Create context provider for child components
    use_context_provider(|| DatePickerContext {
        open,
        value: props.value,
        on_value_change: props.on_value_change,
        disabled: props.disabled,
        separator: props.separator,
    });

    rsx! {
        div {
            role: "application",
            "aria-label": "DatePicker",
            "data-disabled": (props.disabled)(),
            ..props.attributes,
            {props.children}
        }
    }
}

/// The props for the [`SelectDateTrigger`] component
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerTriggerProps {
    /// Additional attributes for the trigger button
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children to render inside the trigger
    pub children: Element,
}

/// # DatePickerTrigger
/// 
/// The `PopoverTrigger` is a button that toggles the visibility of the [`PopoverContent`].
///
/// ```rust
/// ```
#[component]
pub fn DatePickerTrigger(props: DatePickerTriggerProps) -> Element {
    let mut ctx = use_context::<DatePickerContext>();
    let mut open = ctx.open;

    let mut selected_date = use_signal(|| None::<Date>);
    let mut view_date = use_signal(|| UtcDateTime::now().date());

    use_effect(move || {
        let value = (ctx.value)();
        let date = value.date();

        selected_date.set(date);
        if let Some(date) = date {
            view_date.set(date)
        }

        if value.is_selected() {
            open.set(false);
        }
    });

    rsx! {
        PopoverRoot {
            class: "popover",
            open: open(),
            on_open_change: move |v| open.set(v),
            PopoverTrigger {
                attributes: props.attributes,
                {props.children}
            }
            PopoverContent {
                gap: "0.25rem",
                Calendar {
                    selected_date: selected_date(),
                    on_date_change: move |date| ctx.set_date(date),
                    view_date: view_date(),
                    on_view_change: move |new_view: Date| view_date.set(new_view),
                    CalendarHeader {
                        CalendarNavigation {
                            CalendarPreviousMonthButton {
                                svg {
                                    class: "calendar-previous-month-icon",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    polyline { points: "15 6 9 12 15 18" }
                                }
                            }
                            CalendarSelectMonth { class: "calendar-month-select" }
                            CalendarSelectYear { class: "calendar-year-select" }
                            CalendarNextMonthButton {
                                svg {
                                    class: "calendar-next-month-icon",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    polyline { points: "9 18 15 12 9 6" }
                                }
                            }
                        }
                    }
                    CalendarGrid {}
                }
            }
        }
    }
}

/// The props for the [`DatePickerInput`] component
#[derive(Props, Clone, PartialEq)]
pub struct DatePickerInputProps {
    /// Additional attributes for the value element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the date picker element
    pub children: Element,
}

/// # DatePickerInput
///
/// The input element for the [`DatePicker`](super::date_picker::DatePicker) component which allow users to enter a date value.
///
/// ```rust
/// ```
#[component]
pub fn DatePickerInput(props: DatePickerInputProps) -> Element {
    let mut ctx = use_context::<DatePickerContext>();

    let display_value = use_memo(move || ctx.value.to_string());

    rsx! {
        input {
            style: "min-width: 240px",
            placeholder: (ctx.value)().placeholder(ctx.separator),
            value: display_value,
            oninput: move |e| {
                let text = e.value().parse().unwrap_or(display_value());

                let format = format_description!("[year]-[month]-[day]");
                let parts = text.split(ctx.separator);
                for (index, str) in parts.enumerate() {
                    if (ctx.value)().is_empty() || index > 0 {
                        if let Ok(date) = Date::parse(str, &format) {
                            ctx.set_date(Some(date));
                        }
                    }
                }
            },
            ..props.attributes,
        }
        {props.children}
    }
}

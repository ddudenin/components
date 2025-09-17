//! Defines the [`DatePicker`] component and its subcomponents, which allowing users to enter or select a date value

use crate::{
    popover::{PopoverContent, PopoverRoot, PopoverTrigger},
    ContentAlign,
};

use crate::dioxus_elements::input_data::MouseButton;
use dioxus::prelude::*;
use time::{macros::format_description, Date};

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
        let value = if end < start {
            DateValue::Range {
                start: end,
                end: Some(start),
            }
        } else {
            DateValue::Range {
                start,
                end: Some(end),
            }
        };
        Self {
            is_range: true,
            value,
        }
    }

    fn is_empty(&self) -> bool {
        self.value == DateValue::Empty
    }

    fn is_range(&self) -> bool {
        self.is_range
    }

    fn set_date(&self, date: Option<Date>) -> Self {
        match date {
            Some(date) => match self.value {
                DateValue::Range { start, end } => {
                    if end.is_some() {
                        Self::new(self.is_range, date)
                    } else {
                        Self::new_range(start, date)
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

    fn is_equal_date(&self, d: Option<Date>) -> bool {
        match d {
            Some(d) => match self.value {
                DateValue::Single { date } => date == d,
                DateValue::Range { start, end } => {
                    if let Some(date) = end {
                        if date == d {
                            return true;
                        }
                    }

                    start == d
                }
                DateValue::Empty => false,
            },
            None => self.value == DateValue::Empty,
        }
    }

    fn is_selected(&self) -> bool {
        match self.value {
            DateValue::Range { end, .. } => end.is_some(),
            _ => true,
        }
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
    selected_date: ReadOnlySignal<Option<Date>>,
    open: Signal<bool>,
    read_only: ReadOnlySignal<bool>,

    // Configuration
    disabled: ReadOnlySignal<bool>,
    separator: &'static str,
    format_placeholder: Callback<(), String>,
}

impl DatePickerContext {
    fn set_date(&mut self, date: Option<Date>) {
        let value = (self.value)();
        if value.is_equal_date(date) {
            return;
        }

        let value = value.set_date(date);
        self.on_value_change.call(value);

        if value.is_selected() {
            self.open.set(false);
        }
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
    pub selected_date: ReadOnlySignal<Option<Date>>,

    /// Whether the date picker is disabled
    #[props(default)]
    pub disabled: ReadOnlySignal<bool>,

    /// Whether the date picker is enable user input
    #[props(default = ReadOnlySignal::new(Signal::new(false)))]
    pub read_only: ReadOnlySignal<bool>,

    /// Separator between range value
    #[props(default = " - ")]
    pub separator: &'static str,

    /// Callback when display placeholder
    #[props(default = Callback::new(|_| "YYYY-MM-DD".to_string()))]
    pub on_format_placeholder: Callback<(), String>,

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
        selected_date: props.selected_date,
        disabled: props.disabled,
        read_only: props.read_only,
        separator: props.separator,
        format_placeholder: props.on_format_placeholder,
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

    use_effect(move || {
        let date = (ctx.selected_date)();
        ctx.set_date(date);
    });

    rsx! {
        PopoverRoot {
            class: "popover",
            open: open(),
            on_open_change: move |v| open.set(v),
            PopoverTrigger { attributes: props.attributes,
                svg {
                    class: "date-picker-expand-icon",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }
            PopoverContent {
                gap: "0.25rem",
                align: ContentAlign::End,
                {props.children}
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

    let placeholder = {
        let is_range = (ctx.value)().is_range();
        let capacity = if is_range { 2 } else { 1 };
        let text = ctx.format_placeholder.call(());
        vec![text; capacity].join(ctx.separator)
    };

    rsx! {
        input {
            style: "min-width: 240px",
            placeholder,
            value: display_value,
            disabled: ctx.disabled,
            readonly: ctx.read_only,
            cursor: if (ctx.read_only)() { "pointer" } else { "text" },
            oninput: move |e| {
                let text = e.value().parse().unwrap_or(display_value());
                let format = format_description!("[year]-[month]-[day]");
                let parts = text.split(ctx.separator);
                for (index, str) in parts.enumerate() {
                    if (ctx.value)().is_empty() || index > 0 {
                        if let Ok(date) = Date::parse(str, &format) {
                            tracing::info!("oninput: {date}");
                            ctx.set_date(Some(date));
                        }
                    }
                }
            },
            onpointerdown: move |event| {
                if (ctx.read_only)() && event.trigger_button() == Some(MouseButton::Primary) {
                    ctx.open.toggle();
                }
            },
            ..props.attributes,
        }
        {props.children}
    }
}

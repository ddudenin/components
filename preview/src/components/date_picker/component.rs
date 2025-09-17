use dioxus::prelude::*;
use dioxus_primitives::date_picker::{
    self, DatePickerInputProps, DatePickerProps,
};

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/components/date_picker/style.css"),
        }
        div {
            date_picker::DatePicker {
                class: "date-picker",
                value: props.value,
                on_value_change: props.on_value_change,
                selected_date: props.selected_date,
                disabled: props.disabled,
                attributes: props.attributes,
                {props.children}
            }
        }
    }
}

#[component]
pub fn DatePickerInput(props: DatePickerInputProps) -> Element {
    rsx! {
        date_picker::DatePickerInput {
            class: "date-picker-input",
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn DatePickerTrigger(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        date_picker::DatePickerTrigger {
            class: "date-picker-trigger",
            attributes,
            svg {
                class: "date-picker-expand-icon",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

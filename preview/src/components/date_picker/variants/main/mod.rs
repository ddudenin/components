use super::super::component::*;
use dioxus::prelude::*;

use dioxus_primitives::date_picker::DatePickerValue;

#[component]
pub fn Demo() -> Element {
    let mut value = use_signal(|| DatePickerValue::range_none());

    rsx! {
        div {
            DatePicker {
                value: value(),
                on_value_change: move |v| {
                    tracing::info!("Selected: {v}");
                    value.set(v);
                },
                DatePickerInput {
                    DatePickerTrigger { aria_label: "DatePicker Trigger" }
                }
            }
        }
    }
}

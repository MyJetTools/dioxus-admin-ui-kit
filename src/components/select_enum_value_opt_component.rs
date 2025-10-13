use std::str::FromStr;

use crate::types::SelectEnumValue;
use dioxus::prelude::*;
use rust_extensions::AsStr;

const NULL_VALUE: &'static str = "---NULL---";

pub fn select_enum_value_opt<TItem: Eq + Clone + 'static + AsStr + FromStr>(
    caption: &str,
    value_selector: &SelectEnumValue<TItem>,
    on_input: EventHandler<Option<TItem>>,
) -> Element {
    let selected = value_selector.get_value();

    let items = value_selector.get_all().into_iter().map(|itm| {
        let selected = Some(itm) == selected;
        rsx! {
            option { selected, value: itm.as_str(), {itm.as_str()} }
        }
    });

    let selected_value = match selected {
        Some(value) => value.as_str(),
        None => NULL_VALUE,
    };
    rsx! {
        div { class: "edit-wrapper",
            label { {caption} }
            select {

                class: "form-select",
                onchange: move |e| {
                    let value = e.value();
                    let value = if value.as_str() == NULL_VALUE {
                        None
                    } else {
                        TItem::from_str(value.as_str()).ok()
                    };
                    on_input.call(value);
                },
                value: selected_value,
                option { value: NULL_VALUE, "Not selected" }
                {items}
            }
        }
    }
}

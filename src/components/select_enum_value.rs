use crate::types::*;
use dioxus::prelude::*;
use rust_extensions::AsStr;
use std::str::FromStr;

pub fn select_enum_value<
    TItem: Eq + Clone + Default + 'static + AsStr + FromStr,
    TEnumIterator: EnumIterator<TItem = TItem>,
>(
    caption: &str,
    value_selector: TEnumIterator,
    on_input: EventHandler<TItem>,
) -> Element {
    let selected = value_selector.get_value();

    let items = TEnumIterator::get_all().into_iter().map(|itm| {
        let selected = itm == &selected;
        rsx! {
            option { selected, value: itm.as_str(), {itm.as_str()} }
        }
    });

    rsx! {
        div { class: "edit-wrapper",
            label { {caption} }
            select {

                class: "form-select",
                onchange: move |e| {
                    let value = e.value();
                    let value = match TItem::from_str(value.as_str()).ok() {
                        Some(value) => value,
                        None => TItem::default(),
                    };
                    on_input.call(value);
                },
                value: selected.as_str(),
                {items}
            }
        }
    }
}

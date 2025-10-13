use super::*;
use rust_extensions::AsStr;

#[derive(Debug, Clone)]
pub struct SelectEnumValue<TItem: AsStr + Clone + 'static> {
    selected: Option<TItem>,
    all_options: &'static [TItem],
    allow_null_result: bool,
}

impl<TItem: AsStr + Clone + 'static> SelectEnumValue<TItem> {
    pub fn new<TEnumIterator: EnumIterator<TItem = TItem>>(item: Option<TItem>) -> Self {
        let items = TEnumIterator::get_all();
        Self {
            selected: item,
            all_options: items,
            allow_null_result: true,
        }
    }

    pub fn allow_null_result(mut self, value: bool) -> Self {
        self.allow_null_result = value;
        self
    }

    pub fn validation_ok(&self) -> bool {
        if self.allow_null_result {
            return true;
        }

        self.selected.is_some()
    }

    pub fn get_all(&self) -> &'static [TItem] {
        self.all_options
    }

    pub fn get_value(&self) -> Option<&TItem> {
        self.selected.as_ref()
    }

    pub fn set_value(&mut self, value: Option<TItem>) {
        self.selected = value;
    }
}

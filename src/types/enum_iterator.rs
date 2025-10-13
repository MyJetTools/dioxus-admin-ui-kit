use rust_extensions::AsStr;

pub trait EnumIterator {
    type TItem;

    fn get_value(&self) -> Self::TItem;
    fn get_all() -> &'static [Self::TItem];
}

#[derive(Debug, Clone)]
pub struct OptValueSelector<TItem: AsStr + Clone + 'static> {
    selected: Option<TItem>,
    all_options: &'static [TItem],
}

impl<TItem: AsStr + Clone + 'static> OptValueSelector<TItem> {
    pub fn new<TEnumIterator: EnumIterator<TItem = TItem>>(item: Option<TItem>) -> Self {
        let items = TEnumIterator::get_all();
        Self {
            selected: item,
            all_options: items,
        }
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

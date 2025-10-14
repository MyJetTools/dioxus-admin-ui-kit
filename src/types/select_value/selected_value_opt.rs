use super::*;
use rust_common::object_id::{IdExtension, ObjectId};
pub struct SelectItemsGroup<TValue: SelectValue> {
    pub name: &'static str,
    pub values: Vec<TValue>,
}

impl<TValue: SelectValue> SelectItemsGroup<TValue> {
    pub fn new(name: &'static str, values: Vec<TValue>) -> Self {
        Self { name, values }
    }
}

pub struct SelectedValueOpt<TValue: SelectValue + Clone> {
    groups: Vec<SelectItemsGroup<TValue>>,
    selected: Option<TValue>,
}

impl<TValue: SelectValue + Clone> Default for SelectedValueOpt<TValue> {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            selected: None,
        }
    }
}

impl<TValue: SelectValue + Clone> SelectedValueOpt<TValue> {
    pub fn new(groups: Vec<SelectItemsGroup<TValue>>) -> Self {
        let result = Self {
            groups,
            selected: None,
        };

        result
    }

    pub fn new_with_selected(
        groups: Vec<SelectItemsGroup<TValue>>,
        selected: Option<&impl SelectValue>,
    ) -> Self {
        let mut result = Self::new(groups);

        if let Some(selected) = selected {
            let id = selected.get_id();
            result.set_selected(Some(id));
        }

        result
    }

    pub fn set_selected(&mut self, id: Option<&str>) {
        let Some(id) = id else {
            self.selected = None;
            return;
        };

        for group in self.groups.iter() {
            for item in group.values.iter() {
                if item.get_id() == id {
                    self.selected = Some(item.clone());
                    return;
                }
            }
        }
    }

    pub fn get_selected(&self) -> Option<&TValue> {
        self.selected.as_ref()
    }

    pub fn push(&mut self, group: impl Into<SelectItemsGroup<TValue>>) {
        let group = group.into();
        self.groups.push(group);
    }

    pub fn push_item(&mut self, value: TValue) {
        if self.groups.len() == 0 {
            self.groups.push(SelectItemsGroup {
                name: "",
                values: vec![value],
            });
        } else {
            self.groups.get_mut(0).unwrap().values.push(value);
        }
    }

    pub fn get_items(&self) -> &[SelectItemsGroup<TValue>] {
        self.groups.as_slice()
    }
}

impl<TValue: SelectValue + Clone> Into<SelectedValueOpt<TValue>> for Vec<TValue> {
    fn into(self) -> SelectedValueOpt<TValue> {
        SelectedValueOpt {
            groups: vec![SelectItemsGroup {
                name: "",
                values: self,
            }],
            selected: None,
        }
    }
}

impl<TValue: SelectValue + Clone> Into<SelectItemsGroup<TValue>> for Vec<TValue> {
    fn into(self) -> SelectItemsGroup<TValue> {
        SelectItemsGroup {
            name: "",
            values: self,
        }
    }
}

impl<Id: IdExtension> SelectValue for ObjectId<Id> {
    fn get_id(&self) -> &str {
        self.as_str()
    }

    fn get_value(&self) -> &str {
        self.as_str()
    }
}

use super::TableItem;
use dioxus::prelude::*;
use std::{marker::PhantomData, rc::Rc};

pub struct RenderTable<'s, TItem: TableItem> {
    phantom: PhantomData<TItem>,
    table_classes: Vec<&'static str>,
    items: &'s [Rc<TItem>],
    wrapped_div: Option<&'static str>,
}

impl<'s, TItem: TableItem> RenderTable<'s, TItem> {
    pub fn new(items: &'s [Rc<TItem>]) -> Self {
        Self {
            phantom: Default::default(),
            table_classes: Default::default(),
            items,
            wrapped_div: Default::default(),
        }
    }

    pub fn with_wrapped_div(mut self, classes: &'static str) -> Self {
        self.wrapped_div = Some(classes);
        self
    }

    pub fn with_class(mut self, classes: &[&'static str]) -> Self {
        self.table_classes.extend_from_slice(classes);
        self
    }

    fn render_content(
        self,
        header_action: Option<Element>,
        line_action: impl Fn(&Rc<TItem>) -> Element,
    ) -> Element {
        let table_classes = self.table_classes.join(" ");

        let header = TItem::HEADER.into_iter().map(|header_name| {
            rsx! {
                th { {*header_name} }
            }
        });

        let (header_action, has_action) = if let Some(header_action) = header_action {
            let result = rsx! {

                th { {header_action} }

            };

            (result, true)
        } else {
            (rsx! {}, false)
        };

        let items = self.items.into_iter().map(|itm| {
            let items = (0..TItem::COLUMNS_AMOUNT).into_iter().map(|index| {
                let value = itm.get_value(index);

                match value {
                    crate::components::ValueToRender::AsStr(value) => {
                        rsx! {
                            td { {value} }
                        }
                    }
                    crate::components::ValueToRender::AsString(value) => {
                        rsx! {
                            td { {value} }
                        }
                    }
                    crate::components::ValueToRender::El(value) => {
                        rsx! {
                            td { {value} }
                        }
                    }
                }
            });

            if has_action {
                let item = line_action(itm);

                rsx! {
                    tr {
                        {items}
                        td { {item} }
                    }
                }
            } else {
                rsx! {
                    tr { {items} }
                }
            }
        });

        let result = rsx! {
            table { class: table_classes,
                thead {
                    tr {
                        {header}
                        {header_action}
                    }
                }

                tbody { {items} }
            }
        };

        match self.wrapped_div {
            Some(classes) => rsx! {
                div { class: classes, {result} }
            },
            None => result,
        }
    }

    pub fn render_line_with_actions(
        self,
        header_action: Element,
        line_action: impl Fn(&Rc<TItem>) -> Element,
    ) -> Element {
        self.render_content(Some(header_action), line_action)
    }

    pub fn render(self) -> Element {
        self.render_content(None, |_| rsx! {})
    }
}

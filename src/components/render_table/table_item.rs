use dioxus::core::Element;

pub trait TableItem {
    const HEADER: &'static [&'static str];

    const COLUMNS_AMOUNT: usize;

    fn get_value<'s>(&'s self, index: usize) -> ValueToRender<'s>;
}

pub enum ValueToRender<'s> {
    AsStr(&'s str),
    AsString(String),
    El(Element),
}

impl<'s> Into<ValueToRender<'s>> for &'s str {
    fn into(self) -> ValueToRender<'s> {
        ValueToRender::AsStr(self)
    }
}

impl<'s> Into<ValueToRender<'s>> for String {
    fn into(self) -> ValueToRender<'s> {
        ValueToRender::AsString(self)
    }
}

impl<'s> Into<ValueToRender<'s>> for Element {
    fn into(self) -> ValueToRender<'s> {
        ValueToRender::El(self)
    }
}

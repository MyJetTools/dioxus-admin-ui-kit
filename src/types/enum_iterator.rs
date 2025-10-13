pub trait EnumIterator {
    type TItem;

    fn get_value(&self) -> Self::TItem;
    fn get_all() -> &'static [Self::TItem];
}

pub trait SelectValue {
    fn get_id(&self) -> &str;
    fn get_value(&self) -> &str;
}

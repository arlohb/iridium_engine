pub trait Component {
    fn get_type() -> &'static str
        where Self: Sized;
    fn dyn_get_type(&self) -> &'static str;
}

impl std::fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.dyn_get_type())
    }
}

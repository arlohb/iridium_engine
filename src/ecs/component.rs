pub trait Component {
    fn get_type(&self) -> &'static str;
}

impl std::fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_type())
    }
}

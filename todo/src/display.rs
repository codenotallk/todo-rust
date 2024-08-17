pub trait DisplayMessage: 'static {
    fn show(&self, message: String);
}

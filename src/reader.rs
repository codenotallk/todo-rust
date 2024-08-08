pub trait Reader: 'static {
    fn read (&mut self) -> String;
}
use std::error::Error;

pub trait Logger {
    fn trace(&self, msg: Option<&str>);
    fn debug(&self, msg: &str);
    fn info(&self, msg: &str);
    fn warn(&self, msg: &str);
    fn error(&self, err: Box<dyn Error>);
    fn fatal(&self, err: Box<dyn Error>);
}
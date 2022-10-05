use std::io::Error;

pub trait PushStrategy {
    fn push(&self) -> Result<(), Error>;
}

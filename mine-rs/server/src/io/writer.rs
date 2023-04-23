use std::fmt::Debug;

#[derive(Debug)]
pub struct IOWriter<I: Debug> {
    pub input: I,
}

impl<I: Debug> IOWriter<I> {
    pub fn from(input: I) -> Self {
        Self { input }
    }
}

impl<I: Clone + Debug> Clone for IOWriter<I> {
    fn clone(&self) -> Self {
        Self {
            input: self.input.clone(),
        }
    }
}

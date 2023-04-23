pub struct IOReader<O> {
    pub output: O,
}

impl<O> IOReader<O> {
    pub fn from(output: O) -> Self {
        Self { output }
    }
}

impl<O: Clone> Clone for IOReader<O> {
    fn clone(&self) -> Self {
        Self {
            output: self.output.clone(),
        }
    }
}

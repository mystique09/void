use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Name<T>(String, PhantomData<T>);

impl<T> Name<T> {
    pub fn new(name: String) -> Self {
        Self(name, PhantomData)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

impl<T> From<String> for Name<T> {
    fn from(value: String) -> Self {
        Self(value, PhantomData)
    }
}
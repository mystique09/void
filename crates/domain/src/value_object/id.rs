use std::marker::PhantomData;

#[derive(Debug)]
pub struct Id<T> {
    id: i64,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    pub fn to_i64(&self) -> i64 {
        self.id
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.to_i64() == other.to_i64()
    }
}

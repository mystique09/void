use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct Id<T>(i64, PhantomData<fn() -> T>);

impl<T> Id<T> {
    pub fn new(id: i64) -> Self {
        Self(id, PhantomData)
    }

    pub fn to_i64(&self) -> i64 {
        self.0
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.to_i64() == other.to_i64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_new_id_instance() {
        let id: Id<u32> = Id::new(2);

        assert_eq!(id.to_i64(), 2);
        assert_eq!(id, Id::new(2));
    }
}

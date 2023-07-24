use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Default)]
pub struct AppContext {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl AppContext {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn store<T: Clone + 'static>(&mut self, t: T) {
        self.data.entry(TypeId::of::<T>()).or_insert(Box::new(t));
    }
}

#[derive(Default, Clone, Debug)]
pub struct Data<T> {
    t: T,
}

impl<T> Data<T> {
    pub fn new(t: T) -> Data<T> {
        Data { t }
    }
}
pub trait FromContext: Default + Clone + 'static {
    fn extract(ctx: &AppContext) -> Self;
}

impl<T: Default + Clone + 'static> FromContext for Data<T> {
    fn extract(ctx: &AppContext) -> Self {
        ctx.data
            .get(&TypeId::of::<T>())
            .and_then(|boxed| (&**boxed).downcast_ref::<T>())
            .map(|t| Data::new(t.clone()))
            .unwrap_or_default()
    }
}

use std::any::{ Any };

pub trait TxContext: Send + Sync {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl dyn TxContext + '_ {
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut::<T>()
    }
}

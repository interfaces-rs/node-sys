pub mod macros;

#[allow(unused_imports)]
use js_sys::Object;

// FIXME
pub trait Cast {
    fn cast<T>(&self) -> T
    where
        Self: Clone + Into<T>,
    {
        Into::<T>::into(self.clone())
    }
}
impl<A> Cast for A {}

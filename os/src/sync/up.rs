//! Uniprocessor interior mutability primitives
use core::cell::{Ref, RefCell, RefMut};
/// Wrap a static data structure inside it so that we are
/// able to access it without any `unsafe`.
///
/// We should only use it in uniprocessor.
///
/// In order to get mutable reference of inner data, call
/// `exclusive_access`.
pub struct UPSafeCell<T> {
    /// inner data
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in
    /// uniprocessor.
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }
    /// Panic if the data has been borrowed.
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
    /// 获取对 `T` 的只读引用。
    /// 返回一个 `Ref` 类型的引用，持有对内部数据 `T` 的只读访问权。
    pub fn access(&self) -> Ref<'_,T>{
        self.inner.borrow()
    }
}

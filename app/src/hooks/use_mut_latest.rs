use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

pub struct UseMutLatestHandle<T> {
    inner: Rc<RefCell<Rc<RefCell<T>>>>,
}

impl<T> UseMutLatestHandle<T> {
    /// Get the latest mutable ref to state or props.
    pub fn current(&self) -> Rc<RefCell<T>> {
        self.inner.borrow().clone()
    }
}

impl<T> Clone for UseMutLatestHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub fn use_mut_latest<T>(value: T) -> UseMutLatestHandle<T>
where
    T: 'static,
{
    let value_rc = Rc::new(RefCell::new(value));
    let inner = use_mut_ref(|| value_rc.clone());

    // Update the ref each render so if it changes the newest value will be saved.
    *inner.borrow_mut() = value_rc;

    UseMutLatestHandle { inner }
}



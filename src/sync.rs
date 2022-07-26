use once_cell::sync::OnceCell;
use std::{cell::Cell, error::Error, marker::PhantomData};

pub struct LazyTry<T, E = Box<dyn std::error::Error + Sync>, F = fn() -> Result<T, E>> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
    _marker: PhantomData<E>,
}

unsafe impl<T, E: Error + Sync, F: Send> Sync for LazyTry<T, E, F> where OnceCell<T>: Sync {}

impl<T, E, F: FnOnce() -> Result<T, E>> LazyTry<T, E, F> {
    pub const fn new(f: F) -> Self {
        Self {
            cell: OnceCell::new(),
            init: Cell::new(Some(f)),
            _marker: PhantomData,
        }
    }

    /// # Panic
    /// when `force()` has been called before and retured an `Err`
    pub fn force(&self) -> Result<&T, E> {
        self.cell.get_or_try_init(|| match self.init.take() {
            Some(f) => f(),
            None => panic!("Lazy instance has previously been poisoned"),
        })
    }
}

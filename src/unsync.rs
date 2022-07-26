use once_cell::unsync::OnceCell;
use std::cell::Cell;

pub struct LazyTry<T, F> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
}

pub trait FailableFn<T, E> {
    fn call(self) -> Result<T, E>;
}

impl<T, E, F: FnOnce() -> Result<T, E>> LazyTry<T, F> {
    pub const fn new(f: F) -> Self {
        Self {
            cell: OnceCell::new(),
            init: Cell::new(Some(f)),
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

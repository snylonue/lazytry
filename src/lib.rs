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

pub fn into_box_err(e: impl Error + Sync + 'static) -> Box<dyn Error + Sync> {
    Box::new(e)
}

#[cfg(test)]
mod tests {
    use std::num::{IntErrorKind, ParseIntError};

    use super::*;

    #[test]
    fn lazy_try_force() {
        let lazy: LazyTry<i32> = LazyTry::new(|| "1".parse().map_err(into_box_err));

        assert_eq!(lazy.force().unwrap(), &1);
        assert_eq!(unsafe { *lazy.force().unwrap_unchecked() }, 1);
    }

    #[test]
    fn lazy_try_force_err() {
        let lazy: LazyTry<i32, ParseIntError> = LazyTry::new(|| "a".parse());

        assert_eq!(
            *lazy.force().unwrap_err().kind(),
            IntErrorKind::InvalidDigit
        );
    }
}

use crate::error::Error;
use once_cell::unsync::OnceCell;
use std::cell::Cell;

pub type LazyTryFn<T, E> = LazyTry<T, fn() -> Result<T, E>>;

pub struct LazyTry<T, F> {
    cell: OnceCell<T>,
    init: Cell<Option<F>>,
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
    pub fn force(&self) -> Result<&T, Error<E>> {
        self.cell.get_or_try_init(|| match self.init.take() {
            Some(f) => f().map_err(Error::Failed),
            None => Err(Error::Poisoned),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::num::IntErrorKind;

    use super::*;

    #[test]
    fn lazy_try_force() {
        let lazy: LazyTry<i32, _> = LazyTry::new(|| "1".parse());

        assert_eq!(lazy.force().unwrap(), &1);
        assert_eq!(unsafe { *lazy.force().unwrap_unchecked() }, 1);
    }

    #[test]
    fn lazy_try_force_err() {
        let lazy: LazyTry<i32, _> = LazyTry::new(|| "a".parse());

        assert_eq!(
            *lazy.force().unwrap_err().into_err().unwrap().kind(),
            IntErrorKind::InvalidDigit
        );
        assert_eq!(lazy.force().unwrap_err(), Error::Poisoned);
    }
}

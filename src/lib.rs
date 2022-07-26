pub mod unsync;
pub mod sync;

use std::error::Error;

pub fn into_box_err(e: impl Error + Sync + 'static) -> Box<dyn Error + Sync> {
    Box::new(e)
}

#[cfg(test)]
mod tests {
    use std::num::IntErrorKind;

    use super::*;
    use unsync::LazyTry;

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
            *lazy.force().unwrap_err().kind(),
            IntErrorKind::InvalidDigit
        );
    }
}

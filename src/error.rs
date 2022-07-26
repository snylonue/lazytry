use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<E> {
    /// Initialization has failed already.
    Poisoned,
    /// Initialization function returns an Error.
    Failed(E),
}

impl<E> Error<E> {
    pub fn err(&self) -> Option<&E> {
        match self {
            Error::Poisoned => None,
            Error::Failed(e) => Some(e),
        }
    }

    pub fn into_err(self) -> Option<E> {
        match self {
            Error::Poisoned => None,
            Error::Failed(e) => Some(e),
        }
    }
}

impl<E: Display> Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Poisoned => f.write_str("Initialization has failed before"),
            Error::Failed(e) => write!(f, "Initialization fails: {e}"),
        }
    }
}

impl<E: std::error::Error> std::error::Error for Error<E> {}

impl<E> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Self::Failed(e)
    }
}

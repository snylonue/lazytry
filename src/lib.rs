pub mod sync;
pub mod unsync;

use std::error::Error;

pub fn into_box_err(e: impl Error + Sync + 'static) -> Box<dyn Error + Sync> {
    Box::new(e)
}

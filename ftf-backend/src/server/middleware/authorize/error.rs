use std::{error, fmt};

#[derive(Debug, Default)]
pub struct Invalid(pub(super) ());

impl Invalid {
    pub fn new() -> Self {
        Invalid(())
    }
}

impl fmt::Display for Invalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("invalid authorization token provided")
    }
}

impl error::Error for Invalid {}

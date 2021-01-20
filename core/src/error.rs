use crate::group::GroupError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WwcError {
    #[error("Group error: {0}")]
    Group(#[from] GroupError),
}

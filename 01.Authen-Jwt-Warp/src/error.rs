use thiserror::Error;
use warp::{reject};
// 1.3 Set the data model(Error)
#[derive(Error, Debug)]
pub enum Error{
    #[error("Account discontent")]
    discontent,
    #[error("JWT CreateError")]
    jwtcreationerror,

}

impl reject::Reject for Error{}
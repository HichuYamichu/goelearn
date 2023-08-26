pub mod auth;
// pub mod errors;
pub mod errors2;
pub use errors2 as errors;

pub use auth::*;
pub use errors::*;
use sea_orm::{ActiveValue::NotSet, Set};

pub fn option_to_active_value<T: Into<sea_orm::Value>>(opt: Option<T>) -> sea_orm::ActiveValue<T> {
    match opt {
        Some(v) => Set(v),
        None => NotSet,
    }
}

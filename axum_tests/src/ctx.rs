use crate::{Error, Result};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64,
}

// Constructor
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Property Accessors
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        parts
            .extensions
            .get::<Ctx>()
            .cloned()
            .ok_or(Error::AuthFailCtxNotInRequestExt)
    }
}

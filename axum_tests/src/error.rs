use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Login
    LoginFail,

    // -- Auth errors.
    AuthFailNoAuthTokenCookie,
    AuthFailtokenwrongFormat,
    AuthFailCtxNotInRequestExt,

    // -- Model errors.
    TicketDeleteFailIdNotFound { id: u64 },

    // --Server startup errors
    IoError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError((err))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        let status = StatusCode::INTERNAL_SERVER_ERROR;

        (status, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

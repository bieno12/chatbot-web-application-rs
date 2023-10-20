use core::prelude::rust_2015;
use std::fmt::Display;

use actix_web::{HttpResponse, body::BoxBody};

#[derive(Debug)]
pub enum Error
{
	RusqliteError(rusqlite::Error)
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		todo!()
    }
}
impl actix_web::ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let mut res = HttpResponse::new(self.status_code());

        res.set_body(BoxBody::new(format!("{self:#?}")))
    }
}

impl From<rusqlite::Error> for Error
{
    fn from(value: rusqlite::Error) -> Self {
        Error::RusqliteError(value)
    }
}



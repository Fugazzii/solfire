use std::fmt::Display;
use actix_web::{http::header, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::presenter::Presenter;

pub struct JsonPresenter;

impl Presenter for JsonPresenter {

    fn present<'a, S, E>(
        result: Result<S, E>,
        success_msg: &str,
        error_msg: &str
    ) -> HttpResponse 
    where S: Deserialize<'a> + Serialize, E: Display
    {
        match result {
            Ok(data) => Self::success::<S>(success_msg, data),
            Err(err) => Self::failure::<E>(error_msg, err)
        }
    }

    fn success<'a, T> (message: &str, data: T) -> HttpResponse 
    where T: Deserialize<'a> + serde::ser::Serialize
    {
        HttpResponse::Ok()
            .append_header(header::ContentType::json())
            .json(json!({
                "success": true,
                "message": message,
                "data": data
            }))
    }

    fn failure<T>(message: &str, error: T) -> HttpResponse 
    where T: Display
    {
        HttpResponse::InternalServerError()
            .append_header(header::ContentType::json())
            .json(json!({
                "success": false,
                "message": message,
                "error": error.to_string()
            }))
    }
}
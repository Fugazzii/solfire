use std::fmt::Display;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub trait Presenter {
    fn present<'a, S, E>(
        result: Result<S, E>,
        success_msg: &str,
        error_msg: &str
    ) -> HttpResponse 
    where S: Deserialize<'a> + Serialize, E: Display;

    fn success<'a, T> (
        message: &str,
        data: T
    ) -> HttpResponse 
    where T: Deserialize<'a> + Serialize;

    fn failure<T>(
        message: &str,
        error: T
    ) -> HttpResponse 
    where T: Display;
}

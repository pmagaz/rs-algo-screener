use actix_web::{error::Error as ActixError, error::JsonPayloadError, HttpRequest};

pub fn json_error(err: JsonPayloadError, req: &HttpRequest) -> ActixError {
    log::error!("JSON Error: {:?}", err);
    let actix_err: ActixError = err.into(); // Convert JsonPayloadError into actix_web::Error
    actix_err
}

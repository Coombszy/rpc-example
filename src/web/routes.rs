use actix_web::{
    error, get, post,
    web::{self},
    Error, HttpResponse,
};


#[get("/health")]
pub async fn health() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NoContent().finish())
}

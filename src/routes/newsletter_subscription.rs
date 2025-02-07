use actix_web::{web::Form, HttpResponse};

pub async fn newsletter_subscription(_form: Form<Newsletter>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct Newsletter {
    email: String,
    name: String,
}

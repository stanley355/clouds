use crate::db::PgPool;
use crate::host::{model, request};
use crate::lib::error::{ErrResponse, ErrType};
use actix_web::{get, post, put, web, HttpResponse};
use diesel::QueryResult;

// Filter host based on Query Param
#[get("/")]
async fn get_all_host(
    param: web::Query<request::HostFilterRequestParam>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let host_list: QueryResult<Vec<model::Host>>;
    if param.always_free.is_some()
        || param.free_tier.is_some()
        || param.frontend_support.is_some()
        || param.backend_support.is_some()
        || param.database_support.is_some()
    {
        host_list = model::Host::filter(param, pool);
    } else {
        host_list = model::Host::get_all(pool);
    }

    match host_list {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
    }
}

#[get("/names")]
async fn get_all_hostname(pool: web::Data<PgPool>) -> HttpResponse {
    match model::Host::get_all_name(pool) {
        Ok(host_names) => HttpResponse::Ok().json(host_names),
        Err(_) => ErrResponse::new_message(ErrType::BadRequest, "Host name not found".to_string()),
    }
}

#[get("/{host_name}")]
async fn get_host(path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
    let host_name = path.into_inner();
    match model::Host::get_one(host_name, pool) {
        Ok(host) => HttpResponse::Ok().json(host),
        Err(_) => ErrResponse::new_message(ErrType::BadRequest, "Host name not found".to_string()),
    }
}

#[post("/")]
async fn insert_new_host(
    body: web::Json<request::AddHostRequest>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match model::Host::add(body, pool) {
        Ok(res) => HttpResponse::Ok().body(format!("Affected Rows: {}", res)),
        Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
    }
}

#[put("/")]
async fn update_host(
    body: web::Json<request::UpdateHostRequest>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match model::Host::update(body, pool) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => ErrResponse::new(ErrType::BadRequest, e.to_string()),
    }
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(get_all_host)
        .service(get_all_hostname)
        .service(get_host)
        .service(insert_new_host)
        .service(update_host);
}

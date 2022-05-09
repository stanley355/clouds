use crate::db::PgPool;
use crate::product::request::*;
use crate::schema::products;
use crate::schema::products::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    pub id: i32,
    pub hosts_id: i32,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub product_url: Option<String>,
    pub free_tier: Option<bool>,
    pub free_trial: Option<bool>,
    pub base_price: Option<f64>,
    pub price_unit: Option<String>,
    pub price_timeunit: Option<String>,
    pub price_desc: Option<String>,
    pub multi_pricing: Option<bool>,
}

impl Product {
    pub fn add(
        host_id: i32,
        body: AddProductRequest,
        pool: web::Data<PgPool>,
    ) -> QueryResult<usize> {
        let conn = &pool.get().unwrap();
        let data = (
            &hosts_id.eq(&host_id),
            &title.eq(&body.title),
            &subtitle.eq(&body.subtitle),
            &description.eq(&body.description),
            &product_url.eq(&body.product_url),
            &free_tier.eq(&body.free_tier),
            &free_trial.eq(&body.free_trial),
            &base_price.eq(&body.base_price),
            &price_unit.eq(&body.price_unit),
            &price_timeunit.eq(&body.price_timeunit),
            &price_desc.eq(&body.price_desc),
            &multi_pricing.eq(&body.multi_pricing),
        );
        diesel::insert_into(products).values(data).execute(conn)
    }
}

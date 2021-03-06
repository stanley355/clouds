use crate::lib::types::{currency::Currency, page_data::PageData, timeunit::TimeUnit};
use crate::schema::plans;
use diesel::AsChangeset;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct AddPlanRequest {
    pub host_name: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub price_unit: Option<String>,
    pub price_timeunit: Option<TimeUnit>,
    pub price_desc: Option<String>,
    // Concurrent Build
    pub concurrent_build: Option<i32>,
    pub concurrent_build_unit: Option<String>,
    pub concurrent_build_timeunit: Option<TimeUnit>,
    pub concurrent_build_desc: Option<String>,
    // Bandwidth
    pub bandwidth: Option<i32>,
    pub bandwidth_unit: Option<String>,
    pub bandwidth_timeunit: Option<TimeUnit>,
    pub bandwidth_desc: Option<String>,
    // Build
    pub build: Option<i32>,
    pub build_unit: Option<String>,
    pub build_timeunit: Option<TimeUnit>,
    pub build_desc: Option<String>,
    // Analytic
    pub analytic: Option<bool>,
    pub analytic_price: Option<i32>,
    pub analytic_unit: Option<String>,
    pub analytic_timeunit: Option<TimeUnit>,
    pub analytic_desc: Option<String>,
    pub plan_url: Option<String>,
    pub currency: Option<Currency>,
    pub discounted_price: Option<i32>,
    pub free_domain: Option<bool>,
    pub domain_extension: Option<String>,
    pub database_benefit: Option<bool>,
    pub page_data: Option<PageData>,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, AsChangeset)]
#[table_name = "plans"]
pub struct UpdatePlanRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub price_unit: Option<String>,
    pub price_timeunit: Option<String>,
    pub price_desc: Option<String>,
    // Concurrent Build
    pub concurrent_build: Option<i32>,
    pub concurrent_build_unit: Option<String>,
    pub concurrent_build_timeunit: Option<String>,
    pub concurrent_build_desc: Option<String>,
    // Bandwidth
    pub bandwidth: Option<i32>,
    pub bandwidth_unit: Option<String>,
    pub bandwidth_timeunit: Option<String>,
    pub bandwidth_desc: Option<String>,
    // Build
    pub build: Option<i32>,
    pub build_unit: Option<String>,
    pub build_timeunit: Option<String>,
    pub build_desc: Option<String>,
    // Analytic
    pub analytic: Option<bool>,
    pub analytic_price: Option<i32>,
    pub analytic_unit: Option<String>,
    pub analytic_timeunit: Option<String>,
    pub analytic_desc: Option<String>,
    pub plan_url: Option<String>,
    pub currency: Option<String>,
    pub discounted_price: Option<i32>,
    pub free_domain: Option<bool>,
    pub domain_extension: Option<String>,
    pub database_benefit: Option<bool>,
    pub page_data: Option<String>,
}

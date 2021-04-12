use super::{cache, db, utils};
use actix_web::http::header::ContentType;
use actix_web::{get, HttpResponse};
use serde::Serialize;
use serde_json::{self, json};

#[derive(Serialize)]
struct RestaurantVotes {
    name: &'static str,
    value: i32,
}

#[get("/getrecipe")]
pub async fn getrecipe() -> HttpResponse {
    let response = json!({
        "recipelink_pancakes": utils::get_recipe_url("pancake").await,
        "recipelink_burritos": utils::get_recipe_url("burritos").await,
        "recipelink_steak": utils::get_recipe_url("steak").await,
        "recipelink_lasagne": utils::get_recipe_url("lasagne").await
    });

    HttpResponse::Ok().set(ContentType::json()).body(response)
}

#[get("/pageviews")]
pub async fn pageviews() -> HttpResponse {
    let page_viewes = match cache::get_views().await {
        Ok(n) => n,
        Err(e) => {
            log::error!("REDIS: {}", e);
            0
        }
    };

    HttpResponse::Ok().body(page_viewes.to_string())
}

#[get("/hostname")]
pub async fn gethostname() -> HttpResponse {
    HttpResponse::Ok().body(utils::get_hostname())
}

#[get("/getstats")]
pub async fn getstats() -> HttpResponse {
    let page_viewes = match cache::get_views().await {
        Ok(n) => n,
        Err(e) => {
            log::error!("REDIS: {}", e);
            0
        }
    };

    HttpResponse::Ok().set(ContentType::json()).body(json!({
        "hostname": utils::get_hostname(),
        "pageviews": page_viewes,
    }))
}

#[get("/getvotes")]
pub async fn getvotes() -> HttpResponse {
    let data = ["outback", "ihop", "bucadibeppo", "chipotle"]
        .iter()
        .map(|m| RestaurantVotes {
            name: m,
            value: db::get_restaurant_votes(m),
        })
        .collect::<Vec<RestaurantVotes>>();

    HttpResponse::Ok()
        .set(ContentType::json())
        .body(serde_json::to_string(&data).unwrap())
}

#[get("/ihop")]
pub async fn ihop() -> HttpResponse {
    HttpResponse::Ok().body(db::add_restaurant_vote("ihop").to_string())
}

#[get("/chipotle")]
pub async fn chipotle() -> HttpResponse {
    HttpResponse::Ok().body(db::add_restaurant_vote("chipotle").to_string())
}

#[get("/outback")]
pub async fn outback() -> HttpResponse {
    HttpResponse::Ok().body(db::add_restaurant_vote("outback").to_string())
}

#[get("/bucadibeppo")]
pub async fn bucadibeppo() -> HttpResponse {
    HttpResponse::Ok().body(db::add_restaurant_vote("bucadibeppo").to_string())
}

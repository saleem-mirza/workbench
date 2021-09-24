use actix_web::client::Client;
use std::sync::Once;

static INIT: Once = Once::new();
static mut HOST: String = String::new();

pub async fn get_recipe_url(item: &str) -> String {
    let client = Client::default();
    let url = format!(
        "https://www.themealdb.com/api/json/v1/1/search.php?s={}",
        item
    );

    match client.get(url).send().await {
        Ok(mut result) => match result.body().await {
            Ok(data) => {
                let content: serde_json::Value = serde_json::from_slice(data.as_ref()).unwrap();
                if !&content.is_object() {
                    return "".to_string();
                }

                let recipes = &content["meals"][0];
                if recipes.is_object() {
                    let url = match recipes.as_object() {
                        Some(recipe) => {
                            if recipe.contains_key("strYoutube") {
                                recipe["strYoutube"].to_string()
                            } else {
                                recipe["strSource"].to_string()
                            }
                        }
                        None => String::from(""),
                    };
                    return url;
                } else {
                    String::from("")
                }
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
                String::from("")
            }
        },
        Err(e) => {
            eprintln!("{}", e.to_string());
            String::from("")
        }
    }
}

pub fn get_hostname() -> &'static str {
    unsafe {
        INIT.call_once(|| {
            HOST = match &hostname::get() {
                Ok(host) => match host.to_str() {
                    Some(x) => String::from(x),
                    _ => String::from("NONE"),
                },
                _ => String::from("NONE"),
            };
        });

        &HOST
    }
}

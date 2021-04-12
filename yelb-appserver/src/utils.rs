use actix_web::client::Client;
use std::sync::Once;

static INIT: Once = Once::new();
static mut HOST: String = String::new();

pub async fn get_recipe_url(item: &str) -> String {
    let client = Client::default();
    let url = format!("http://www.recipepuppy.com/api/?q={}", item);
    match client.get(url).send().await {
        Ok(mut result) => match result.body().await {
            Ok(data) => {
                let content: serde_json::Value = serde_json::from_slice(data.as_ref()).unwrap();
                content["results"][0]["href"].to_string().replace("\"", "")
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

use redis::{self, AsyncCommands, RedisError};
use std::sync::Once;

static INIT: Once = Once::new();
static mut REDIS_CONNECTION_STR: String = String::new();

pub fn get_redis_connection() -> &'static str {
    unsafe {
        INIT.call_once(|| {
            let cache_server = match std::env::var("REDIS_SERVER_ENDPOINT") {
                Ok(s) => s,
                _ => "redis-server".to_string(),
            };

            let cache_port = match std::env::var("REDIS_SERVER_PORT") {
                Ok(s) => match s.parse::<u16>() {
                    Ok(p) => p,
                    Err(e) => {
                        log::warn!("{}, using default port {}", e, 6379);
                        6379
                    }
                },
                _ => 6379,
            };

            REDIS_CONNECTION_STR = format!("redis://{}:{}", cache_server, cache_port);
        });
        &REDIS_CONNECTION_STR
    }
}

pub async fn get_views() -> Result<i64, RedisError> {
    let client = redis::Client::open(get_redis_connection())?;
    let mut con = client.get_async_connection().await?;
    let result: i64 = con.incr("pageviews", 1).await?;
    Ok(result)
}

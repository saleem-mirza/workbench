use postgres::{Client, NoTls};
use std::sync::Once;

static INIT: Once = Once::new();
static mut CON_STR: String = String::new();

fn connect(con_str: &str) -> Option<Client> {
    match Client::connect(con_str, NoTls) {
        Ok(c) => Some(c),
        _ => None,
    }
}

pub fn get_yelb_connection() -> Option<Client> {
    match Client::connect(get_db_connection_str(), NoTls) {
        Ok(c) => Some(c),
        Err(e) => {
            log::error!("[+POSTGRES+] {}", e);
            None
        }
    }
}

pub fn get_db_connection_str() -> &'static str {
    unsafe {
        INIT.call_once(|| {
            let pg_server = match std::env::var("YELB_DB_SERVER_ENDPOINT") {
                Ok(s) => {
                    if s.len() > 0 {
                        s
                    } else {
                        "yelb-db".to_string()
                    }
                }
                _ => "yelb-db".to_string(),
            };

            let pg_port = match std::env::var("YELB_DB_SERVER_PORT") {
                Ok(s) => s.parse::<u16>().unwrap_or(5432),
                _ => 5432,
            };

            let pg_db = match std::env::var("YELB_DB_DATABASE") {
                Ok(s) => {
                    if s.len() > 0 {
                        s
                    } else {
                        "yelbdatabase".to_string()
                    }
                }
                _ => "yelbdatabase".to_string(),
            };

            let pg_user = match std::env::var("YELB_DB_USER") {
                Ok(s) => {
                    if s.len() > 0 {
                        s
                    } else {
                        "postgres".to_string()
                    }
                }
                _ => "postgres".to_string(),
            };

            let pg_pass = match std::env::var("YELB_DB_PASS") {
                Ok(s) => {
                    if s.len() > 0 {
                        s
                    } else {
                        "postgres_password".to_string()
                    }
                }
                _ => "postgres_password".to_string(),
            };

            CON_STR = format!(
                "user={} password={} host={} port={} dbname={}",
                pg_user, pg_pass, pg_server, pg_port, pg_db
            );

            for _ in 0..10 {
                if connect(&CON_STR).is_some() {
                    break;
                }
                println!(".");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
        &CON_STR
    }
}

pub fn initialize_database() {
    if let Some(mut client) = get_yelb_connection() {
        if let Err(e) = client.batch_execute(
            r#"
                    CREATE TABLE IF NOT EXISTS restaurants (
                        name        char(30),
                        count       integer,
                        PRIMARY KEY (name)
                    )
                "#,
        ) {
            log::error!("[-POSTGRES-] {}", e);
            return;
        }

        if client
            .query("SELECT name, count FROM restaurants", &[])
            .unwrap()
            .len()
            == 0
        {
            client
                .execute(
                    r#"
                INSERT INTO restaurants (name, count) VALUES 
                    ('outback', 0),
                    ('bucadibeppo', 0),
                    ('chipotle', 0),
                    ('ihop', 0)
                "#,
                    &[],
                )
                .unwrap();
        };
    }
}

pub fn get_restaurant_votes(restaurant: &str) -> i32 {
    if let Some(mut client) = get_yelb_connection() {
        let data = client
            .query(
                "SELECT count from restaurants WHERE name=($1)",
                &[&restaurant],
            )
            .unwrap();

        match data.get(0) {
            Some(x) => x.get(0),
            _ => 0,
        }
    } else {
        0
    }
}

pub fn add_restaurant_vote(restaurant: &str) -> i32 {
    if let Some(mut client) = get_yelb_connection() {
        let data = client
            .query(
                "UPDATE restaurants SET count = count + 1 WHERE name=($1) RETURNING count",
                &[&restaurant],
            )
            .unwrap();

        match data.get(0) {
            Some(x) => x.get(0),
            _ => 0,
        }
    } else {
        0
    }
}

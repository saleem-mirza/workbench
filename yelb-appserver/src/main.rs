use actix_web::{
    middleware::{self, Logger},
    web, App, HttpResponse, HttpServer,
};
use num_cpus;
use std::io::Write;
use yelb_appserver::{self, db, handlers, utils};

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut port: u16 = 4567;

    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info"))
        .format(|buf, rec| {
            writeln!(
                buf,
                "{} {}: {}",
                chrono::Utc::now().to_rfc3339(),
                rec.level(),
                rec.args()
            )
        })
        .init();

    port = match std::env::var("PORT") {
        Ok(s) => match s.parse::<u16>() {
            Ok(p) => p,
            Err(e) => {
                log::warn!("{}, using default port {}", e, port);
                port
            }
        },
        _ => port,
    };

    db::initialize_database();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("Access-Control-Allow-Origin", "*")
                    .header(
                        "Access-Control-Allow-Headers",
                        "Authorization,Accepts,Content-Type,X-CSRF-Token,X-Requested-With",
                    )
                    .header(
                        "Access-Control-Allow-Methods",
                        "GET,POST,PUT,DELETE,OPTIONS",
                    ),
            )
            .service(
                web::scope("/api")
                    .service(handlers::getrecipe)
                    .service(handlers::pageviews)
                    .service(handlers::gethostname)
                    .service(handlers::getstats)
                    .service(handlers::getvotes)
                    .service(handlers::ihop)
                    .service(handlers::chipotle)
                    .service(handlers::outback)
                    .service(handlers::bucadibeppo)
                    .route(
                        "/who",
                        web::get().to(|| HttpResponse::Ok().body(utils::get_hostname())),
                    ),
            )
    })
    .workers(num_cpus::get() * 2)
    .max_connection_rate(1024)
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

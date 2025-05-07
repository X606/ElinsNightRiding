use std::net::SocketAddr;

use axum::routing::*;
use axum_server::tls_rustls::RustlsConfig;

#[tokio::main]
async fn main() {
    let tls_config = RustlsConfig::from_pem_file("/etc/letsencrypt/live/carworld.x606.dev/fullchain.pem", "/etc/letsencrypt/live/carworld.x606.dev/privkey.pem").await.expect("unable to load files");

     // build our application with a single route
     let app = Router::new()
     .route("/", get(main_route))
     .route("/test", get(test));

     let addr = SocketAddr::from(([0, 0, 0, 0], 7878));
     axum_server::bind_rustls(addr, tls_config)
         .serve(app.into_make_service())
         .await
         .unwrap();

    //axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}

async fn main_route() -> &'static str {
    "Hello, World!"
}

async fn test() -> &'static str {
    "hi :3"
}

use crate::wallet::ShiroWallet;
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use std::sync::Mutex;

mod healthz;
mod keys;
mod wallet;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let shiro_wallet = Mutex::new(wallet::ShiroWallet::new());
        let data = web::Data::new(shiro_wallet);
        let cors = Cors::default()
            .allow_any_origin()
            .send_wildcard()
            .allowed_methods(vec!["GET", "DELETE", "OPTIONS", "POST", "PUT"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::ACCEPT_ENCODING,
                header::CONTENT_TYPE,
                header::CONTENT_LENGTH,
            ])
            .max_age(3600);

        let frontend = actix_files::Files::new("/", "./app").index_file("index.html");

        App::new()
            .app_data(data)
            .wrap(cors)
            .service(healthz::get)
            .service(keys::post)
            .service(keys::put)
            .service(wallet::address::get)
            .service(wallet::invoice::put)
            .service(wallet::asset_balance::get)
            .service(wallet::assets::put)
            .service(wallet::blind::put)
            .service(wallet::data::get)
            .service(wallet::dir::get)
            .service(wallet::drain_to::put)
            .service(wallet::go_online::put)
            .service(wallet::issue::rgb20::put)
            .service(wallet::refresh::post)
            .service(wallet::send::post)
            .service(wallet::put)
            .service(wallet::transfers::delete)
            .service(wallet::transfers::put)
            .service(wallet::unspents::put)
            .service(wallet::utxos::put)
            .service(frontend)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{test, web, App};
    use once_cell::sync::Lazy;
    use serde::Deserialize;
    use serde::Serialize;

    pub static PROXY_ENDPOINT: Lazy<String> =
        Lazy::new(|| "rgbhttpjsonrpc:http://127.0.0.1:3000/json-rpc".to_string());

    #[derive(Serialize, Deserialize)]
    pub struct OnlineResult {
        pub id: String,
        pub electrum_url: String,
        pub proxy_url: String,
    }

    #[actix_web::test]
    async fn test_root() {
        let shiro_wallet = Mutex::new(ShiroWallet::new());
        let app = test::init_service(App::new().app_data(web::Data::new(shiro_wallet))).await;
        let req = test::TestRequest::get().uri("/").to_request();

        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
    }
}

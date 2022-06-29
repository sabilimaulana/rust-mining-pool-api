use actix_web::{get, post};

use {crate::utils::*, crate::wallet::*, actix_web::web::Json, actix_web::HttpResponse};

#[get("/wallets")]
pub async fn list_wallets() -> HttpResponse {
    let wallets: Vec<Wallet> = vec![];

    ResponseType::Ok(wallets).get_response()
}

#[get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    let wallet: Option<Wallet> = None;
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Wallet not found.".to_string()))
            .get_response(),
    }
}

#[post("/wallets")]
pub async fn create_wallet(_wallet_request: Json<NewWalletRequest>) -> HttpResponse {
    let wallet: Vec<Wallet> = vec![];
    ResponseType::Created(wallet).get_response()
}

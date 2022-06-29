use actix_web::{get, post};

use {crate::miner::*, crate::utils::*, actix_web::web::Json, actix_web::HttpResponse};

#[get("/miners")]
pub async fn list_miners() -> HttpResponse {
    let miners: Vec<Miner> = vec![];

    ResponseType::Ok(miners).get_response()
}

#[get("/miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    let miner: Option<Miner> = None;
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(NotFoundMessage::new("Miner not found.".to_string()))
            .get_response(),
    }
}

#[post("/wallets/{id}/miners/")]
pub async fn create_miner(_miner_request: Json<NewMinerRequest>) -> HttpResponse {
    let miner: Vec<Miner> = vec![];
    ResponseType::Created(miner).get_response()
}

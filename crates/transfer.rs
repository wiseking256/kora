use axum::{Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    to: String,
    amount: u64,
}

#[derive(Debug, Serialize)]
pub struct TransferResponse {
    signed_tx: String,
}

pub async fn handle_transfer(
    Json(payload): Json<TransferRequest>
) -> Json<TransferResponse> {
    // This is dummy logic to simulate transfer
    let dummy_tx = format!("signed_transfer_to_{}_amount_{}", payload.to, payload.amount);

    Json(TransferResponse {
        signed_tx: dummy_tx,
    })
  }

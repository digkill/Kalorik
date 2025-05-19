use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};
use crate::db::queries;

#[derive(Deserialize)]
pub struct SubscriptionCallback {
    pub user_id: i64,
    pub status: String,
    pub expires_at: Option<String>,
}

#[post("/subscription/callback")]
pub async fn subscription_callback(
    req: HttpRequest,
    body: web::Bytes,
) -> impl Responder {
    let signature = req
        .headers()
        .get("X-Signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let secret = std::env::var("HMAC_SECRET").unwrap_or_else(|_| "default_secret".into());
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(&body);
    let expected = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

    if expected != signature {
        log::warn!("Invalid webhook signature. Expected {}, got {}", expected, signature);
        return HttpResponse::Unauthorized().body("invalid signature");
    }

    let payload: SubscriptionCallback = match serde_json::from_slice(&body) {
        Ok(p) => p,
        Err(_) => return HttpResponse::BadRequest().body("invalid json"),
    };

    if payload.status != "success" {
        log::warn!("Received non-success status: {}", payload.status);
        return HttpResponse::BadRequest().body("invalid status");
    }

    let expires_at = payload
        .expires_at
        .unwrap_or_else(|| "2099-12-31T00:00:00Z".to_string());

    match queries::update_subscription(payload.user_id, &expires_at).await {
        Ok(_) => {
            log::info!("Subscription updated for user {}", payload.user_id);
            HttpResponse::Ok().body("ok")
        }
        Err(e) => {
            log::error!("DB error on subscription update: {}", e);
            HttpResponse::InternalServerError().body("db error")
        }
    }
}

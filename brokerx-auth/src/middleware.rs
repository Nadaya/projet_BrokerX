use axum::{
    http::Request,
    middleware::Next,
    response::Response,
    body::Body,
};
use base64::engine::general_purpose;
use base64::Engine;
use brokerx_types::auth::LoginRequest;
use reqwest::Client;

/// Middleware pour l'authentification Basic Auth
pub async fn basic_auth(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<Response, (axum::http::StatusCode, String)> {
    if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Basic ") {
                let encoded = auth_str.trim_start_matches("Basic ");
                if let Ok(decoded) = general_purpose::STANDARD.decode(encoded) {
                    if let Ok(creds) = String::from_utf8(decoded) {
                        let parts: Vec<&str> = creds.split(':').collect();
                        if parts.len() == 2 {
                            let username = parts[0];
                            let password = parts[1];

                            // Crée le payload LoginRequest
                            let login_request = LoginRequest { username, password };

                            // Appel à account-service pour vérifier les identifiants
                            // On suppose que account-service expose un endpoint POST /login
                            let client = Client::new();
                            let res = client
                                .post("http://account-service:8080/login")
                                .json(&login_request)
                                .send()
                                .await;

                            match res {
                                Ok(resp) if resp.status().is_success() => {
                                    // Auth réussie, on continue
                                    return Ok(next.run(req).await);
                                }
                                _ => {
                                    return Err((
                                        axum::http::StatusCode::UNAUTHORIZED,
                                        "Identifiants invalides".into(),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Err((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authorization manquant ou invalide".into(),
    ))
}

use super::{AuthService, LoginRequest, RegisterRequest, RefreshTokenRequest};
use anyhow::Result;
use bytes::Bytes;
use http::{Method, Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use serde_json::json;
use std::sync::Arc;

pub struct AuthRoutes {
    auth_service: Arc<AuthService>,
}

impl AuthRoutes {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn handle_request(
        &self,
        req: Request<Incoming>,
        path: &str,
    ) -> Result<Response<Full<Bytes>>> {
        match (req.method(), path) {
            (&Method::POST, "/auth/register") => self.handle_register(req).await,
            (&Method::POST, "/auth/login") => self.handle_login(req).await,
            (&Method::POST, "/auth/refresh") => self.handle_refresh(req).await,
            (&Method::POST, "/auth/logout") => self.handle_logout(req).await,
            (&Method::GET, "/auth/me") => self.handle_get_me(req).await,
            (&Method::PUT, "/auth/profile") => self.handle_update_profile(req).await,
            (&Method::POST, "/auth/change-password") => self.handle_change_password(req).await,
            _ => Ok(not_found_response()),
        }
    }

    async fn handle_register(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
        let body_bytes = req.into_body().collect().await?.to_bytes();
        let register_req: RegisterRequest = serde_json::from_slice(&body_bytes)?;

        match self.auth_service.register(register_req).await {
            Ok(auth_response) => {
                let body = serde_json::to_vec(&auth_response)?;
                Ok(Response::builder()
                    .status(StatusCode::CREATED)
                    .header("Content-Type", "application/json")
                    .body(Full::new(Bytes::from(body)))?)
            }
            Err(e) => {
                let error_body = json!({
                    "error": e.to_string()
                });
                Ok(Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "application/json")
                    .body(Full::new(Bytes::from(error_body.to_string())))?)
            }
        }
    }

    async fn handle_login(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
        let body_bytes = req.into_body().collect().await?.to_bytes();
        let login_req: LoginRequest = serde_json::from_slice(&body_bytes)?;

        match self.auth_service.login(login_req).await {
            Ok(auth_response) => {
                let body = serde_json::to_vec(&auth_response)?;
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(Full::new(Bytes::from(body)))?)
            }
            Err(e) => {
                let error_body = json!({
                    "error": e.to_string()
                });
                Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("Content-Type", "application/json")
                    .body(Full::new(Bytes::from(error_body.to_string())))?)
            }
        }
    }

    async fn handle_refresh(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
        let body_bytes = req.into_body().collect().await?.to_bytes();
        let refresh_req: RefreshTokenRequest = serde_json::from_slice(&body_bytes)?;

        match self.auth_service.refresh_token(&refresh_req.refresh_token).await {
            Ok(auth_response) => {
                let body = serde_json::to_vec(&auth_response)?;
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(Full::new(Bytes::from(body)))?)
            }
            Err(e) => {
                let error_body = json!({
                    "error": e.to_string()
                });
                Ok(Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("Content-Type", "application/json")
                    .body(Full::new(Bytes::from(error_body.to_string())))?)
            }
        }
    }

    async fn handle_logout(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        if let Some(auth_header) = auth_header {
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
                let _ = self.auth_service.logout(token).await;
            }
        }

        let body = json!({
            "message": "Logged out successfully"
        });

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Full::new(Bytes::from(body.to_string())))?)
    }

    async fn handle_get_me(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| anyhow::anyhow!("Missing authorization header"))?;

        if !auth_header.starts_with("Bearer ") {
            return Ok(unauthorized_response());
        }

        let token = &auth_header[7..];

        match self.auth_service.validate_token(token).await {
            Ok(claims) => {
                match self.auth_service.get_user(&claims.sub).await {
                    Ok(user) => {
                        let body = serde_json::to_vec(&user)?;
                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", "application/json")
                            .body(Full::new(Bytes::from(body)))?)
                    }
                    Err(_) => Ok(unauthorized_response()),
                }
            }
            Err(_) => Ok(unauthorized_response()),
        }
    }

    async fn handle_update_profile(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| anyhow::anyhow!("Missing authorization header"))?;

        if !auth_header.starts_with("Bearer ") {
            return Ok(unauthorized_response());
        }

        let token = auth_header[7..].to_string();
        let body_bytes = req.into_body().collect().await?.to_bytes();

        #[derive(serde::Deserialize)]
        struct UpdateProfileRequest {
            full_name: Option<String>,
            avatar_url: Option<String>,
        }

        let update_req: UpdateProfileRequest = serde_json::from_slice(&body_bytes)?;

        match self.auth_service.validate_token(&token).await {
            Ok(claims) => {
                match self.auth_service.update_profile(&claims.sub, update_req.full_name, update_req.avatar_url).await {
                    Ok(user) => {
                        let body = serde_json::to_vec(&user)?;
                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", "application/json")
                            .body(Full::new(Bytes::from(body)))?)
                    }
                    Err(e) => {
                        let error_body = json!({
                            "error": e.to_string()
                        });
                        Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .header("Content-Type", "application/json")
                            .body(Full::new(Bytes::from(error_body.to_string())))?)
                    }
                }
            }
            Err(_) => Ok(unauthorized_response()),
        }
    }

    async fn handle_change_password(&self, req: Request<Incoming>) -> Result<Response<Full<Bytes>>> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| anyhow::anyhow!("Missing authorization header"))?;

        if !auth_header.starts_with("Bearer ") {
            return Ok(unauthorized_response());
        }

        let token = auth_header[7..].to_string();
        let body_bytes = req.into_body().collect().await?.to_bytes();

        #[derive(serde::Deserialize)]
        struct ChangePasswordRequest {
            old_password: String,
            new_password: String,
        }

        let change_req: ChangePasswordRequest = serde_json::from_slice(&body_bytes)?;

        match self.auth_service.validate_token(&token).await {
            Ok(claims) => {
                match self.auth_service.change_password(&claims.sub, &change_req.old_password, &change_req.new_password).await {
                    Ok(_) => {
                        let body = json!({
                            "message": "Password changed successfully"
                        });
                        Ok(Response::builder()
                            .status(StatusCode::OK)
                            .header("Content-Type", "application/json")
                            .body(Full::new(Bytes::from(body.to_string())))?)
                    }
                    Err(e) => {
                        let error_body = json!({
                            "error": e.to_string()
                        });
                        Ok(Response::builder()
                            .status(StatusCode::BAD_REQUEST)
                            .header("Content-Type", "application/json")
                            .body(Full::new(Bytes::from(error_body.to_string())))?)
                    }
                }
            }
            Err(_) => Ok(unauthorized_response()),
        }
    }
}

fn not_found_response() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(r#"{"error": "Not found"}"#)))
        .unwrap()
}

fn unauthorized_response() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(r#"{"error": "Unauthorized"}"#)))
        .unwrap()
}
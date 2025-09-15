use super::{AuthService, Claims, UserInfo};
use http::{Request, Response, StatusCode};
use http_body_util::Full;
use bytes::Bytes;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthMiddleware {
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn verify_request(&self, req: &Request<impl std::any::Any>) -> Result<Claims, StatusCode> {
        // Extract Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Check for Bearer token
        if !auth_header.starts_with("Bearer ") {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let token = &auth_header[7..];

        // Validate token
        self.auth_service
            .validate_token(token)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)
    }

    pub async fn get_current_user(&self, token: &str) -> Result<UserInfo, anyhow::Error> {
        let claims = self.auth_service.validate_token(token).await?;
        self.auth_service.get_user(&claims.sub).await
    }
}

// Helper function to create unauthorized response
pub fn unauthorized_response() -> Response<Full<Bytes>> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(
            r#"{"error": "Unauthorized"}"#
        )))
        .unwrap()
}

// Helper function to check if path requires authentication
pub fn requires_auth(path: &str) -> bool {
    // Define public paths that don't require authentication
    let public_paths = vec![
        "/",
        "/playground",
        "/arena",
        "/auth/login",
        "/auth/register",
        "/auth/refresh",
        "/health",
    ];

    // Static assets don't require auth
    if path.starts_with("/static/") || path.starts_with("/assets/") {
        return false;
    }

    // Check if path is in public paths
    !public_paths.contains(&path) && !public_paths.iter().any(|p| path.starts_with(p))
}

// Extension trait to add user info to request
pub struct AuthContext {
    pub user: Option<UserInfo>,
    pub claims: Option<Claims>,
}

impl AuthContext {
    pub fn new() -> Self {
        Self {
            user: None,
            claims: None,
        }
    }

    pub fn with_claims(claims: Claims) -> Self {
        Self {
            user: None,
            claims: Some(claims),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.claims.is_some()
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.claims
            .as_ref()
            .map(|c| c.role == role)
            .unwrap_or(false)
    }

    pub fn user_id(&self) -> Option<String> {
        self.claims.as_ref().map(|c| c.sub.clone())
    }
}
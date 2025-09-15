use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

pub mod middleware;
pub mod routes;

#[cfg(test)]
mod tests;

pub use middleware::AuthMiddleware;
pub use routes::AuthRoutes;

#[derive(Debug, Clone)]
pub struct AuthService {
    pool: Arc<PgPool>,
    jwt_secret: String,
    jwt_expiry_hours: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub username: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub full_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub username: String,
    pub full_name: Option<String>,
    pub role: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

impl AuthService {
    pub fn new(pool: Arc<PgPool>, jwt_secret: String, jwt_expiry_hours: i64) -> Self {
        Self {
            pool,
            jwt_secret,
            jwt_expiry_hours,
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse> {
        // Validate input
        if req.password.len() < 8 {
            return Err(anyhow!("Password must be at least 8 characters long"));
        }

        // Hash password
        let password_hash = hash(req.password.as_bytes(), DEFAULT_COST)?;

        // Insert user into database
        let user_id = Uuid::new_v4();
        let query = r#"
            INSERT INTO users (id, email, username, password_hash, full_name)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, email, username, full_name, role, avatar_url
        "#;

        let row = sqlx::query(query)
            .bind(&user_id)
            .bind(&req.email)
            .bind(&req.username)
            .bind(&password_hash)
            .bind(&req.full_name)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("duplicate key") {
                    anyhow!("Email or username already exists")
                } else {
                    anyhow!("Failed to create user: {}", e)
                }
            })?;

        let user = UserInfo {
            id: row.get::<Uuid, _>("id").to_string(),
            email: row.get("email"),
            username: row.get("username"),
            full_name: row.get("full_name"),
            role: row.get("role"),
            avatar_url: row.get("avatar_url"),
        };

        // Generate tokens
        self.generate_auth_response(user).await
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse> {
        // Find user by email or username
        let query = r#"
            SELECT id, email, username, password_hash, full_name, role, avatar_url, is_active
            FROM users
            WHERE email = $1 OR username = $1
        "#;

        let row = sqlx::query(query)
            .bind(&req.username_or_email)
            .fetch_optional(&*self.pool)
            .await?
            .ok_or_else(|| anyhow!("Invalid credentials"))?;

        // Check if account is active
        let is_active: bool = row.get("is_active");
        if !is_active {
            return Err(anyhow!("Account is disabled"));
        }

        // Verify password
        let password_hash: String = row.get("password_hash");
        if !verify(&req.password, &password_hash)? {
            return Err(anyhow!("Invalid credentials"));
        }

        // Update last login
        let user_id: Uuid = row.get("id");
        sqlx::query("UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&user_id)
            .execute(&*self.pool)
            .await?;

        let user = UserInfo {
            id: user_id.to_string(),
            email: row.get("email"),
            username: row.get("username"),
            full_name: row.get("full_name"),
            role: row.get("role"),
            avatar_url: row.get("avatar_url"),
        };

        // Generate tokens
        self.generate_auth_response(user).await
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthResponse> {
        // Decode and validate refresh token
        let token_data = self.decode_token(refresh_token)?;

        // Get user from database
        let user_id = Uuid::parse_str(&token_data.claims.sub)?;
        let query = r#"
            SELECT id, email, username, full_name, role, avatar_url, is_active
            FROM users
            WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(&user_id)
            .fetch_optional(&*self.pool)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        let is_active: bool = row.get("is_active");
        if !is_active {
            return Err(anyhow!("Account is disabled"));
        }

        let user = UserInfo {
            id: user_id.to_string(),
            email: row.get("email"),
            username: row.get("username"),
            full_name: row.get("full_name"),
            role: row.get("role"),
            avatar_url: row.get("avatar_url"),
        };

        // Generate new tokens
        self.generate_auth_response(user).await
    }

    pub async fn validate_token(&self, token: &str) -> Result<Claims> {
        let token_data = self.decode_token(token)?;
        Ok(token_data.claims)
    }

    pub async fn get_user(&self, user_id: &str) -> Result<UserInfo> {
        let user_id = Uuid::parse_str(user_id)?;
        let query = r#"
            SELECT id, email, username, full_name, role, avatar_url
            FROM users
            WHERE id = $1 AND is_active = true
        "#;

        let row = sqlx::query(query)
            .bind(&user_id)
            .fetch_optional(&*self.pool)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        Ok(UserInfo {
            id: user_id.to_string(),
            email: row.get("email"),
            username: row.get("username"),
            full_name: row.get("full_name"),
            role: row.get("role"),
            avatar_url: row.get("avatar_url"),
        })
    }

    pub async fn logout(&self, token: &str) -> Result<()> {
        // In a production system, you would invalidate the token here
        // For now, we'll just validate it exists
        let _ = self.decode_token(token)?;

        // Could store invalidated tokens in Redis or database
        // For this implementation, tokens expire naturally

        Ok(())
    }

    async fn generate_auth_response(&self, user: UserInfo) -> Result<AuthResponse> {
        let now = Utc::now();
        let access_expiry = now + Duration::hours(self.jwt_expiry_hours);
        let refresh_expiry = now + Duration::days(30);

        // Create access token claims
        let access_claims = Claims {
            sub: user.id.clone(),
            email: user.email.clone(),
            username: user.username.clone(),
            role: user.role.clone(),
            exp: access_expiry.timestamp(),
            iat: now.timestamp(),
        };

        // Create refresh token claims
        let refresh_claims = Claims {
            sub: user.id.clone(),
            email: user.email.clone(),
            username: user.username.clone(),
            role: user.role.clone(),
            exp: refresh_expiry.timestamp(),
            iat: now.timestamp(),
        };

        // Generate tokens
        let access_token = self.encode_token(&access_claims)?;
        let refresh_token = self.encode_token(&refresh_claims)?;

        // Store session in database
        let session_id = Uuid::new_v4();
        let token_hash = self.hash_token(&access_token);
        let refresh_hash = self.hash_token(&refresh_token);

        let query = r#"
            INSERT INTO sessions (id, user_id, token_hash, refresh_token_hash, expires_at)
            VALUES ($1, $2, $3, $4, $5)
        "#;

        sqlx::query(query)
            .bind(&session_id)
            .bind(Uuid::parse_str(&user.id)?)
            .bind(&token_hash)
            .bind(&refresh_hash)
            .bind(access_expiry.naive_utc())
            .execute(&*self.pool)
            .await?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.jwt_expiry_hours * 3600,
            user,
        })
    }

    fn encode_token(&self, claims: &Claims) -> Result<String> {
        let token = encode(
            &Header::default(),
            claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;
        Ok(token)
    }

    fn decode_token(&self, token: &str) -> Result<TokenData<Claims>> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data)
    }

    fn hash_token(&self, token: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub async fn change_password(&self, user_id: &str, old_password: &str, new_password: &str) -> Result<()> {
        if new_password.len() < 8 {
            return Err(anyhow!("Password must be at least 8 characters long"));
        }

        let user_id = Uuid::parse_str(user_id)?;

        // Verify old password
        let query = "SELECT password_hash FROM users WHERE id = $1";
        let row = sqlx::query(query)
            .bind(&user_id)
            .fetch_optional(&*self.pool)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        let password_hash: String = row.get("password_hash");
        if !verify(old_password, &password_hash)? {
            return Err(anyhow!("Invalid old password"));
        }

        // Update password
        let new_hash = hash(new_password.as_bytes(), DEFAULT_COST)?;
        sqlx::query("UPDATE users SET password_hash = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
            .bind(&new_hash)
            .bind(&user_id)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    pub async fn update_profile(&self, user_id: &str, full_name: Option<String>, avatar_url: Option<String>) -> Result<UserInfo> {
        let user_id = Uuid::parse_str(user_id)?;

        let query = r#"
            UPDATE users
            SET full_name = COALESCE($1, full_name),
                avatar_url = COALESCE($2, avatar_url),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            RETURNING id, email, username, full_name, role, avatar_url
        "#;

        let row = sqlx::query(query)
            .bind(&full_name)
            .bind(&avatar_url)
            .bind(&user_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(UserInfo {
            id: user_id.to_string(),
            email: row.get("email"),
            username: row.get("username"),
            full_name: row.get("full_name"),
            role: row.get("role"),
            avatar_url: row.get("avatar_url"),
        })
    }
}
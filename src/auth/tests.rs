#[cfg(test)]
mod tests {
    use super::super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    async fn setup_test_db() -> Result<Arc<PgPool>> {
        // Use test database URL or in-memory database
        let database_url = env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/aichat_test".to_string());

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        // Run migrations
        let migration_sql = include_str!("../../migrations/001_initial_schema.sql");
        for statement in migration_sql.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                let _ = sqlx::query(trimmed).execute(&pool).await;
            }
        }

        Ok(Arc::new(pool))
    }

    async fn cleanup_test_user(pool: &PgPool, email: &str) {
        let _ = sqlx::query("DELETE FROM users WHERE email = $1")
            .bind(email)
            .execute(pool)
            .await;
    }

    #[tokio::test]
    async fn test_user_registration() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        let auth_service = AuthService::new(pool.clone(), "test_secret".to_string(), 24);

        let test_email = "test_registration@example.com";
        cleanup_test_user(&pool, test_email).await;

        let register_req = RegisterRequest {
            email: test_email.to_string(),
            username: "testuser".to_string(),
            password: "Test123!@#".to_string(),
            full_name: Some("Test User".to_string()),
        };

        let result = auth_service.register(register_req).await;
        assert!(result.is_ok());

        let auth_response = result.unwrap();
        assert_eq!(auth_response.user.email, test_email);
        assert_eq!(auth_response.user.username, "testuser");
        assert!(!auth_response.access_token.is_empty());
        assert!(!auth_response.refresh_token.is_empty());

        cleanup_test_user(&pool, test_email).await;
    }

    #[tokio::test]
    async fn test_user_login() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        let auth_service = AuthService::new(pool.clone(), "test_secret".to_string(), 24);

        let test_email = "test_login@example.com";
        cleanup_test_user(&pool, test_email).await;

        // First register a user
        let register_req = RegisterRequest {
            email: test_email.to_string(),
            username: "loginuser".to_string(),
            password: "Test123!@#".to_string(),
            full_name: None,
        };

        auth_service.register(register_req).await.expect("Registration failed");

        // Test login with email
        let login_req = LoginRequest {
            username_or_email: test_email.to_string(),
            password: "Test123!@#".to_string(),
        };

        let result = auth_service.login(login_req).await;
        assert!(result.is_ok());

        let auth_response = result.unwrap();
        assert_eq!(auth_response.user.email, test_email);

        // Test login with username
        let login_req = LoginRequest {
            username_or_email: "loginuser".to_string(),
            password: "Test123!@#".to_string(),
        };

        let result = auth_service.login(login_req).await;
        assert!(result.is_ok());

        // Test login with wrong password
        let login_req = LoginRequest {
            username_or_email: test_email.to_string(),
            password: "WrongPassword".to_string(),
        };

        let result = auth_service.login(login_req).await;
        assert!(result.is_err());

        cleanup_test_user(&pool, test_email).await;
    }

    #[tokio::test]
    async fn test_token_validation() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        let auth_service = AuthService::new(pool.clone(), "test_secret".to_string(), 24);

        let test_email = "test_token@example.com";
        cleanup_test_user(&pool, test_email).await;

        // Register and get tokens
        let register_req = RegisterRequest {
            email: test_email.to_string(),
            username: "tokenuser".to_string(),
            password: "Test123!@#".to_string(),
            full_name: None,
        };

        let auth_response = auth_service.register(register_req).await.expect("Registration failed");

        // Validate access token
        let claims = auth_service.validate_token(&auth_response.access_token).await;
        assert!(claims.is_ok());

        let claims = claims.unwrap();
        assert_eq!(claims.email, test_email);
        assert_eq!(claims.username, "tokenuser");

        // Test invalid token
        let invalid_token = "invalid.token.here";
        let result = auth_service.validate_token(invalid_token).await;
        assert!(result.is_err());

        cleanup_test_user(&pool, test_email).await;
    }

    #[tokio::test]
    async fn test_refresh_token() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        let auth_service = AuthService::new(pool.clone(), "test_secret".to_string(), 24);

        let test_email = "test_refresh@example.com";
        cleanup_test_user(&pool, test_email).await;

        // Register and get tokens
        let register_req = RegisterRequest {
            email: test_email.to_string(),
            username: "refreshuser".to_string(),
            password: "Test123!@#".to_string(),
            full_name: None,
        };

        let initial_auth = auth_service.register(register_req).await.expect("Registration failed");

        // Use refresh token to get new tokens
        let new_auth = auth_service.refresh_token(&initial_auth.refresh_token).await;
        assert!(new_auth.is_ok());

        let new_auth = new_auth.unwrap();
        assert_eq!(new_auth.user.email, test_email);
        assert_ne!(new_auth.access_token, initial_auth.access_token);

        cleanup_test_user(&pool, test_email).await;
    }

    #[tokio::test]
    async fn test_password_change() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        let auth_service = AuthService::new(pool.clone(), "test_secret".to_string(), 24);

        let test_email = "test_password@example.com";
        cleanup_test_user(&pool, test_email).await;

        // Register user
        let register_req = RegisterRequest {
            email: test_email.to_string(),
            username: "passworduser".to_string(),
            password: "OldPassword123!".to_string(),
            full_name: None,
        };

        let auth_response = auth_service.register(register_req).await.expect("Registration failed");
        let user_id = auth_response.user.id;

        // Change password
        let result = auth_service.change_password(&user_id, "OldPassword123!", "NewPassword456!").await;
        assert!(result.is_ok());

        // Try login with old password
        let login_req = LoginRequest {
            username_or_email: test_email.to_string(),
            password: "OldPassword123!".to_string(),
        };
        let result = auth_service.login(login_req).await;
        assert!(result.is_err());

        // Login with new password
        let login_req = LoginRequest {
            username_or_email: test_email.to_string(),
            password: "NewPassword456!".to_string(),
        };
        let result = auth_service.login(login_req).await;
        assert!(result.is_ok());

        cleanup_test_user(&pool, test_email).await;
    }

    #[tokio::test]
    async fn test_profile_update() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        let auth_service = AuthService::new(pool.clone(), "test_secret".to_string(), 24);

        let test_email = "test_profile@example.com";
        cleanup_test_user(&pool, test_email).await;

        // Register user
        let register_req = RegisterRequest {
            email: test_email.to_string(),
            username: "profileuser".to_string(),
            password: "Test123!@#".to_string(),
            full_name: Some("Initial Name".to_string()),
        };

        let auth_response = auth_service.register(register_req).await.expect("Registration failed");
        let user_id = auth_response.user.id;

        // Update profile
        let updated_user = auth_service.update_profile(
            &user_id,
            Some("Updated Name".to_string()),
            Some("https://example.com/avatar.png".to_string()),
        ).await;

        assert!(updated_user.is_ok());
        let updated_user = updated_user.unwrap();
        assert_eq!(updated_user.full_name, Some("Updated Name".to_string()));
        assert_eq!(updated_user.avatar_url, Some("https://example.com/avatar.png".to_string()));

        cleanup_test_user(&pool, test_email).await;
    }

    #[tokio::test]
    async fn test_duplicate_registration() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        let auth_service = AuthService::new(pool.clone(), "test_secret".to_string(), 24);

        let test_email = "test_duplicate@example.com";
        cleanup_test_user(&pool, test_email).await;

        // First registration
        let register_req = RegisterRequest {
            email: test_email.to_string(),
            username: "duplicateuser".to_string(),
            password: "Test123!@#".to_string(),
            full_name: None,
        };

        let result = auth_service.register(register_req.clone()).await;
        assert!(result.is_ok());

        // Try to register with same email
        let result = auth_service.register(register_req).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));

        cleanup_test_user(&pool, test_email).await;
    }

    #[tokio::test]
    async fn test_password_validation() {
        let pool = setup_test_db().await.expect("Failed to setup test database");
        let auth_service = AuthService::new(pool.clone(), "test_secret".to_string(), 24);

        // Test short password
        let register_req = RegisterRequest {
            email: "test_short@example.com".to_string(),
            username: "shortpass".to_string(),
            password: "Short1".to_string(),
            full_name: None,
        };

        let result = auth_service.register(register_req).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least 8 characters"));
    }
}
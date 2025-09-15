# AIChat Authentication System

## Overview

The AIChat authentication system provides secure user management with JWT-based authentication, supporting user registration, login, profile management, and session handling.

## Features

- **User Registration & Login**: Email/username based authentication with bcrypt password hashing
- **JWT Token Management**: Access and refresh token system with configurable expiry
- **Session Management**: Database-backed session tracking
- **Profile Management**: User profile updates and avatar support
- **Role-Based Access**: Support for user, admin, and premium roles
- **Secure Password Management**: Password change functionality with old password verification
- **Audit Logging**: Track authentication events for security monitoring

## Database Setup

### Prerequisites

1. PostgreSQL database (local or cloud-based like Supabase)
2. Set the `DATABASE_URL` environment variable:

```bash
export DATABASE_URL="postgresql://username:password@localhost:5432/aichat"
```

### Running Migrations

The database schema is automatically created when the server starts if `DATABASE_URL` is set. The migration script is located at `migrations/001_initial_schema.sql`.

## Configuration

### Environment Variables

```bash
# Required for authentication
DATABASE_URL=postgresql://username:password@localhost:5432/aichat
JWT_SECRET=your-secure-jwt-secret-key

# Optional
JWT_EXPIRY_HOURS=24  # Default: 24 hours
```

### JWT Configuration

- **Access Token Expiry**: Configurable via `JWT_EXPIRY_HOURS` (default: 24 hours)
- **Refresh Token Expiry**: Fixed at 30 days
- **Token Algorithm**: HS256

## API Endpoints

### Public Endpoints (No Authentication Required)

#### POST /auth/register
Register a new user account.

**Request Body:**
```json
{
  "email": "user@example.com",
  "username": "johndoe",
  "password": "SecurePassword123!",
  "full_name": "John Doe"  // Optional
}
```

**Response:**
```json
{
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "johndoe",
    "full_name": "John Doe",
    "role": "user",
    "avatar_url": null
  }
}
```

#### POST /auth/login
Authenticate with email or username.

**Request Body:**
```json
{
  "username_or_email": "user@example.com",
  "password": "SecurePassword123!"
}
```

**Response:** Same as registration response

#### POST /auth/refresh
Get new tokens using refresh token.

**Request Body:**
```json
{
  "refresh_token": "eyJ..."
}
```

**Response:** Same as registration response

### Protected Endpoints (Authentication Required)

Include the JWT token in the Authorization header:
```
Authorization: Bearer eyJ...
```

#### GET /auth/me
Get current user information.

**Response:**
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "username": "johndoe",
  "full_name": "John Doe",
  "role": "user",
  "avatar_url": "https://example.com/avatar.jpg"
}
```

#### PUT /auth/profile
Update user profile.

**Request Body:**
```json
{
  "full_name": "John Smith",  // Optional
  "avatar_url": "https://example.com/new-avatar.jpg"  // Optional
}
```

**Response:** Updated user object

#### POST /auth/change-password
Change user password.

**Request Body:**
```json
{
  "old_password": "OldPassword123!",
  "new_password": "NewPassword456!"
}
```

**Response:**
```json
{
  "message": "Password changed successfully"
}
```

#### POST /auth/logout
Logout and invalidate session.

**Response:**
```json
{
  "message": "Logged out successfully"
}
```

## Database Schema

### Users Table
- `id`: UUID primary key
- `email`: Unique email address
- `username`: Unique username
- `password_hash`: Bcrypt hashed password
- `full_name`: Optional full name
- `avatar_url`: Optional avatar URL
- `role`: User role (user/admin/premium)
- `is_active`: Account status
- `email_verified`: Email verification status
- `created_at`: Registration timestamp
- `updated_at`: Last update timestamp
- `last_login`: Last login timestamp
- `metadata`: JSONB for additional data

### Sessions Table
- `id`: UUID primary key
- `user_id`: Foreign key to users
- `token_hash`: SHA256 hash of access token
- `refresh_token_hash`: SHA256 hash of refresh token
- `ip_address`: Client IP address
- `user_agent`: Client user agent
- `expires_at`: Token expiration
- `created_at`: Session creation time
- `last_activity`: Last activity timestamp

### Additional Tables
- `chat_sessions`: Store user chat history
- `user_documents`: RAG document storage
- `api_key_listings`: Marketplace API keys
- `marketplace_transactions`: Transaction records
- `billing`: Usage and billing records
- `user_api_keys`: User's personal API keys
- `audit_logs`: Security audit trail

## Security Considerations

1. **Password Requirements**: Minimum 8 characters
2. **Token Security**: Tokens are hashed before storing in database
3. **HTTPS Only**: Always use HTTPS in production
4. **Rate Limiting**: Implement rate limiting on auth endpoints
5. **CORS Configuration**: Configure appropriate CORS headers
6. **Secret Management**: Use strong JWT secret and rotate regularly

## Testing

Run authentication tests:

```bash
# Set test database URL
export TEST_DATABASE_URL="postgresql://postgres:postgres@localhost:5432/aichat_test"

# Run tests
cargo test auth::tests
```

## Integration with AIChat

The authentication system integrates seamlessly with AIChat's existing infrastructure:

1. **Server Integration**: Auth middleware automatically protects API endpoints
2. **Web UI Integration**: Enhanced GUI includes login/register forms
3. **Session Persistence**: Chat sessions are linked to authenticated users
4. **API Key Management**: Users can manage their API keys through the marketplace

## Deployment

### Using Supabase

1. Create a new Supabase project
2. Get the connection string from Settings > Database
3. Set `DATABASE_URL` with the connection string
4. The schema will be created automatically on first run

### Using Self-Hosted PostgreSQL

1. Install PostgreSQL
2. Create a database: `CREATE DATABASE aichat;`
3. Set `DATABASE_URL` environment variable
4. Start the server to run migrations

### Docker Deployment

```dockerfile
ENV DATABASE_URL=postgresql://user:pass@db:5432/aichat
ENV JWT_SECRET=your-secure-secret
```

## Troubleshooting

### Common Issues

1. **"Database connection failed"**
   - Check `DATABASE_URL` is set correctly
   - Ensure PostgreSQL is running
   - Verify network connectivity

2. **"JWT validation failed"**
   - Check token hasn't expired
   - Ensure `JWT_SECRET` matches between restarts
   - Verify token format in Authorization header

3. **"User already exists"**
   - Email or username is already registered
   - Use different credentials or login instead

## Future Enhancements

- [ ] Email verification
- [ ] OAuth2 providers (Google, GitHub)
- [ ] Two-factor authentication
- [ ] Password reset via email
- [ ] API rate limiting per user
- [ ] Advanced session management
- [ ] Webhook notifications for auth events
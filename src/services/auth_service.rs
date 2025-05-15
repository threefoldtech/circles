use crate::models::user::{User, UserPreferences};
use crate::utils::naming::{CircleName, NameError, NameRegistry};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur in the authentication service
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Invalid name: {0}")]
    InvalidName(#[from] NameError),

    #[error("Password too weak")]
    WeakPassword,

    #[error("Session expired")]
    SessionExpired,

    #[error("Invalid session")]
    InvalidSession,
}

/// Result type for authentication operations
pub type AuthResult<T> = Result<T, AuthError>;

/// Represents user credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// User's name (e.g., "user.john")
    pub name: String,
    /// User's email
    pub email: String,
    /// User's password (hashed)
    pub password_hash: String,
}

/// Represents a user session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session ID
    pub id: Uuid,
    /// User ID
    pub user_id: Uuid,
    /// When the session was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the session expires
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Session data
    pub data: HashMap<String, String>,
}

/// Service for managing user authentication and identity
#[derive(Debug)]
pub struct AuthService {
    /// Users in the system
    users: HashMap<Uuid, User>,
    /// User credentials
    credentials: HashMap<Uuid, Credentials>,
    /// Active sessions
    sessions: HashMap<Uuid, Session>,
    /// Name registry for user names
    name_registry: NameRegistry,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            credentials: HashMap::new(),
            sessions: HashMap::new(),
            name_registry: NameRegistry::new(),
        }
    }

    /// Register a new user
    pub fn register_user(&mut self, name: &str, email: &str, password: &str) -> AuthResult<Uuid> {
        // Parse and validate the name
        let user_name = CircleName::new(name)?;

        // Check if the name is already taken
        if self.name_registry.get_record(name).is_some() {
            return Err(AuthError::UserAlreadyExists);
        }

        // Check if the email is already taken
        if self.credentials.values().any(|c| c.email == email) {
            return Err(AuthError::UserAlreadyExists);
        }

        // Validate password strength
        if !self.is_password_strong(password) {
            return Err(AuthError::WeakPassword);
        }

        // Create the user
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            name: name.to_string(),
            email: email.to_string(),
            created_at: Utc::now(),
            preferences: UserPreferences::default(),
        };

        // Hash the password
        let password_hash = self.hash_password(password);

        // Create credentials
        let credentials = Credentials {
            name: name.to_string(),
            email: email.to_string(),
            password_hash,
        };

        // Register the name
        self.name_registry
            .register_name(user_name, user_id)
            .map_err(|e| AuthError::InvalidName(e))?;

        // Store the user and credentials
        self.users.insert(user_id, user);
        self.credentials.insert(user_id, credentials);

        Ok(user_id)
    }

    /// Authenticate a user
    pub fn authenticate(&mut self, email: &str, password: &str) -> AuthResult<Session> {
        // Find the user by email
        let user_id = self
            .credentials
            .iter()
            .find(|(_, c)| c.email == email)
            .map(|(id, _)| *id)
            .ok_or(AuthError::InvalidCredentials)?;

        // Check the password
        let credentials = self.credentials.get(&user_id).unwrap();
        if !self.verify_password(password, &credentials.password_hash) {
            return Err(AuthError::InvalidCredentials);
        }

        // Create a session
        let session = self.create_session(user_id);

        Ok(session)
    }

    /// Get a user by ID
    pub fn get_user(&self, user_id: Uuid) -> AuthResult<&User> {
        self.users.get(&user_id).ok_or(AuthError::UserNotFound)
    }

    /// Find a user by email
    pub fn find_user_by_email(&self, email: &str) -> Option<(Uuid, &User)> {
        // Find the user ID by email
        let user_id = self
            .credentials
            .iter()
            .find(|(_, c)| c.email == email)
            .map(|(id, _)| *id);

        // Get the user
        user_id.and_then(|id| self.users.get(&id).map(|user| (id, user)))
    }

    /// Get a user by session ID
    pub fn get_user_by_session(&self, session_id: Uuid) -> AuthResult<&User> {
        let session = self
            .sessions
            .get(&session_id)
            .ok_or(AuthError::InvalidSession)?;

        // Check if the session has expired
        if Utc::now() > session.expires_at {
            return Err(AuthError::SessionExpired);
        }

        self.get_user(session.user_id)
    }

    /// Update user preferences
    pub fn update_preferences(
        &mut self,
        user_id: Uuid,
        preferences: UserPreferences,
    ) -> AuthResult<()> {
        let user = self
            .users
            .get_mut(&user_id)
            .ok_or(AuthError::UserNotFound)?;

        user.preferences = preferences;

        Ok(())
    }

    /// Change a user's password
    pub fn change_password(
        &mut self,
        user_id: Uuid,
        old_password: &str,
        new_password: &str,
    ) -> AuthResult<()> {
        // Check if the user exists
        let credentials = self
            .credentials
            .get(&user_id)
            .ok_or(AuthError::UserNotFound)?;

        // Verify the old password
        if !self.verify_password(old_password, &credentials.password_hash) {
            return Err(AuthError::InvalidCredentials);
        }

        // Validate the new password
        if !self.is_password_strong(new_password) {
            return Err(AuthError::WeakPassword);
        }

        // Hash the new password
        let password_hash = self.hash_password(new_password);

        // Update the credentials
        let credentials = self.credentials.get_mut(&user_id).unwrap();
        credentials.password_hash = password_hash;

        Ok(())
    }

    /// Create a session for a user
    fn create_session(&mut self, user_id: Uuid) -> Session {
        let session_id = Uuid::new_v4();
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(24); // 24-hour session

        let session = Session {
            id: session_id,
            user_id,
            created_at: now,
            expires_at,
            data: HashMap::new(),
        };

        self.sessions.insert(session_id, session.clone());

        session
    }

    /// Invalidate a session
    pub fn invalidate_session(&mut self, session_id: Uuid) -> AuthResult<()> {
        self.sessions
            .remove(&session_id)
            .ok_or(AuthError::InvalidSession)?;

        Ok(())
    }

    /// Check if a password is strong enough
    fn is_password_strong(&self, password: &str) -> bool {
        // Password must be at least 8 characters long
        if password.len() < 8 {
            return false;
        }

        // Password must contain at least one uppercase letter
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        if !has_uppercase {
            return false;
        }

        // Password must contain at least one lowercase letter
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        if !has_lowercase {
            return false;
        }

        // Password must contain at least one digit
        let has_digit = password.chars().any(|c| c.is_digit(10));
        if !has_digit {
            return false;
        }

        true
    }

    /// Hash a password
    fn hash_password(&self, password: &str) -> String {
        // In a real implementation, this would use a proper password hashing algorithm
        // like bcrypt, Argon2, or PBKDF2. For this prototype, we'll use a simple hash.
        format!("hashed:{}", password)
    }

    /// Verify a password against a hash
    fn verify_password(&self, password: &str, hash: &str) -> bool {
        // In a real implementation, this would use the appropriate verification function
        // for the hashing algorithm used. For this prototype, we'll use a simple check.
        hash == &format!("hashed:{}", password)
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}

/// Save credentials to disk
pub fn save_credentials(credentials: &Credentials) -> std::io::Result<()> {
    // In a real implementation, this would securely save credentials to disk
    // For this prototype, we'll just pretend it works
    println!("Saving credentials for {}", credentials.name);
    Ok(())
}

/// Load credentials from disk
pub fn load_credentials() -> std::io::Result<Credentials> {
    // In a real implementation, this would load credentials from disk
    // For this prototype, we'll return a dummy user
    Ok(Credentials {
        name: "user.demo".to_string(),
        email: "demo@example.com".to_string(),
        password_hash: "hashed:password".to_string(),
    })
}

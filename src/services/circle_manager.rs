use crate::models::circle::{Circle, CircleSettings, CircleType, Member, Role};
use crate::models::features::Features;
use crate::models::user::User;
use crate::utils::naming::{CircleName, NameError, NameRegistry};
use chrono::Utc;
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Errors that can occur in the circle manager
#[derive(Debug, Error)]
pub enum CircleManagerError {
    #[error("Circle not found")]
    CircleNotFound,

    #[error("User not found")]
    UserNotFound,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid circle name: {0}")]
    InvalidName(#[from] NameError),

    #[error("Circle already exists")]
    CircleAlreadyExists,

    #[error("Member already exists in circle")]
    MemberAlreadyExists,

    #[error("Member not found in circle")]
    MemberNotFound,

    #[error("Cannot remove the last administrator")]
    CannotRemoveLastAdmin,

    #[error("Circle is locked")]
    CircleLocked,
}

/// Result type for circle manager operations
pub type CircleResult<T> = Result<T, CircleManagerError>;

/// Service for managing circles
#[derive(Debug)]
pub struct CircleManager {
    /// All circles in the system
    circles: HashMap<Uuid, Circle>,
    /// Name registry for circle names
    name_registry: NameRegistry,
    /// Access tokens for external access
    access_tokens: HashMap<String, (Uuid, Role)>,
    /// Circles that are locked (cannot be modified)
    locked_circles: Vec<Uuid>,
}

impl CircleManager {
    /// Create a new circle manager
    pub fn new() -> Self {
        Self {
            circles: HashMap::new(),
            name_registry: NameRegistry::new(),
            access_tokens: HashMap::new(),
            locked_circles: Vec::new(),
        }
    }

    /// Create a new circle
    pub fn create_circle(
        &mut self,
        name: &str,
        circle_type: CircleType,
        creator: &User,
        settings: Option<CircleSettings>,
    ) -> CircleResult<Uuid> {
        // Parse and validate the name
        let circle_name = CircleName::new(name)?;

        // Check if the name is already taken
        if self.name_registry.get_record(name).is_some() {
            return Err(CircleManagerError::CircleAlreadyExists);
        }

        // Create the circle
        let mut circle = Circle::new(name.to_string(), circle_type, creator.id);

        // Update the creator's name
        if let Some(member) = circle.members.iter_mut().find(|m| m.user_id == creator.id) {
            member.name = creator.name.clone();
        }

        // Apply custom settings if provided
        if let Some(custom_settings) = settings {
            circle.settings = custom_settings;
        }

        // Register the name
        self.name_registry
            .register_name(circle_name, circle.id)
            .map_err(|_| CircleManagerError::CircleAlreadyExists)?;

        // Store the circle
        let circle_id = circle.id;
        self.circles.insert(circle_id, circle);

        Ok(circle_id)
    }

    /// Get a circle by ID
    pub fn get_circle(&self, circle_id: Uuid) -> CircleResult<&Circle> {
        self.circles
            .get(&circle_id)
            .ok_or(CircleManagerError::CircleNotFound)
    }

    /// Get a mutable reference to a circle by ID
    fn get_circle_mut(&mut self, circle_id: Uuid) -> CircleResult<&mut Circle> {
        // Check if the circle is locked
        if self.locked_circles.contains(&circle_id) {
            return Err(CircleManagerError::CircleLocked);
        }

        self.circles
            .get_mut(&circle_id)
            .ok_or(CircleManagerError::CircleNotFound)
    }

    /// Get a circle by name
    pub fn get_circle_by_name(&self, name: &str) -> CircleResult<&Circle> {
        // Resolve the name to a circle ID
        let entries = self
            .name_registry
            .resolve(name)
            .ok_or(CircleManagerError::CircleNotFound)?;

        // Find the first circle entry
        for entry in entries {
            if let crate::utils::naming::RecordEntry::Circle(circle_id) = entry {
                return self.get_circle(*circle_id);
            }
        }

        Err(CircleManagerError::CircleNotFound)
    }

    /// Get all circles
    pub fn get_all_circles(&self) -> Vec<&Circle> {
        self.circles.values().collect()
    }

    /// Get circles for a user
    pub fn get_user_circles(&self, user_id: Uuid) -> Vec<&Circle> {
        self.circles
            .values()
            .filter(|circle| {
                circle
                    .members
                    .iter()
                    .any(|member| member.user_id == user_id)
            })
            .collect()
    }

    /// Add a member to a circle
    pub fn add_member(
        &mut self,
        circle_id: Uuid,
        user: &User,
        role: Role,
        added_by: Uuid,
    ) -> CircleResult<()> {
        // Check if the user adding the member has permission
        self.check_permission(circle_id, added_by, Role::Administrator)?;

        let circle = self.get_circle_mut(circle_id)?;

        // Check if the user is already a member
        if circle.members.iter().any(|m| m.user_id == user.id) {
            return Err(CircleManagerError::MemberAlreadyExists);
        }

        // Add the member
        let member = Member {
            user_id: user.id,
            name: user.name.clone(),
            role,
            joined_at: Utc::now(),
        };

        circle.members.push(member);

        Ok(())
    }

    /// Remove a member from a circle
    pub fn remove_member(
        &mut self,
        circle_id: Uuid,
        user_id: Uuid,
        removed_by: Uuid,
    ) -> CircleResult<()> {
        // Check if the user removing the member has permission
        self.check_permission(circle_id, removed_by, Role::Administrator)?;

        let circle = self.get_circle_mut(circle_id)?;

        // First, check if the member exists and count admins
        let mut member_index = None;
        let mut admin_count = 0;

        for (idx, member) in circle.members.iter().enumerate() {
            if member.user_id == user_id {
                member_index = Some(idx);
            }

            if member.role == Role::Administrator {
                admin_count += 1;
            }
        }

        let member_index = member_index.ok_or(CircleManagerError::MemberNotFound)?;
        let is_admin = circle.members[member_index].role == Role::Administrator;

        // Check if this is the last administrator
        if is_admin && admin_count <= 1 {
            return Err(CircleManagerError::CannotRemoveLastAdmin);
        }

        // Remove the member
        circle.members.remove(member_index);

        Ok(())
    }

    /// Update a member's role
    pub fn update_member_role(
        &mut self,
        circle_id: Uuid,
        user_id: Uuid,
        new_role: Role,
        updated_by: Uuid,
    ) -> CircleResult<()> {
        // Check if the user updating the role has permission
        self.check_permission(circle_id, updated_by, Role::Administrator)?;

        let circle = self.get_circle_mut(circle_id)?;

        // First, check if the member exists and count admins
        let mut member_index = None;
        let mut admin_count = 0;

        for (idx, member) in circle.members.iter().enumerate() {
            if member.user_id == user_id {
                member_index = Some(idx);
            }

            if member.role == Role::Administrator {
                admin_count += 1;
            }
        }

        let member_index = member_index.ok_or(CircleManagerError::MemberNotFound)?;
        let is_admin = circle.members[member_index].role == Role::Administrator;

        // Check if this would remove the last administrator
        if is_admin && admin_count <= 1 && new_role != Role::Administrator {
            return Err(CircleManagerError::CannotRemoveLastAdmin);
        }

        // Update the role
        circle.members[member_index].role = new_role;

        Ok(())
    }

    /// Update circle settings
    pub fn update_circle_settings(
        &mut self,
        circle_id: Uuid,
        settings: CircleSettings,
        updated_by: Uuid,
    ) -> CircleResult<()> {
        // Check if the user updating the settings has permission
        self.check_permission(circle_id, updated_by, Role::Administrator)?;

        let circle = self.get_circle_mut(circle_id)?;

        // Update the settings
        circle.settings = settings;

        Ok(())
    }

    /// Update circle features
    pub fn update_circle_features(
        &mut self,
        circle_id: Uuid,
        features: Features,
        updated_by: Uuid,
    ) -> CircleResult<()> {
        // Check if the user updating the features has permission
        self.check_permission(circle_id, updated_by, Role::Administrator)?;

        let circle = self.get_circle_mut(circle_id)?;

        // Update the features
        circle.features = features;

        Ok(())
    }

    /// Delete a circle
    pub fn delete_circle(&mut self, circle_id: Uuid, deleted_by: Uuid) -> CircleResult<()> {
        // Check if the user deleting the circle has permission
        self.check_permission(circle_id, deleted_by, Role::Administrator)?;

        // Remove the circle
        self.circles
            .remove(&circle_id)
            .ok_or(CircleManagerError::CircleNotFound)?;

        // TODO: Remove the name from the registry

        Ok(())
    }

    /// Check if a user has permission to perform an action
    pub fn check_permission(
        &self,
        circle_id: Uuid,
        user_id: Uuid,
        required_role: Role,
    ) -> CircleResult<()> {
        let circle = self.get_circle(circle_id)?;

        // Find the user's role
        let user_role = circle
            .members
            .iter()
            .find(|m| m.user_id == user_id)
            .map(|m| m.role);

        match user_role {
            Some(role) => {
                // Check if the role is sufficient
                let has_permission = match required_role {
                    Role::Read => true, // Any role can read
                    Role::Write => {
                        role == Role::Write
                            || role == Role::Administrator
                            || role == Role::Coordinator
                    }
                    Role::Coordinator => role == Role::Coordinator || role == Role::Administrator,
                    Role::Administrator => role == Role::Administrator,
                };

                if has_permission {
                    Ok(())
                } else {
                    Err(CircleManagerError::PermissionDenied(format!(
                        "User does not have the required role: {:?}",
                        required_role
                    )))
                }
            }
            None => Err(CircleManagerError::PermissionDenied(
                "User is not a member of the circle".to_string(),
            )),
        }
    }

    /// Create an access token for external access
    pub fn create_access_token(
        &mut self,
        circle_id: Uuid,
        role: Role,
        created_by: Uuid,
    ) -> CircleResult<String> {
        // Check if the user creating the token has permission
        self.check_permission(circle_id, created_by, Role::Administrator)?;

        // Generate a token
        let token = crate::utils::naming::generate_access_token();

        // Store the token
        self.access_tokens.insert(token.clone(), (circle_id, role));

        Ok(token)
    }

    /// Get a circle using an access token
    pub fn get_circle_by_token(&self, token: &str) -> CircleResult<(&Circle, Role)> {
        let (circle_id, role) =
            self.access_tokens
                .get(token)
                .ok_or(CircleManagerError::PermissionDenied(
                    "Invalid access token".to_string(),
                ))?;

        let circle = self.get_circle(*circle_id)?;

        Ok((circle, *role))
    }

    /// Lock a circle to prevent modifications
    pub fn lock_circle(&mut self, circle_id: Uuid) {
        if !self.locked_circles.contains(&circle_id) {
            self.locked_circles.push(circle_id);
        }
    }

    /// Unlock a circle to allow modifications
    pub fn unlock_circle(&mut self, circle_id: Uuid) {
        self.locked_circles.retain(|id| *id != circle_id);
    }
}

impl Default for CircleManager {
    fn default() -> Self {
        Self::new()
    }
}

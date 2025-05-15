use crate::models::circle::Role;
use uuid::Uuid;

/// State for the add member dialog
#[derive(Debug, Clone)]
pub struct AddMemberState {
    /// Whether the dialog is open
    pub open: bool,
    /// ID of the circle to add member to
    pub circle_id: Option<Uuid>,
    /// Circle name for display
    pub circle_name: String,
    /// Email of the member to add
    pub member_email: String,
    /// Selected role for the new member
    pub selected_role: Role,
}

impl Default for AddMemberState {
    fn default() -> Self {
        Self {
            open: false,
            circle_id: None,
            circle_name: String::new(),
            member_email: String::new(),
            selected_role: Role::Read, // Default to Read permission
        }
    }
}

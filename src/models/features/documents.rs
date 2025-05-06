use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// State for the documents feature
#[derive(Debug, Clone, Default)]
pub struct DocumentsState {
    /// Current view mode (grid or list)
    pub view_mode: ViewMode,
    /// Current folder path (breadcrumbs)
    pub current_path: Vec<FolderBreadcrumb>,
    /// Current folder ID
    pub current_folder_id: Option<Uuid>,
    /// Search query
    pub search_query: String,
    /// Selected item for context menu
    pub selected_item: Option<SelectedItem>,
    /// Rename dialog state
    pub rename_dialog: RenameDialogState,
    /// Delete dialog state
    pub delete_dialog: DeleteDialogState,
    /// Create new dialog state
    pub create_dialog: CreateDialogState,
}

/// View mode for documents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewMode {
    /// Grid view (icons)
    Grid,
    /// List view (details)
    List,
}

impl Default for ViewMode {
    fn default() -> Self {
        Self::Grid
    }
}

/// Breadcrumb for folder navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderBreadcrumb {
    /// Folder ID
    pub id: Uuid,
    /// Folder name
    pub name: String,
}

/// Selected item for context menu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectedItem {
    /// Selected folder
    Folder(Uuid),
    /// Selected document
    Document(Uuid),
}

/// Rename dialog state
#[derive(Debug, Clone, Default)]
pub struct RenameDialogState {
    /// Whether the dialog is open
    pub open: bool,
    /// Item being renamed
    pub item: Option<SelectedItem>,
    /// Current name
    pub current_name: String,
    /// New name
    pub new_name: String,
}

/// Delete dialog state
#[derive(Debug, Clone, Default)]
pub struct DeleteDialogState {
    /// Whether the dialog is open
    pub open: bool,
    /// Item being deleted
    pub item: Option<SelectedItem>,
    /// Item name
    pub item_name: String,
}

/// Create dialog state
#[derive(Debug, Clone, Default)]
pub struct CreateDialogState {
    /// Whether the dialog is open
    pub open: bool,
    /// Type of item to create
    pub create_type: Option<CreateType>,
    /// New item name
    pub name: String,
}

/// Type of item to create
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateType {
    /// Create a new folder
    Folder,
    /// Create a new document
    Document,
}

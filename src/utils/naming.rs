use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use uuid::Uuid;

/// Represents a name in the system with two parts (e.g., "project.sunflower")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CircleName {
    /// First part of the name (e.g., "project")
    pub prefix: String,
    /// Second part of the name (e.g., "sunflower")
    pub suffix: String,
}

/// Represents a name record that can point to various resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameRecord {
    /// The name this record is for
    pub name: CircleName,
    /// The owner of this name
    pub owner_id: Uuid,
    /// When the name was registered
    pub registered_at: chrono::DateTime<chrono::Utc>,
    /// When the name expires
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Cost of the name in dollars per year
    pub annual_cost: f64,
    /// Record entries
    pub entries: Vec<RecordEntry>,
    /// Aliases for this name
    pub aliases: Vec<CircleName>,
    /// Whether this name is permanent (cannot be changed)
    pub is_permanent: bool,
}

/// Types of record entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordEntry {
    /// Points to an IP address
    IP(IpAddr),
    /// Points to a URL
    URL(String),
    /// Points to a circle
    Circle(Uuid),
    /// Contains text data
    TXT(String),
    /// Contains custom metadata
    Metadata(HashMap<String, String>),
}

impl CircleName {
    /// Create a new CircleName from a string in the format "prefix.suffix"
    pub fn new(name: &str) -> Result<Self, NameError> {
        let parts: Vec<&str> = name.split('.').collect();

        if parts.len() != 2 {
            return Err(NameError::InvalidFormat);
        }

        let prefix = parts[0].to_string();
        let suffix = parts[1].to_string();

        // Validate each part
        if !Self::is_valid_part(&prefix) || !Self::is_valid_part(&suffix) {
            return Err(NameError::InvalidCharacters);
        }

        Ok(Self { prefix, suffix })
    }

    /// Check if a name part is valid (alphanumeric and hyphens only)
    fn is_valid_part(part: &str) -> bool {
        if part.is_empty() {
            return false;
        }

        part.chars().all(|c| c.is_alphanumeric() || c == '-')
    }

    /// Get the full name as a string
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.prefix, self.suffix)
    }

    /// Calculate the cost of this name based on length
    pub fn calculate_cost(&self) -> f64 {
        // Base cost is $1 per year for 2 parts of 8 characters each
        let base_cost = 1.0;

        // Premium names (short names) cost more
        let length_factor = if self.prefix.len() <= 3 || self.suffix.len() <= 3 {
            // Short names are premium
            5.0
        } else if self.prefix.len() <= 5 || self.suffix.len() <= 5 {
            // Medium-length names are somewhat premium
            2.0
        } else {
            // Standard length names use the base cost
            1.0
        };

        base_cost * length_factor
    }
}

impl NameRecord {
    /// Create a new name record
    pub fn new(name: CircleName, owner_id: Uuid) -> Self {
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::days(365); // 1 year by default
        let annual_cost = name.calculate_cost();

        Self {
            name,
            owner_id,
            registered_at: now,
            expires_at,
            annual_cost,
            entries: Vec::new(),
            aliases: Vec::new(),
            is_permanent: false,
        }
    }

    /// Add a record entry
    pub fn add_entry(&mut self, entry: RecordEntry) {
        self.entries.push(entry);
    }

    /// Add an alias
    pub fn add_alias(&mut self, alias: CircleName) {
        self.aliases.push(alias);
    }

    /// Make this name permanent
    pub fn make_permanent(&mut self) {
        self.is_permanent = true;
    }

    /// Renew the name for another year
    pub fn renew(&mut self) {
        self.expires_at = self.expires_at + chrono::Duration::days(365);
    }
}

/// Errors that can occur when working with names
#[derive(Debug, thiserror::Error)]
pub enum NameError {
    #[error("Invalid name format. Names must be in the format 'prefix.suffix'")]
    InvalidFormat,

    #[error(
        "Invalid characters in name. Names can only contain alphanumeric characters and hyphens"
    )]
    InvalidCharacters,

    #[error("Name already exists")]
    AlreadyExists,

    #[error("Name not found")]
    NotFound,

    #[error("Name is permanent and cannot be changed")]
    Permanent,
}

/// Registry for managing names
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NameRegistry {
    /// Map of names to records
    names: HashMap<String, NameRecord>,
    /// Map of UUIDs to names
    id_to_name: HashMap<Uuid, Vec<String>>,
}

impl NameRegistry {
    /// Create a new name registry
    pub fn new() -> Self {
        Self {
            names: HashMap::new(),
            id_to_name: HashMap::new(),
        }
    }

    /// Register a new name
    pub fn register_name(&mut self, name: CircleName, owner_id: Uuid) -> Result<(), NameError> {
        let name_str = name.to_string();

        if self.names.contains_key(&name_str) {
            return Err(NameError::AlreadyExists);
        }

        let record = NameRecord::new(name, owner_id);
        self.names.insert(name_str.clone(), record);

        // Update the reverse mapping
        self.id_to_name
            .entry(owner_id)
            .or_insert_with(Vec::new)
            .push(name_str);

        Ok(())
    }

    /// Get a name record by name
    pub fn get_record(&self, name: &str) -> Option<&NameRecord> {
        self.names.get(name)
    }

    /// Get a mutable name record by name
    pub fn get_record_mut(&mut self, name: &str) -> Option<&mut NameRecord> {
        self.names.get_mut(name)
    }

    /// Get all names for an ID
    pub fn get_names_for_id(&self, id: Uuid) -> Vec<String> {
        self.id_to_name.get(&id).cloned().unwrap_or_default()
    }

    /// Add an alias for a name
    pub fn add_alias(&mut self, name: &str, alias: CircleName) -> Result<(), NameError> {
        let alias_str = alias.to_string();

        if self.names.contains_key(&alias_str) {
            return Err(NameError::AlreadyExists);
        }

        let record = self.get_record_mut(name).ok_or(NameError::NotFound)?;

        if record.is_permanent {
            return Err(NameError::Permanent);
        }

        record.add_alias(alias);

        Ok(())
    }

    /// Resolve a name to its record entries
    pub fn resolve(&self, name: &str) -> Option<&Vec<RecordEntry>> {
        self.names.get(name).map(|record| &record.entries)
    }
}

/// Generate a token-style name for external access
pub fn generate_access_token() -> String {
    use rand::{Rng, thread_rng};
    let mut rng = thread_rng();

    // Generate a random 8-character token
    let token: String = (0..8)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            match idx {
                0..=9 => (b'0' + idx as u8) as char,
                10..=35 => (b'a' + (idx - 10) as u8) as char,
                _ => (b'A' + (idx - 36) as u8) as char,
            }
        })
        .collect();

    token
}

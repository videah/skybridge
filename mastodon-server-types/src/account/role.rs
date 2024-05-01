use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Role {
    /// The ID of the Role in the database.
    pub id: u32,
    /// The name of the role.
    pub name: String,
    /// The hex code assigned to this role. If no hex code is assigned, the string will be empty.
    pub color: String,
    /// A bitmask that represents the sum of all permissions granted to the role.
    pub permissions: u32,
    /// Whether the role is publicly visible as a badge on user profiles.
    pub highlighted: bool,
}

use serde::{Deserialize, Serialize};

use crate::prelude::{File, PermissionOverride};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TextChannel {
    #[serde(rename = "server")]
    server_id: String,
    #[serde(rename = "_id")]
    id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    icon: Option<File>,
    #[serde(default)]
    last_message_id: Option<String>,
    #[serde(default)]
    default_permissions: PermissionOverride,
    #[serde(default)]
    role_permissions: PermissionOverride,
    #[serde(default)]
    nsfw: bool,
}

impl TextChannel {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    
    pub fn get_server_id(&self) -> String {
        self.server_id.clone()
    }

    pub fn get_description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn get_icon(&self) -> Option<File> {
        self.icon.clone()
    }

    pub fn get_last_message_id(&self) -> Option<String> {
        self.last_message_id.clone()
    }

    pub fn get_role_permissions(&self) -> PermissionOverride {
        self.role_permissions.clone()
    }

    pub fn get_default_permissions(&self) -> PermissionOverride {
        self.default_permissions.clone()
    }

    pub fn is_nsfw(&self) -> bool {
        self.nsfw
    }
}
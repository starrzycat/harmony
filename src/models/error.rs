use serde::Deserialize;

use super::{Permission, user::UserPermission};

/// Represents an error when performing an action with Revolt's API
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum RevoltError {
    #[serde(rename = "LabelMe")]
    Default,
    AlreadyOnboarded,
    UsernameTaken,
    InvalidUsername,
    UnknownUser,
    AlreadyFriends,
    AlreadySentRequest,
    Blocked,
    BlockedByOther,
    NotFriends,
    UnknownChannel,
    UnknownAttachment,
    UnknownMessage,
    CannotEditMessage,
    CannotJoinCall,
    TooManyAttachments { max: usize },
    TooManyReplies { max: usize },
    TooManyChannels { max: usize },
    TooManyEmbeds { max: usize },
    EmptyMessage,
    PayloadTooLarge,
    CannotRemoveYourself,
    GroupTooLarge { max: usize },
    AlreadyInGroup,
    NotInGroup,
    UnknownServer,
    InvalidRole,
    Banned,
    TooManyServers { max: usize },
    TooManyEmoji { max: usize },
    TooManyRoles { max: usize },
    ReachedMaximumBots,
    IsBot,
    BotIsPrivate,
    CannotReportYourself,
    MissingPermission { permission: Permission },
    MissingUserPermission { permission: UserPermission },
    NotElevated,
    NotPrivileged,
    CannotGiveMissingPermissions,
    NotOwner,
    DatabaseError {
        operation: String,
        with: String
    },
    InternalError,
    InvalidOperation,
    InvalidCredentials,
    InvalidProperty,
    InvalidSession,
    DuplicateNonce,
    VosoUnavailable,
    NotFound,
    NoEffect,
    FailedValidation,
}
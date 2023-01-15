//! Module containing admin interface
use serde::{Deserialize, Serialize};

/// Object instance-level block with full information available to admins.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AdminDomainBlock {
    /// The ID of the DomainBlock in the database.
    pub id: String,
    /// The domain that is not allowed to federate.
    pub domain: String,
    /// When the domain was blocked from federating (ISO 8601 Datetime).
    pub created_at: String,
    /// The policy to be applied by this domain block. One of:
    /// * silence = Account statuses from this domain will be hidden by default
    /// suspend = All incoming data from this domain will be rejected
    /// noop = Do nothing. Allows for rejecting media or reports
    pub severity: String,
    /// Whether to reject media attachments from this domain
    pub reject_media: bool,
    /// Whether to reject reports from this domain
    pub reject_reports: bool,
    /// Private comment.
    pub private_comment: Option<String>,
    /// Public comment.
    pub public_comment: Option<String>,
    /// Whether to obfuscate public displays of this domain block.
    pub obfuscate: bool,
}

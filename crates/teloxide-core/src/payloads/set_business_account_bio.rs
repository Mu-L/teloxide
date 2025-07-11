//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{BusinessConnectionId, True};

impl_payload! {
    /// Changes the bio of a managed business account. Requires the _can_change_bio_ business bot right. Returns _true_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetBusinessAccountBio (SetBusinessAccountBioSetters) => True {
        required {
            /// Unique identifier of the business connection
            pub business_connection_id: BusinessConnectionId,
        }
        optional {
            /// The new value of the bio for the business account; 0-140 characters
            pub bio: String [into],
        }
    }
}

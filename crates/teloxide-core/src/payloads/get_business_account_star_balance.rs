//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{BusinessConnectionId, StarAmount};

impl_payload! {
    /// Returns the amount of Telegram Stars owned by a managed business account. Requires the _can_view_gifts_and_stars_ business bot right. Returns [`StarAmount`] on success.
    ///
    /// [`StarAmount`]: crate::types::StarAmount
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub GetBusinessAccountStarBalance (GetBusinessAccountStarBalanceSetters) => StarAmount {
        required {
            /// Unique identifier of the business connection
            pub business_connection_id: BusinessConnectionId,
        }
    }
}

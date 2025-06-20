//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{ChatInviteLink, Recipient, Seconds};

impl_payload! {
    /// Use this method to create a subscription invite link for a channel chat. The bot must have the can_invite_users administrator rights. The link can be edited using the method [`EditChatSubscriptionInviteLink`] or revoked using the method [`RevokeChatInviteLink`]. Returns the new invite link as a [`ChatInviteLink`] object.
    ///
    /// [`ChatInviteLink`]: crate::types::ChatInviteLink
    /// [`EditChatSubscriptionInviteLink`]: crate::payloads::EditChatSubscriptionInviteLink
    /// [`RevokeChatInviteLink`]: crate::payloads::RevokeChatInviteLink
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub CreateChatSubscriptionInviteLink (CreateChatSubscriptionInviteLinkSetters) => ChatInviteLink {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days).
            pub subscription_period: Seconds,
            /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-10000
            pub subscription_price: u32,
        }
        optional {
            /// Invite link name; 0-32 characters
            pub name: String [into],
        }
    }
}

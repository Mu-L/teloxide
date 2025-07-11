//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{InlineQueryResult, PreparedInlineMessage, UserId};

impl_payload! {
    /// Stores a message that can be sent by a user of a Mini App. Returns a [`PreparedInlineMessage`] object.
    ///
    /// [`PreparedInlineMessage`]: crate::types::PreparedInlineMessage
    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub SavePreparedInlineMessage (SavePreparedInlineMessageSetters) => PreparedInlineMessage {
        required {
            /// Unique identifier of the target user that can use the prepared message
            pub user_id: UserId,
            /// An object describing the message to be sent
            pub result: InlineQueryResult,
        }
        optional {
            /// Pass `true`, if the message can be sent to private chats with users
            pub allow_user_chats: bool,
            /// Pass `true`, if the message can be sent to private chats with bots
            pub allow_bot_chats: bool,
            /// Pass `true`, if the message can be sent to group and supergroup chats
            pub allow_group_chats: bool,
            /// Pass `true`, if the message can be sent to channel chats
            pub allow_channel_chats: bool,
        }
    }
}

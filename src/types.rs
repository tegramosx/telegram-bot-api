/// All types used in the Bot API responses are represented as JSON-objects.
/// It is safe to use 32-bit signed integers for storing all Integer fields unless otherwise noted.
/// Optional fields may be not returned when irrelevant.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// This object represents an incoming update.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// Optional. New incoming message of any kind - text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// Optional. New version of a message that is known to the bot and was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,
    /// Optional. New incoming channel post of any kind - text, photo, sticker, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Message>,
    /// Optional. New version of a channel post that is known to the bot and was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Message>,
    /// Optional. New incoming inline query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,
    /// Optional. The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// Optional. New incoming callback query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,
    /// Optional. New incoming shipping query. Only for invoices with flexible price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,
    /// Optional. New incoming pre-checkout query. Contains full information about checkout
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// Optional. New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// Optional. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,
    /// Optional. The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,
    /// Optional. A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify “chat_member” in the list of allowed_updates to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,
    /// Optional. A request to join the chat has been sent. The bot must have the can_invite_users administrator right in the chat to receive these updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,
}
impl Update {
    pub fn new(update_id: i64) -> Self {
        Self {
            update_id,
            message: None,
            edited_message: None,
            channel_post: None,
            edited_channel_post: None,
            inline_query: None,
            chosen_inline_result: None,
            callback_query: None,
            shipping_query: None,
            pre_checkout_query: None,
            poll: None,
            poll_answer: None,
            my_chat_member: None,
            chat_member: None,
            chat_join_request: None,
        }
    }
}

/// Describes the current status of a webhook.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i64,
    /// Optional. Currently used webhook IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Optional. Unix time for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,
    /// Optional. Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// Optional. Unix time of the most recent error that happened when trying to synchronize available updates with Telegram datacenters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i64>,
    /// Optional. The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    /// Optional. A list of update types the bot is subscribed to. Defaults to all update types except chat_member
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}
impl WebhookInfo {
    pub fn new(url: String, has_custom_certificate: bool, pending_update_count: i64) -> Self {
        Self {
            url,
            has_custom_certificate,
            pending_update_count,
            ip_address: None,
            last_error_date: None,
            last_error_message: None,
            last_synchronization_error_date: None,
            max_connections: None,
            allowed_updates: None,
        }
    }
}

/// This object represents a Telegram user or bot.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    /// Unique identifier for this user or bot. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: String,
    /// Optional. User's or bot's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. User's or bot's username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. IETF language tag of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Optional. True, if this user is a Telegram Premium user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// Optional. True, if this user added the bot to the attachment menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    /// Optional. True, if the bot can be invited to groups. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    /// Optional. True, if privacy mode is disabled for the bot. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    /// Optional. True, if the bot supports inline queries. Returned only in getMe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
}
impl User {
    pub fn new(id: i64, is_bot: bool, first_name: String) -> Self {
        Self {
            id,
            is_bot,
            first_name,
            last_name: None,
            username: None,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
        }
    }
}

/// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ChatType {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "supergroup")]
    Supergroup,
    #[serde(rename = "channel")]
    Channel,
}

/// This object represents a chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Chat {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    type_name: ChatType,
    /// Optional. Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Optional. First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Optional. Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Chat photo. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    /// Optional. Bio of the other party in a private chat. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Optional. True, if privacy settings of the other party in the private chat allows to use tg://user?id=<user_id> links only in chats with the user. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    /// Optional. True, if the privacy settings of the other party restrict sending voice and video note messages in the private chat. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    /// Optional. True, if users need to join the supergroup before they can send messages. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    /// Optional. True, if all users directly joining the supergroup need to be approved by supergroup administrators. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    /// Optional. Description, for groups, supergroups and channel chats. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Primary invite link, for groups, supergroups and channel chats. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    /// Optional. The most recent pinned message (by sending date). Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Message>,
    /// Optional. Default chat member permissions, for groups and supergroups. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    /// Optional. For supergroups, the minimum allowed delay between consecutive messages sent by each unpriviledged user; in seconds. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i64>,
    /// Optional. The time after which all messages sent to the chat will be automatically deleted; in seconds. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i64>,
    /// Optional. True, if messages from the chat can't be forwarded to other chats. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// Optional. For supergroups, name of group sticker set. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    /// Optional. True, if the bot can change the group sticker set. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    /// Optional. Unique identifier for the linked chat, i.e. the discussion group identifier for a channel and vice versa; for supergroups and channel chats. This identifier may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,
    /// Optional. For supergroups, the location to which the supergroup is connected. Returned only in getChat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}
impl Chat {
    pub fn new(id: i64, type_name: ChatType) -> Self {
        Self {
            id,
            type_name,
            title: None,
            username: None,
            first_name: None,
            last_name: None,
            photo: None,
            bio: None,
            has_private_forwards: None,
            has_restricted_voice_and_video_messages: None,
            join_to_send_messages: None,
            join_by_request: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            permissions: None,
            slow_mode_delay: None,
            message_auto_delete_time: None,
            has_protected_content: None,
            sticker_set_name: None,
            can_set_sticker_set: None,
            linked_chat_id: None,
            location: None,
        }
    }
}

/// This object represents a message.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i64,
    /// Optional. Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,
    /// Optional. Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field from contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Box<Chat>>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Box<Chat>,
    /// Optional. For forwarded messages, sender of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from: Option<User>,
    /// Optional. For messages forwarded from channels or from anonymous administrators, information about the original sender chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_chat: Option<Box<Chat>>,
    /// Optional. For messages forwarded from channels, identifier of the original message in the channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_message_id: Option<i64>,
    /// Optional. For forwarded messages that were originally sent in channels or by an anonymous chat administrator, signature of the message sender if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_signature: Option<String>,
    /// Optional. Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_sender_name: Option<String>,
    /// Optional. For forwarded messages, date the original message was sent in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_date: Option<i64>,
    /// Optional. True, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,
    /// Optional. For replies, the original message. Note that the Message object in this field will not contain further reply_to_message fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,
    /// Optional. Bot through which the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,
    /// Optional. Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,
    /// Optional. True, if the message can't be forwarded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// Optional. The unique identifier of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,
    /// Optional. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
    /// Optional. For text messages, the actual UTF-8 text of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    /// Optional. Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    /// Optional. Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    /// Optional. Message is a photo, available sizes of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Optional. Message is a sticker, information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    /// Optional. Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    /// Optional. Message is a video note, information about the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    /// Optional. Message is a voice message, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    /// Optional. Caption for the animation, audio, document, photo, video or voice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Message is a shared contact, information about the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    /// Optional. Message is a dice with random value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    /// Optional. Message is a game, information about the game. More about games »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    /// Optional. Message is a native poll, information about the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    /// Optional. Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
    /// Optional. Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Optional. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,
    /// Optional. A member was removed from the group, information about them (this member may be the bot itself)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,
    /// Optional. A chat title was changed to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,
    /// Optional. A chat photo was change to this value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    /// Optional. Service message: the chat photo was deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,
    /// Optional. Service message: the group has been created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,
    /// Optional. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,
    /// Optional. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,
    /// Optional. Service message: auto-delete timer settings changed in the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    /// Optional. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    /// Optional. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,
    /// Optional. Specified message was pinned. Note that the Message object in this field will not contain further reply_to_message fields even if it is itself a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    /// Optional. Message is an invoice for a payment, information about the invoice. More about payments »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    /// Optional. Message is a service message about a successful payment, information about the payment. More about payments »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,
    /// Optional. The domain name of the website on which the user has logged in. More about Telegram Login »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,
    /// Optional. Telegram Passport data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,
    /// Optional. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    /// Optional. Service message: video chat scheduled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    /// Optional. Service message: video chat started
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<VideoChatStarted>,
    /// Optional. Service message: video chat ended
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<VideoChatEnded>,
    /// Optional. Service message: new participants invited to a video chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    /// Optional. Service message: data sent by a Web App
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,
    /// Optional. Inline keyboard attached to the message. login_url buttons are represented as ordinary url buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl Message {
    pub fn new(message_id: i64, date: i64, chat: Box<Chat>) -> Self {
        Self {
            message_id,
            from: None,
            sender_chat: None,
            date,
            chat,
            forward_from: None,
            forward_from_chat: None,
            forward_from_message_id: None,
            forward_signature: None,
            forward_sender_name: None,
            forward_date: None,
            is_automatic_forward: None,
            reply_to_message: None,
            via_bot: None,
            edit_date: None,
            has_protected_content: None,
            media_group_id: None,
            author_signature: None,
            text: None,
            entities: None,
            animation: None,
            audio: None,
            document: None,
            photo: None,
            sticker: None,
            video: None,
            video_note: None,
            voice: None,
            caption: None,
            caption_entities: None,
            contact: None,
            dice: None,
            game: None,
            poll: None,
            venue: None,
            location: None,
            new_chat_members: None,
            left_chat_member: None,
            new_chat_title: None,
            new_chat_photo: None,
            delete_chat_photo: None,
            group_chat_created: None,
            supergroup_chat_created: None,
            channel_chat_created: None,
            message_auto_delete_timer_changed: None,
            migrate_to_chat_id: None,
            migrate_from_chat_id: None,
            pinned_message: None,
            invoice: None,
            successful_payment: None,
            connected_website: None,
            passport_data: None,
            proximity_alert_triggered: None,
            video_chat_scheduled: None,
            video_chat_started: None,
            video_chat_ended: None,
            video_chat_participants_invited: None,
            web_app_data: None,
            reply_markup: None,
        }
    }
}

/// This object represents a unique message identifier.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageId {
    /// Unique message identifier
    pub message_id: i64,
}
impl MessageId {
    pub fn new(message_id: i64) -> Self {
        Self { message_id }
    }
}

/// This object represents one special entity in a text message. For example, hashtags, usernames, URLs, etc.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageEntity {
    /// Type of the entity. Currently, can be “mention” (@username), “hashtag” (#hashtag), “cashtag” ($USD), “bot_command” (/start@jobs_bot), “url” (https://telegram.org), “email” (do-not-reply@telegram.org), “phone_number” (+1-212-555-0123), “bold” (bold text), “italic” (italic text), “underline” (underlined text), “strikethrough” (strikethrough text), “spoiler” (spoiler message), “code” (monowidth string), “pre” (monowidth block), “text_link” (for clickable text URLs), “text_mention” (for users without usernames), “custom_emoji” (for inline custom emoji stickers)
    #[serde(rename = "type")]
    pub type_name: String,
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// Optional. For “text_link” only, URL that will be opened after user taps on the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. For “text_mention” only, the mentioned user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// Optional. For “pre” only, the programming language of the entity text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Optional. For “custom_emoji” only, unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}
impl MessageEntity {
    pub fn new(type_name: String, offset: i64, length: i64) -> Self {
        Self {
            type_name,
            offset,
            length,
            url: None,
            user: None,
            language: None,
            custom_emoji_id: None,
        }
    }
}

/// This object represents one size of a photo or a file / sticker thumbnail.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Photo width
    pub width: i64,
    /// Photo height
    pub height: i64,
    /// Optional. File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl PhotoSize {
    pub fn new(file_id: String, file_unique_id: String, width: i64, height: i64) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            file_size: None,
        }
    }
}

/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Animation {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: i64,
    /// Video height as defined by sender
    pub height: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Optional. Animation thumbnail as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    /// Optional. Original animation filename as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl Animation {
    pub fn new(
        file_id: String,
        file_unique_id: String,
        width: i64,
        height: i64,
        duration: i64,
    ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            duration,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,
    /// Optional. Performer of the audio as defined by sender or by audio tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Optional. Title of the audio as defined by sender or by audio tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Original filename as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// Optional. Thumbnail of the album cover to which the music file belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}
impl Audio {
    pub fn new(file_id: String, file_unique_id: String, duration: i64) -> Self {
        Self {
            file_id,
            file_unique_id,
            duration,
            performer: None,
            title: None,
            file_name: None,
            mime_type: None,
            file_size: None,
            thumb: None,
        }
    }
}

/// This object represents a general file (as opposed to photos, voice messages and audio files).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Document {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Optional. Document thumbnail as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    /// Optional. Original filename as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl Document {
    pub fn new(file_id: String, file_unique_id: String) -> Self {
        Self {
            file_id,
            file_unique_id,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}

/// This object represents a video file.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width as defined by sender
    pub width: i64,
    /// Video height as defined by sender
    pub height: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Optional. Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    /// Optional. Original filename as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl Video {
    pub fn new(
        file_id: String,
        file_unique_id: String,
        width: i64,
        height: i64,
        duration: i64,
    ) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            duration,
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}

/// This object represents a video message (available in Telegram apps as of v.4.0).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width and height (diameter of the video message) as defined by sender
    pub length: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Optional. Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    /// Optional. File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl VideoNote {
    pub fn new(file_id: String, file_unique_id: String, length: i64, duration: i64) -> Self {
        Self {
            file_id,
            file_unique_id,
            length,
            duration,
            thumb: None,
            file_size: None,
        }
    }
}

/// This object represents a voice note.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Voice {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,
    /// Optional. MIME type of the file as defined by sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl Voice {
    pub fn new(file_id: String, file_unique_id: String, duration: i64) -> Self {
        Self {
            file_id,
            file_unique_id,
            duration,
            mime_type: None,
            file_size: None,
        }
    }
}

/// This object represents a phone contact.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Optional. Additional data about the contact in the form of a vCard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}
impl Contact {
    pub fn new(phone_number: String, first_name: String) -> Self {
        Self {
            phone_number,
            first_name,
            last_name: None,
            user_id: None,
            vcard: None,
        }
    }
}

/// This object represents an animated emoji that displays a random value.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Value of the dice, 1-6 for “🎲”, “🎯” and “🎳” base emoji, 1-5 for “🏀” and “⚽” base emoji, 1-64 for “🎰” base emoji
    pub value: i64,
}
impl Dice {
    pub fn new(emoji: String, value: i64) -> Self {
        Self { emoji, value }
    }
}

/// This object contains information about one answer option in a poll.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: String,
    /// Number of users that voted for this option
    pub voter_count: i64,
}
impl PollOption {
    pub fn new(text: String, voter_count: i64) -> Self {
        Self { text, voter_count }
    }
}

/// This object represents an answer of a user in a non-anonymous poll.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// The user, who changed the answer to the poll
    pub user: User,
    /// 0-based identifiers of answer options, chosen by the user. May be empty if the user retracted their vote.
    pub option_ids: Vec<i64>,
}
impl PollAnswer {
    pub fn new(poll_id: String, user: User, option_ids: Vec<i64>) -> Self {
        Self {
            poll_id,
            user,
            option_ids,
        }
    }
}

/// This object contains information about a poll.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-300 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: i64,
    /// True, if the poll is closed
    pub is_closed: bool,
    /// True, if the poll is anonymous
    pub is_anonymous: bool,
    /// Poll type, currently can be “regular” or “quiz”
    #[serde(rename = "type")]
    pub type_name: String,
    /// True, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// Optional. 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    /// Optional. Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Optional. Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// Optional. Amount of time in seconds the poll will be active after creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    /// Optional. Point in time (Unix timestamp) when the poll will be automatically closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
}
impl Poll {
    pub fn new(
        id: String,
        question: String,
        options: Vec<PollOption>,
        total_voter_count: i64,
        is_closed: bool,
        is_anonymous: bool,
        type_name: String,
        allows_multiple_answers: bool,
    ) -> Self {
        Self {
            id,
            question,
            options,
            total_voter_count,
            is_closed,
            is_anonymous,
            type_name,
            allows_multiple_answers,
            correct_option_id: None,
            explanation: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
        }
    }
}

/// This object represents a point on the map.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Optional. Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// Optional. The direction in which user is moving, in degrees; 1-360. For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// Optional. The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}
impl Location {
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            longitude,
            latitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }
}

/// This object represents a venue.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Venue {
    /// Venue location. Can't be a live location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}
impl Venue {
    pub fn new(location: Location, title: String, address: String) -> Self {
        Self {
            location,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }
}

/// Describes data sent from a Web App to the bot.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WebAppData {
    /// The data. Be aware that a bad client can send arbitrary data in this field.
    pub data: String,
    /// Text of the web_app keyboard button from which the Web App was opened. Be aware that a bad client can send arbitrary data in this field.
    pub button_text: String,
}
impl WebAppData {
    pub fn new(data: String, button_text: String) -> Self {
        Self { data, button_text }
    }
}

/// This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,
    /// User that set the alert
    pub watcher: User,
    /// The distance between the users
    pub distance: i64,
}
impl ProximityAlertTriggered {
    pub fn new(traveler: User, watcher: User, distance: i64) -> Self {
        Self {
            traveler,
            watcher,
            distance,
        }
    }
}

/// This object represents a service message about a change in auto-delete timer settings.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat; in seconds
    pub message_auto_delete_time: i64,
}
impl MessageAutoDeleteTimerChanged {
    pub fn new(message_auto_delete_time: i64) -> Self {
        Self {
            message_auto_delete_time,
        }
    }
}

/// This object represents a service message about a video chat scheduled in the chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    pub start_date: i64,
}
impl VideoChatScheduled {
    pub fn new(start_date: i64) -> Self {
        Self { start_date }
    }
}

/// This object represents a service message about a video chat started in the chat. Currently holds no information.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoChatStarted {}
impl VideoChatStarted {
    pub fn new() -> Self {
        Self {}
    }
}

/// This object represents a service message about a video chat ended in the chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoChatEnded {
    /// Video chat duration in seconds
    pub duration: i64,
}
impl VideoChatEnded {
    pub fn new(duration: i64) -> Self {
        Self { duration }
    }
}

/// This object represents a service message about new members invited to a video chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Vec<User>,
}
impl VideoChatParticipantsInvited {
    pub fn new(users: Vec<User>) -> Self {
        Self { users }
    }
}

/// This object represent a user's profile pictures.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}
impl UserProfilePhotos {
    pub fn new(total_count: i64, photos: Vec<Vec<PhotoSize>>) -> Self {
        Self {
            total_count,
            photos,
        }
    }
}

/// This object represents a file ready to be downloaded. The file can be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// Optional. File path. Use https://api.telegram.org/file/bot<token>/<file_path> to get the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}
impl File {
    pub fn new(file_id: String, file_unique_id: String) -> Self {
        Self {
            file_id,
            file_unique_id,
            file_size: None,
            file_path: None,
        }
    }
}

/// Describes a Web App.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened with additional data as specified in Initializing Web Apps
    pub url: String,
}
impl WebAppInfo {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

/// This object represents a custom keyboard with reply options (see Introduction to bots for details and examples).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of KeyboardButton objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Optional. Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to false, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,
    /// Optional. Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    /// Optional. The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Optional. Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}
impl ReplyKeyboardMarkup {
    pub fn new(keyboard: Vec<Vec<KeyboardButton>>) -> Self {
        Self {
            keyboard,
            resize_keyboard: None,
            one_time_keyboard: None,
            input_field_placeholder: None,
            selective: None,
        }
    }
}

/// This object represents one button of the reply keyboard. For simple text buttons String can be used instead of this object to specify text of the button. Optional fields web_app, request_contact, request_location, and request_poll are mutually exclusive.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed
    pub text: String,
    /// Optional. If True, the user's phone number will be sent as a contact when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    /// Optional. If True, the user's current location will be sent when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    /// Optional. If specified, the user will be asked to create a poll and send it to the bot when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,
    /// Optional. If specified, the described Web App will be launched when the button is pressed. The Web App will be able to send a “web_app_data” service message. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}
impl KeyboardButton {
    pub fn new(text: String) -> Self {
        Self {
            text,
            request_contact: None,
            request_location: None,
            request_poll: None,
            web_app: None,
        }
    }
}

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct KeyboardButtonPollType {
    /// Optional. If quiz is passed, the user will be allowed to create only polls in the quiz mode. If regular is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_name: Option<String>,
}
impl KeyboardButtonPollType {
    pub fn new() -> Self {
        Self { type_name: None }
    }
}

/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see ReplyKeyboardMarkup).
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use one_time_keyboard in ReplyKeyboardMarkup)
    pub remove_keyboard: bool,
    /// Optional. Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}
impl ReplyKeyboardRemove {
    pub fn new(remove_keyboard: bool) -> Self {
        Self {
            remove_keyboard,
            selective: None,
        }
    }
}

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
impl InlineKeyboardMarkup {
    pub fn new(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self { inline_keyboard }
    }
}

/// This object represents one button of an inline keyboard. You must use exactly one of the optional fields.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineKeyboardButton {
    /// Label text on the button
    pub text: String,
    /// Optional. HTTP or tg:// URL to be opened when the button is pressed. Links tg://user?id=<user_id> can be used to mention a user by their ID without using a username, if this is allowed by their privacy settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
    /// Optional. Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery. Available only in private chats between a user and the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
    /// Optional. An HTTPS URL used to automatically authorize the user. Can be used as a replacement for the Telegram Login Widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,
    /// Optional. If set, pressing the button will prompt the user to select one of their chats, open that chat and insert the bot's username and the specified inline query in the input field. May be empty, in which case just the bot's username will be inserted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,
    /// Optional. If set, pressing the button will insert the bot's username and the specified inline query in the current chat's input field. May be empty, in which case only the bot's username will be inserted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,
    /// Optional. Description of the game that will be launched when the user presses the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,
    /// Optional. Specify True, to send a Pay button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}
impl InlineKeyboardButton {
    pub fn new(text: String) -> Self {
        Self {
            text,
            url: None,
            callback_data: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }
}

/// This object represents a parameter of the inline keyboard button used to automatically authorize a user. Serves as a great replacement for the Telegram Login Widget when the user is coming from Telegram. All the user needs to do is tap/click a button and confirm that they want to log in:
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginUrl {
    /// An HTTPS URL to be opened with user authorization data added to the query string when the button is pressed. If the user refuses to provide authorization data, the original URL without information about the user will be opened. The data added is the same as described in Receiving authorization data.
    pub url: String,
    /// Optional. New text of the button in forwarded messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    /// Optional. Username of a bot, which will be used for user authorization. See Setting up a bot for more details. If not specified, the current bot's username will be assumed. The url's domain must be the same as the domain linked with the bot. See Linking your domain to the bot for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    /// Optional. Pass True to request the permission for your bot to send messages to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}
impl LoginUrl {
    pub fn new(url: String) -> Self {
        Self {
            url,
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }
}

/// This object represents an incoming callback query from a callback button in an inline keyboard. If the button that originated the query was attached to a message sent by the bot, the field message will be present. If the button was attached to a message sent via the bot (in inline mode), the field inline_message_id will be present. Exactly one of the fields data or game_short_name will be present.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Optional. Message with the callback button that originated the query. Note that message content and message date will not be available if the message is too old
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// Optional. Identifier of the message sent via the bot in inline mode, that originated the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent. Useful for high scores in games.
    pub chat_instance: String,
    /// Optional. Data associated with the callback button. Be aware that the message originated the query can contain no callback buttons with this data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Optional. Short name of a Game to be returned, serves as the unique identifier for the game
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}
impl CallbackQuery {
    pub fn new(id: String, from: User, chat_instance: String) -> Self {
        Self {
            id,
            from,
            message: None,
            inline_message_id: None,
            chat_instance,
            data: None,
            game_short_name: None,
        }
    }
}

/// Upon receiving a message with this object, Telegram clients will display a reply interface to the user (act as if the user has selected the bot's message and tapped 'Reply'). This can be extremely useful if you want to create user-friendly step-by-step interfaces without having to sacrifice privacy mode.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ForceReply {
    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    pub force_reply: bool,
    /// Optional. The placeholder to be shown in the input field when the reply is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// Optional. Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}
impl ForceReply {
    pub fn new(force_reply: bool) -> Self {
        Self {
            force_reply,
            input_field_placeholder: None,
            selective: None,
        }
    }
}

/// This object represents a chat photo.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub small_file_id: String,
    /// Unique file identifier of small (160x160) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub small_file_unique_id: String,
    /// File identifier of big (640x640) chat photo. This file_id can be used only for photo download and only for as long as the photo is not changed.
    pub big_file_id: String,
    /// Unique file identifier of big (640x640) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub big_file_unique_id: String,
}
impl ChatPhoto {
    pub fn new(
        small_file_id: String,
        small_file_unique_id: String,
        big_file_id: String,
        big_file_unique_id: String,
    ) -> Self {
        Self {
            small_file_id,
            small_file_unique_id,
            big_file_id,
            big_file_unique_id,
        }
    }
}

/// Represents an invite link for a chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator, then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// True, if users joining the chat via the link need to be approved by chat administrators
    pub creates_join_request: bool,
    /// True, if the link is primary
    pub is_primary: bool,
    /// True, if the link is revoked
    pub is_revoked: bool,
    /// Optional. Invite link name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. Point in time (Unix timestamp) when the link will expire or has been expired
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// Optional. The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// Optional. Number of pending join requests created using this link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<i64>,
}
impl ChatInviteLink {
    pub fn new(
        invite_link: String,
        creator: User,
        creates_join_request: bool,
        is_primary: bool,
        is_revoked: bool,
    ) -> Self {
        Self {
            invite_link,
            creator,
            creates_join_request,
            is_primary,
            is_revoked,
            name: None,
            expire_date: None,
            member_limit: None,
            pending_join_request_count: None,
        }
    }
}

/// Represents the rights of an administrator in a chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatAdministratorRights {
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// True, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// Optional. True, if the administrator can post in the channel; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Optional. True, if the administrator can edit messages of other users and can pin messages; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Optional. True, if the user is allowed to pin messages; groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}
impl ChatAdministratorRights {
    pub fn new(
        is_anonymous: bool,
        can_manage_chat: bool,
        can_delete_messages: bool,
        can_manage_video_chats: bool,
        can_restrict_members: bool,
        can_promote_members: bool,
        can_change_info: bool,
        can_invite_users: bool,
    ) -> Self {
        Self {
            is_anonymous,
            can_manage_chat,
            can_delete_messages,
            can_manage_video_chats,
            can_restrict_members,
            can_promote_members,
            can_change_info,
            can_invite_users,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
        }
    }
}

/// Represents a chat member that owns the chat and has all administrator privileges.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMemberOwner {
    /// Information about the user
    pub user: User,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// Optional. Custom title for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}
impl ChatMemberOwner {
    pub fn new(user: User, is_anonymous: bool) -> Self {
        Self {
            user,
            is_anonymous,
            custom_title: None,
        }
    }
}

/// Represents a chat member that has some additional privileges.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMemberAdministrator {
    /// Information about the user
    pub user: User,
    /// True, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: bool,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
    /// True, if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    pub can_manage_chat: bool,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: bool,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: bool,
    /// True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: bool,
    /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// Optional. True, if the administrator can post in the channel; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Optional. True, if the administrator can edit messages of other users and can pin messages; channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Optional. True, if the user is allowed to pin messages; groups and supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// Optional. Custom title for this user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}
impl ChatMemberAdministrator {
    pub fn new(
        user: User,
        can_be_edited: bool,
        is_anonymous: bool,
        can_manage_chat: bool,
        can_delete_messages: bool,
        can_manage_video_chats: bool,
        can_restrict_members: bool,
        can_promote_members: bool,
        can_change_info: bool,
        can_invite_users: bool,
    ) -> Self {
        Self {
            user,
            can_be_edited,
            is_anonymous,
            can_manage_chat,
            can_delete_messages,
            can_manage_video_chats,
            can_restrict_members,
            can_promote_members,
            can_change_info,
            can_invite_users,
            can_post_messages: None,
            can_edit_messages: None,
            can_pin_messages: None,
            custom_title: None,
        }
    }
}

/// Represents a chat member that has no additional privileges or restrictions.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMemberMember {
    /// Information about the user
    pub user: User,
}
impl ChatMemberMember {
    pub fn new(user: User) -> Self {
        Self { user }
    }
}

/// Represents a chat member that is under certain restrictions in the chat. Supergroups only.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMemberRestricted {
    /// Information about the user
    pub user: User,
    /// True, if the user is a member of the chat at the moment of the request
    pub is_member: bool,
    /// True, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// True, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// True, if the user is allowed to pin messages
    pub can_pin_messages: bool,
    /// True, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: bool,
    /// True, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes
    pub can_send_media_messages: bool,
    /// True, if the user is allowed to send polls
    pub can_send_polls: bool,
    /// True, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: bool,
    /// True, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: bool,
    /// Date when restrictions will be lifted for this user; unix time. If 0, then the user is restricted forever
    pub until_date: i64,
}
impl ChatMemberRestricted {
    pub fn new(
        user: User,
        is_member: bool,
        can_change_info: bool,
        can_invite_users: bool,
        can_pin_messages: bool,
        can_send_messages: bool,
        can_send_media_messages: bool,
        can_send_polls: bool,
        can_send_other_messages: bool,
        can_add_web_page_previews: bool,
        until_date: i64,
    ) -> Self {
        Self {
            user,
            is_member,
            can_change_info,
            can_invite_users,
            can_pin_messages,
            can_send_messages,
            can_send_media_messages,
            can_send_polls,
            can_send_other_messages,
            can_add_web_page_previews,
            until_date,
        }
    }
}

/// Represents a chat member that isn't currently a member of the chat, but may join it themselves.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMemberLeft {
    /// Information about the user
    pub user: User,
}
impl ChatMemberLeft {
    pub fn new(user: User) -> Self {
        Self { user }
    }
}

/// Represents a chat member that was banned in the chat and can't return to the chat or view chat messages.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMemberBanned {
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; unix time. If 0, then the user is banned forever
    pub until_date: i64,
}
impl ChatMemberBanned {
    pub fn new(user: User, until_date: i64) -> Self {
        Self { user, until_date }
    }
}

/// This object represents changes in the status of a chat member.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,
    /// Performer of the action, which resulted in the change
    pub from: User,
    /// Date the change was done in Unix time
    pub date: i64,
    /// Previous information about the chat member
    pub old_chat_member: ChatMember,
    /// New information about the chat member
    pub new_chat_member: ChatMember,
    /// Optional. Chat invite link, which was used by the user to join the chat; for joining by invite link events only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}
impl ChatMemberUpdated {
    pub fn new(
        chat: Chat,
        from: User,
        date: i64,
        old_chat_member: ChatMember,
        new_chat_member: ChatMember,
    ) -> Self {
        Self {
            chat,
            from,
            date,
            old_chat_member,
            new_chat_member,
            invite_link: None,
        }
    }
}

/// Represents a join request sent to a chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Date the request was sent in Unix time
    pub date: i64,
    /// Optional. Bio of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Optional. Chat invite link that was used by the user to send the join request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
}
impl ChatJoinRequest {
    pub fn new(chat: Chat, from: User, date: i64) -> Self {
        Self {
            chat,
            from,
            date,
            bio: None,
            invite_link: None,
        }
    }
}

/// Describes actions that a non-administrator user is allowed to take in a chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatPermissions {
    /// Optional. True, if the user is allowed to send text messages, contacts, locations and venues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    /// Optional. True, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,
    /// Optional. True, if the user is allowed to send polls, implies can_send_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    /// Optional. True, if the user is allowed to send animations, games, stickers and use inline bots, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    /// Optional. True, if the user is allowed to add web page previews to their messages, implies can_send_media_messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    /// Optional. True, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Optional. True, if the user is allowed to invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Optional. True, if the user is allowed to pin messages. Ignored in public supergroups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}
impl ChatPermissions {
    pub fn new() -> Self {
        Self {
            can_send_messages: None,
            can_send_media_messages: None,
            can_send_polls: None,
            can_send_other_messages: None,
            can_add_web_page_previews: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
        }
    }
}

/// Represents a location to which a chat is connected.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatLocation {
    /// The location to which the supergroup is connected. Can't be a live location.
    pub location: Location,
    /// Location address; 1-64 characters, as defined by the chat owner
    pub address: String,
}
impl ChatLocation {
    pub fn new(location: Location, address: String) -> Self {
        Self { location, address }
    }
}

/// This object represents a bot command.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BotCommand {
    /// Text of the command; 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    pub command: String,
    /// Description of the command; 1-256 characters.
    pub description: String,
}
impl BotCommand {
    pub fn new(command: String, description: String) -> Self {
        Self {
            command,
            description,
        }
    }
}

/// Represents the default scope of bot commands. Default commands are used if no commands with a narrower scope are specified for the user.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BotCommandScopeDefault {}
impl BotCommandScopeDefault {
    pub fn new() -> Self {
        Self {}
    }
}

/// Represents the scope of bot commands, covering all private chats.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BotCommandScopeAllPrivateChats {}
impl BotCommandScopeAllPrivateChats {
    pub fn new() -> Self {
        Self {}
    }
}

/// Represents the scope of bot commands, covering all group and supergroup chats.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BotCommandScopeAllGroupChats {}
impl BotCommandScopeAllGroupChats {
    pub fn new() -> Self {
        Self {}
    }
}

/// Represents the scope of bot commands, covering all group and supergroup chat administrators.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BotCommandScopeAllChatAdministrators {}
impl BotCommandScopeAllChatAdministrators {
    pub fn new() -> Self {
        Self {}
    }
}

/// Represents the scope of bot commands, covering a specific chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BotCommandScopeChat {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatId,
}
impl BotCommandScopeChat {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }
}

/// Represents the scope of bot commands, covering all administrators of a specific group or supergroup chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BotCommandScopeChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatId,
}
impl BotCommandScopeChatAdministrators {
    pub fn new(chat_id: ChatId) -> Self {
        Self { chat_id }
    }
}

/// Represents the scope of bot commands, covering a specific member of a group or supergroup chat.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BotCommandScopeChatMember {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}
impl BotCommandScopeChatMember {
    pub fn new(chat_id: ChatId, user_id: i64) -> Self {
        Self { chat_id, user_id }
    }
}

/// Represents a menu button, which opens the bot's list of commands.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MenuButtonCommands {}
impl MenuButtonCommands {
    pub fn new() -> Self {
        Self {}
    }
}

/// Represents a menu button, which launches a Web App.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MenuButtonWebApp {
    /// Text on the button
    pub text: String,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery.
    pub web_app: WebAppInfo,
}
impl MenuButtonWebApp {
    pub fn new(text: String, web_app: WebAppInfo) -> Self {
        Self { text, web_app }
    }
}

/// Describes that no specific value for the menu button was set.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MenuButtonDefault {}
impl MenuButtonDefault {
    pub fn new() -> Self {
        Self {}
    }
}

/// Describes why a request was unsuccessful.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseParameters {
    /// Optional. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    /// Optional. In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<i64>,
}
impl ResponseParameters {
    pub fn new() -> Self {
        Self {
            migrate_to_chat_id: None,
            retry_after: None,
        }
    }
}

/// Represents a photo to be sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputMediaPhoto {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
}
impl InputMediaPhoto {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            caption: None,
            parse_mode: None,
            caption_entities: None,
        }
    }
}

/// Represents a video to be sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputMediaVideo {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Optional. Pass True if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}
impl InputMediaVideo {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }
}

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputMediaAnimation {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Optional. Caption of the animation to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the animation caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Optional. Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Optional. Animation duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}
impl InputMediaAnimation {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
        }
    }
}

/// Represents an audio file to be treated as music to be sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputMediaAudio {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Optional. Caption of the audio to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the audio caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Optional. Performer of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Optional. Title of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl InputMediaAudio {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
        }
    }
}

/// Represents a general file to be sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputMediaDocument {
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass “attach://<file_attach_name>” to upload a new one using multipart/form-data under <file_attach_name> name. More information on Sending Files »
    pub media: InputFile,
    /// Optional. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Optional. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always True, if the document is sent as part of an album.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}
impl InputMediaDocument {
    pub fn new(media: InputFile) -> Self {
        Self {
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
        }
    }
}

/// This object represents a sticker.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Type of the sticker, currently one of “regular”, “mask”, “custom_emoji”. The type of the sticker is independent from its format, which is determined by the fields is_animated and is_video.
    #[serde(rename = "type")]
    pub type_name: String,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// True, if the sticker is animated
    pub is_animated: bool,
    /// True, if the sticker is a video sticker
    pub is_video: bool,
    /// Optional. Sticker thumbnail in the .WEBP or .JPG format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
    /// Optional. Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Optional. Name of the sticker set to which the sticker belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    /// Optional. For premium regular stickers, premium animation for the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<File>,
    /// Optional. For mask stickers, the position where the mask should be placed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// Optional. For custom emoji stickers, unique identifier of the custom emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    /// Optional. File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl Sticker {
    pub fn new(
        file_id: String,
        file_unique_id: String,
        type_name: String,
        width: i64,
        height: i64,
        is_animated: bool,
        is_video: bool,
    ) -> Self {
        Self {
            file_id,
            file_unique_id,
            type_name,
            width,
            height,
            is_animated,
            is_video,
            thumb: None,
            emoji: None,
            set_name: None,
            premium_animation: None,
            mask_position: None,
            custom_emoji_id: None,
            file_size: None,
        }
    }
}

/// This object represents a sticker set.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// Type of stickers in the set, currently one of “regular”, “mask”, “custom_emoji”
    pub sticker_type: String,
    /// True, if the sticker set contains animated stickers
    pub is_animated: bool,
    /// True, if the sticker set contains video stickers
    pub is_video: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Optional. Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}
impl StickerSet {
    pub fn new(
        name: String,
        title: String,
        sticker_type: String,
        is_animated: bool,
        is_video: bool,
        stickers: Vec<Sticker>,
    ) -> Self {
        Self {
            name,
            title,
            sticker_type,
            is_animated,
            is_video,
            stickers,
            thumb: None,
        }
    }
}

/// This object describes the position on faces where a mask should be placed by default.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of “forehead”, “eyes”, “mouth”, or “chin”.
    pub point: String,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f64,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f64,
}
impl MaskPosition {
    pub fn new(point: String, x_shift: f64, y_shift: f64, scale: f64) -> Self {
        Self {
            point,
            x_shift,
            y_shift,
            scale,
        }
    }
}

/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: String,
    /// Sender
    pub from: User,
    /// Text of the query (up to 256 characters)
    pub query: String,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: String,
    /// Optional. Type of the chat from which the inline query was sent. Can be either “sender” for a private chat with the inline query sender, “private”, “group”, “supergroup”, or “channel”. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    /// Optional. Sender location, only for bots that request user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
impl InlineQuery {
    pub fn new(id: String, from: User, query: String, offset: String) -> Self {
        Self {
            id,
            from,
            query,
            offset,
            chat_type: None,
            location: None,
        }
    }
}

/// Represents a link to an article or web page.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title of the result
    pub title: String,
    /// Content of the message to be sent
    pub input_message_content: InputMessageContent,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. URL of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Optional. Pass True if you don't want the URL to be shown in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_url: Option<bool>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}
impl InlineQueryResultArticle {
    pub fn new(id: String, title: String, input_message_content: InputMessageContent) -> Self {
        Self {
            id,
            title,
            input_message_content,
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultPhoto {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in JPEG format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumb_url: String,
    /// Optional. Width of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Optional. Height of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultPhoto {
    pub fn new(id: String, photo_url: String, thumb_url: String) -> Self {
        Self {
            id,
            photo_url,
            thumb_url,
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultGif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// Optional. Width of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i64>,
    /// Optional. Height of the GIF
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i64>,
    /// Optional. Duration of the GIF in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<i64>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumb_url: String,
    /// Optional. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_mime_type: Option<String>,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the GIF animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultGif {
    pub fn new(id: String, gif_url: String, thumb_url: String) -> Self {
        Self {
            id,
            gif_url,
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumb_url,
            thumb_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the MPEG4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i64>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumb_url: String,
    /// Optional. MIME type of the thumbnail, must be one of “image/jpeg”, “image/gif”, or “video/mp4”. Defaults to “image/jpeg”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_mime_type: Option<String>,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultMpeg4Gif {
    pub fn new(id: String, mpeg4_url: String, thumb_url: String) -> Self {
        Self {
            id,
            mpeg4_url,
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumb_url,
            thumb_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// MIME type of the content of the video URL, “text/html” or “video/mp4”
    pub mime_type: String,
    /// URL of the thumbnail (JPEG only) for the video
    pub thumb_url: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i64>,
    /// Optional. Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i64>,
    /// Optional. Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i64>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video. This field is required if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultVideo {
    pub fn new(
        id: String,
        video_url: String,
        mime_type: String,
        thumb_url: String,
        title: String,
    ) -> Self {
        Self {
            id,
            video_url,
            mime_type,
            thumb_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultAudio {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title
    pub title: String,
    /// Optional. Caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the audio caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Optional. Audio duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultAudio {
    pub fn new(id: String, audio_url: String, title: String) -> Self {
        Self {
            id,
            audio_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            performer: None,
            audio_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the the voice message.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultVoice {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid URL for the voice recording
    pub voice_url: String,
    /// Recording title
    pub title: String,
    /// Optional. Caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Recording duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the voice recording
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultVoice {
    pub fn new(id: String, voice_url: String, title: String) -> Self {
        Self {
            id,
            voice_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            voice_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultDocument {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// Optional. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A valid URL for the file
    pub document_url: String,
    /// MIME type of the content of the file, either “application/pdf” or “application/zip”
    pub mime_type: String,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. URL of the thumbnail (JPEG only) for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}
impl InlineQueryResultDocument {
    pub fn new(id: String, title: String, document_url: String, mime_type: String) -> Self {
        Self {
            id,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            document_url,
            mime_type,
            description: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the location.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultLocation {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Location title
    pub title: String,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// Optional. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// Optional. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}
impl InlineQueryResultLocation {
    pub fn new(id: String, latitude: f64, longitude: f64, title: String) -> Self {
        Self {
            id,
            latitude,
            longitude,
            title,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the venue.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultVenue {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    pub longitude: f64,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}
impl InlineQueryResultVenue {
    pub fn new(id: String, latitude: f64, longitude: f64, title: String, address: String) -> Self {
        Self {
            id,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the contact.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultContact {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
    /// Optional. Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,
    /// Optional. Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i64>,
    /// Optional. Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i64>,
}
impl InlineQueryResultContact {
    pub fn new(id: String, phone_number: String, first_name: String) -> Self {
        Self {
            id,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

/// Represents a Game.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultGame {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Short name of the game
    pub game_short_name: String,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
impl InlineQueryResultGame {
    pub fn new(id: String, game_short_name: String) -> Self {
        Self {
            id,
            game_short_name,
            reply_markup: None,
        }
    }
}

/// Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the photo.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultCachedPhoto {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the photo
    pub photo_file_id: String,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Caption of the photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultCachedPhoto {
    pub fn new(id: String, photo_file_id: String) -> Self {
        Self {
            id,
            photo_file_id,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to an animated GIF file stored on the Telegram servers. By default, this animated GIF file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with specified content instead of the animation.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultCachedGif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the GIF file
    pub gif_file_id: String,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the GIF animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultCachedGif {
    pub fn new(id: String, gif_file_id: String) -> Self {
        Self {
            id,
            gif_file_id,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the animation.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the MPEG4 file
    pub mpeg4_file_id: String,
    /// Optional. Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Optional. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultCachedMpeg4Gif {
    pub fn new(id: String, mpeg4_file_id: String) -> Self {
        Self {
            id,
            mpeg4_file_id,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the sticker.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultCachedSticker {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier of the sticker
    pub sticker_file_id: String,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the sticker,
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultCachedSticker {
    pub fn new(id: String, sticker_file_id: String) -> Self {
        Self {
            id,
            sticker_file_id,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the file.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultCachedDocument {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// A valid file identifier for the file
    pub document_file_id: String,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultCachedDocument {
    pub fn new(id: String, title: String, document_file_id: String) -> Self {
        Self {
            id,
            title,
            document_file_id,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use input_message_content to send a message with the specified content instead of the video.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultCachedVideo {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the video file
    pub video_file_id: String,
    /// Title for the result
    pub title: String,
    /// Optional. Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultCachedVideo {
    pub fn new(id: String, video_file_id: String, title: String) -> Self {
        Self {
            id,
            video_file_id,
            title,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the voice message.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultCachedVoice {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the voice message
    pub voice_file_id: String,
    /// Voice message title
    pub title: String,
    /// Optional. Caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the voice message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultCachedVoice {
    pub fn new(id: String, voice_file_id: String, title: String) -> Self {
        Self {
            id,
            voice_file_id,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use input_message_content to send a message with the specified content instead of the audio.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InlineQueryResultCachedAudio {
    /// Unique identifier for this result, 1-64 bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub audio_file_id: String,
    /// Optional. Caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional. Mode for parsing entities in the audio caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Optional. Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Optional. Content of the message to be sent instead of the audio
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
impl InlineQueryResultCachedAudio {
    pub fn new(id: String, audio_file_id: String) -> Self {
        Self {
            id,
            audio_file_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

/// Represents the content of a text message to be sent as the result of an inline query.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputTextMessageContent {
    /// Text of the message to be sent, 1-4096 characters
    pub message_text: String,
    /// Optional. Mode for parsing entities in the message text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in message text, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Disables link previews for links in the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
}
impl InputTextMessageContent {
    pub fn new(message_text: String) -> Self {
        Self {
            message_text,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
        }
    }
}

/// Represents the content of a location message to be sent as the result of an inline query.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f64,
    /// Longitude of the location in degrees
    pub longitude: f64,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Optional. Period in seconds for which the location can be updated, should be between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// Optional. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// Optional. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
}
impl InputLocationMessageContent {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }
}

/// Represents the content of a venue message to be sent as the result of an inline query.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputVenueMessageContent {
    /// Latitude of the venue in degrees
    pub latitude: f64,
    /// Longitude of the venue in degrees
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Optional. Foursquare identifier of the venue, if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Optional. Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Optional. Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Optional. Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}
impl InputVenueMessageContent {
    pub fn new(latitude: f64, longitude: f64, title: String, address: String) -> Self {
        Self {
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }
}

/// Represents the content of a contact message to be sent as the result of an inline query.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Optional. Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional. Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}
impl InputContactMessageContent {
    pub fn new(phone_number: String, first_name: String) -> Self {
        Self {
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
        }
    }
}

/// Represents the content of an invoice message to be sent as the result of an inline query.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputInvoiceMessageContent {
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Payment provider token, obtained via @BotFather
    pub provider_token: String,
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// Optional. The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// Optional. A JSON-serialized array of suggested amounts of tip in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// Optional. A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// Optional. URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Optional. Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Optional. Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Optional. Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Optional. Pass True if you require the user's full name to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Optional. Pass True if you require the user's phone number to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Optional. Pass True if you require the user's email address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Optional. Pass True if you require the user's shipping address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Optional. Pass True if the user's phone number should be sent to provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Optional. Pass True if the user's email address should be sent to provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Optional. Pass True if the final price depends on the shipping method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}
impl InputInvoiceMessageContent {
    pub fn new(
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }
}

/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Optional. Sender location, only for bots that require user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Optional. Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message. Will be also received in callback queries and can be used to edit the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}
impl ChosenInlineResult {
    pub fn new(result_id: String, from: User, query: String) -> Self {
        Self {
            result_id,
            from,
            location: None,
            inline_message_id: None,
            query,
        }
    }
}

/// Describes an inline message sent by a Web App on behalf of a user.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SentWebAppMessage {
    /// Optional. Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
impl SentWebAppMessage {
    pub fn new() -> Self {
        Self {
            inline_message_id: None,
        }
    }
}

/// This object represents a portion of the price for goods or services.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LabeledPrice {
    /// Portion label
    pub label: String,
    /// Price of the product in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub amount: i64,
}
impl LabeledPrice {
    pub fn new(label: String, amount: i64) -> Self {
        Self { label, amount }
    }
}

/// This object contains basic information about an invoice.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Invoice {
    /// Product name
    pub title: String,
    /// Product description
    pub description: String,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: String,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
}
impl Invoice {
    pub fn new(
        title: String,
        description: String,
        start_parameter: String,
        currency: String,
        total_amount: i64,
    ) -> Self {
        Self {
            title,
            description,
            start_parameter,
            currency,
            total_amount,
        }
    }
}

/// This object represents a shipping address.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShippingAddress {
    /// Two-letter ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line for the address
    pub street_line1: String,
    /// Second line for the address
    pub street_line2: String,
    /// Address post code
    pub post_code: String,
}
impl ShippingAddress {
    pub fn new(
        country_code: String,
        state: String,
        city: String,
        street_line1: String,
        street_line2: String,
        post_code: String,
    ) -> Self {
        Self {
            country_code,
            state,
            city,
            street_line1,
            street_line2,
            post_code,
        }
    }
}

/// This object represents information about an order.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OrderInfo {
    /// Optional. User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional. User's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Optional. User email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Optional. User shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}
impl OrderInfo {
    pub fn new() -> Self {
        Self {
            name: None,
            phone_number: None,
            email: None,
            shipping_address: None,
        }
    }
}

/// This object represents one shipping option.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}
impl ShippingOption {
    pub fn new(id: String, title: String, prices: Vec<LabeledPrice>) -> Self {
        Self { id, title, prices }
    }
}

/// This object contains basic information about a successful payment.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Optional. Identifier of the shipping option chosen by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    /// Optional. Order information provided by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}
impl SuccessfulPayment {
    pub fn new(
        currency: String,
        total_amount: i64,
        invoice_payload: String,
        telegram_payment_charge_id: String,
        provider_payment_charge_id: String,
    ) -> Self {
        Self {
            currency,
            total_amount,
            invoice_payload,
            shipping_option_id: None,
            order_info: None,
            telegram_payment_charge_id,
            provider_payment_charge_id,
        }
    }
}

/// This object contains information about an incoming shipping query.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}
impl ShippingQuery {
    pub fn new(
        id: String,
        from: User,
        invoice_payload: String,
        shipping_address: ShippingAddress,
    ) -> Self {
        Self {
            id,
            from,
            invoice_payload,
            shipping_address,
        }
    }
}

/// This object contains information about an incoming pre-checkout query.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Optional. Identifier of the shipping option chosen by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    /// Optional. Order information provided by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
}
impl PreCheckoutQuery {
    pub fn new(
        id: String,
        from: User,
        currency: String,
        total_amount: i64,
        invoice_payload: String,
    ) -> Self {
        Self {
            id,
            from,
            currency,
            total_amount,
            invoice_payload,
            shipping_option_id: None,
            order_info: None,
        }
    }
}

/// Describes Telegram Passport data shared with the bot by the user.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}
impl PassportData {
    pub fn new(data: Vec<EncryptedPassportElement>, credentials: EncryptedCredentials) -> Self {
        Self { data, credentials }
    }
}

/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportFile {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// File size in bytes
    pub file_size: i64,
    /// Unix time when the file was uploaded
    pub file_date: i64,
}
impl PassportFile {
    pub fn new(file_id: String, file_unique_id: String, file_size: i64, file_date: i64) -> Self {
        Self {
            file_id,
            file_unique_id,
            file_size,
            file_date,
        }
    }
}

/// Describes documents or other Telegram Passport elements shared with the bot by the user.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EncryptedPassportElement {
    /// Element type. One of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”, “phone_number”, “email”.
    #[serde(rename = "type")]
    pub type_name: String,
    /// Optional. Base64-encoded encrypted Telegram Passport element data provided by the user, available for “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport” and “address” types. Can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Optional. User's verified phone number, available only for “phone_number” type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Optional. User's verified email address, available only for “email” type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Optional. Array of encrypted files with documents provided by the user, available for “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration” and “temporary_registration” types. Files can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    /// Optional. Encrypted file with the front side of the document, provided by the user. Available for “passport”, “driver_license”, “identity_card” and “internal_passport”. The file can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,
    /// Optional. Encrypted file with the reverse side of the document, provided by the user. Available for “driver_license” and “identity_card”. The file can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,
    /// Optional. Encrypted file with the selfie of the user holding a document, provided by the user; available for “passport”, “driver_license”, “identity_card” and “internal_passport”. The file can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,
    /// Optional. Array of encrypted files with translated versions of documents provided by the user. Available if requested for “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration” and “temporary_registration” types. Files can be decrypted and verified using the accompanying EncryptedCredentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
    /// Base64-encoded element hash for using in PassportElementErrorUnspecified
    pub hash: String,
}
impl EncryptedPassportElement {
    pub fn new(type_name: String, hash: String) -> Self {
        Self {
            type_name,
            data: None,
            phone_number: None,
            email: None,
            files: None,
            front_side: None,
            reverse_side: None,
            selfie: None,
            translation: None,
            hash,
        }
    }
}

/// Describes data required for decrypting and authenticating EncryptedPassportElement. See the Telegram Passport Documentation for a complete description of the data decryption and authentication processes.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for EncryptedPassportElement decryption and authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    pub secret: String,
}
impl EncryptedCredentials {
    pub fn new(data: String, hash: String, secret: String) -> Self {
        Self { data, hash, secret }
    }
}

/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportElementErrorDataField {
    /// Error source, must be data
    pub source: String,
    /// The section of the user's Telegram Passport which has the error, one of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”
    #[serde(rename = "type")]
    pub type_name: String,
    /// Name of the data field which has the error
    pub field_name: String,
    /// Base64-encoded data hash
    pub data_hash: String,
    /// Error message
    pub message: String,
}
impl PassportElementErrorDataField {
    pub fn new(
        source: String,
        type_name: String,
        field_name: String,
        data_hash: String,
        message: String,
    ) -> Self {
        Self {
            source,
            type_name,
            field_name,
            data_hash,
            message,
        }
    }
}

/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportElementErrorFrontSide {
    /// Error source, must be front_side
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    #[serde(rename = "type")]
    pub type_name: String,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}
impl PassportElementErrorFrontSide {
    pub fn new(source: String, type_name: String, file_hash: String, message: String) -> Self {
        Self {
            source,
            type_name,
            file_hash,
            message,
        }
    }
}

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be reverse_side
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “driver_license”, “identity_card”
    #[serde(rename = "type")]
    pub type_name: String,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}
impl PassportElementErrorReverseSide {
    pub fn new(source: String, type_name: String, file_hash: String, message: String) -> Self {
        Self {
            source,
            type_name,
            file_hash,
            message,
        }
    }
}

/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportElementErrorSelfie {
    /// Error source, must be selfie
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”
    #[serde(rename = "type")]
    pub type_name: String,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    /// Error message
    pub message: String,
}
impl PassportElementErrorSelfie {
    pub fn new(source: String, type_name: String, file_hash: String, message: String) -> Self {
        Self {
            source,
            type_name,
            file_hash,
            message,
        }
    }
}

/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportElementErrorFile {
    /// Error source, must be file
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub type_name: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}
impl PassportElementErrorFile {
    pub fn new(source: String, type_name: String, file_hash: String, message: String) -> Self {
        Self {
            source,
            type_name,
            file_hash,
            message,
        }
    }
}

/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportElementErrorFiles {
    /// Error source, must be files
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub type_name: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}
impl PassportElementErrorFiles {
    pub fn new(
        source: String,
        type_name: String,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self {
            source,
            type_name,
            file_hashes,
            message,
        }
    }
}

/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportElementErrorTranslationFile {
    /// Error source, must be translation_file
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub type_name: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}
impl PassportElementErrorTranslationFile {
    pub fn new(source: String, type_name: String, file_hash: String, message: String) -> Self {
        Self {
            source,
            type_name,
            file_hash,
            message,
        }
    }
}

/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportElementErrorTranslationFiles {
    /// Error source, must be translation_files
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver_license”, “identity_card”, “internal_passport”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”
    #[serde(rename = "type")]
    pub type_name: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}
impl PassportElementErrorTranslationFiles {
    pub fn new(
        source: String,
        type_name: String,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self {
            source,
            type_name,
            file_hashes,
            message,
        }
    }
}

/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PassportElementErrorUnspecified {
    /// Error source, must be unspecified
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub type_name: String,
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String,
}
impl PassportElementErrorUnspecified {
    pub fn new(source: String, type_name: String, element_hash: String, message: String) -> Self {
        Self {
            source,
            type_name,
            element_hash,
            message,
        }
    }
}

/// This object represents a game. Use BotFather to create and edit games, their short names will act as unique identifiers.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    /// Title of the game
    pub title: String,
    /// Description of the game
    pub description: String,
    /// Photo that will be displayed in the game message in chats.
    pub photo: Vec<PhotoSize>,
    /// Optional. Brief description of the game or high scores included in the game message. Can be automatically edited to include current high scores for the game when the bot calls setGameScore, or manually edited using editMessageText. 0-4096 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional. Special entities that appear in text, such as usernames, URLs, bot commands, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    /// Optional. Animation that will be displayed in the game message in chats. Upload via BotFather
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}
impl Game {
    pub fn new(title: String, description: String, photo: Vec<PhotoSize>) -> Self {
        Self {
            title,
            description,
            photo,
            text: None,
            text_entities: None,
            animation: None,
        }
    }
}

/// A placeholder, currently holds no information. Use BotFather to set up your game.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CallbackGame {}
impl CallbackGame {
    pub fn new() -> Self {
        Self {}
    }
}

/// This object represents one row of the high scores table for a game.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i64,
    /// User
    pub user: User,
    /// Score
    pub score: i64,
}
impl GameHighScore {
    pub fn new(position: i64, user: User, score: i64) -> Self {
        Self {
            position,
            user,
            score,
        }
    }
}

/// Params represents a set of parameters that gets passed to a request.
pub type Params = HashMap<String, Value>;

/// Unique identifier for the target chat or username of the target channel
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ChatId {
    /// Unique identifier
    IntType(i64),
    /// username
    StringType(String),
}

/// This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum InputFile {
    /// FileID is an ID of a file already uploaded to Telegram.
    FileID(String),
    /// FileURL is a URL to use as a file for a request.
    FileURL(String),
    /// fileAttach is an internal file type used for processed media groups.
    FileAttach(String),
    /// FileBytes contains information about a set of bytes to upload as a File.
    FileBytes(String, Vec<u8>),
    /// FilePath is a path to a local file.
    FilePath(String),
}

/// On success,returns a InputFileResult object data method
pub enum InputFileResult {
    /// don't need upload
    Text(String),
    /// must upload using multipart/form-data
    Part(reqwest::multipart::Part),
}

impl InputFile {
    pub fn need_upload(&self) -> bool {
        matches!(self, InputFile::FileBytes(_, _) | InputFile::FilePath(_))
    }

    pub async fn data(&self) -> Result<InputFileResult, Box<dyn std::error::Error>> {
        match self {
            InputFile::FileID(id) => Ok(InputFileResult::Text(id.clone())),
            InputFile::FileURL(url) => Ok(InputFileResult::Text(url.clone())),
            InputFile::FileAttach(attach) => Ok(InputFileResult::Text(attach.clone())),
            InputFile::FileBytes(file_name, bytes) => Ok(InputFileResult::Part(
                reqwest::multipart::Part::bytes(bytes.clone()).file_name(file_name.to_string()),
            )),
            InputFile::FilePath(path) => Ok(InputFileResult::Part(
                reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(
                    tokio_util::codec::FramedRead::new(
                        tokio::fs::File::open(path).await?,
                        tokio_util::codec::BytesCodec::new(),
                    ),
                ))
                .file_name(path.to_string()),
            )),
        }
    }
}

/// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}
/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
/// ```
/// ChatMemberOwner
/// ChatMemberAdministrator
/// ChatMemberMember
/// ChatMemberRestricted
/// ChatMemberLeft
/// ChatMemberBanned
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "status")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    ChatMemberOwner(ChatMemberOwner),
    #[serde(rename = "administrator")]
    ChatMemberAdministrator(ChatMemberAdministrator),
    #[serde(rename = "member")]
    ChatMemberMember(ChatMemberMember),
    #[serde(rename = "restricted")]
    ChatMemberRestricted(ChatMemberRestricted),
    #[serde(rename = "left")]
    ChatMemberLeft(ChatMemberLeft),
    #[serde(rename = "kicked")]
    ChatMemberBanned(ChatMemberBanned),
}

/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
/// ```
/// BotCommandScopeDefault
/// BotCommandScopeAllPrivateChats
/// BotCommandScopeAllGroupChats
/// BotCommandScopeAllChatAdministrators
/// BotCommandScopeChat
/// BotCommandScopeChatAdministrators
/// BotCommandScopeChatMember
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum BotCommandScope {
    #[serde(rename = "default")]
    BotCommandScopeDefault(BotCommandScopeDefault),
    #[serde(rename = "all_private_chats")]
    BotCommandScopeAllPrivateChats(BotCommandScopeAllPrivateChats),
    #[serde(rename = "all_group_chats")]
    BotCommandScopeAllGroupChats(BotCommandScopeAllGroupChats),
    #[serde(rename = "all_chat_administrators")]
    BotCommandScopeAllChatAdministrators(BotCommandScopeAllChatAdministrators),
    #[serde(rename = "chat")]
    BotCommandScopeChat(BotCommandScopeChat),
    #[serde(rename = "chat_administrators")]
    BotCommandScopeChatAdministrators(BotCommandScopeChatAdministrators),
    #[serde(rename = "chat_member")]
    BotCommandScopeChatMember(BotCommandScopeChatMember),
}

/// This object describes the bot's menu button in a private chat. It should be one of
/// ```
/// MenuButtonCommands
/// MenuButtonWebApp
/// MenuButtonDefault
/// ```
/// If a menu button other than MenuButtonDefault is set for a private chat, then it is applied in the chat. Otherwise the default menu button is applied. By default, the menu button opens the list of bot commands.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MenuButton {
    #[serde(rename = "commands")]
    MenuButtonCommands(MenuButtonCommands),
    #[serde(rename = "web_app")]
    MenuButtonWebApp(MenuButtonWebApp),
    #[serde(rename = "default")]
    MenuButtonDefault(MenuButtonDefault),
}

/// This object represents the content of a media message to be sent. It should be one of
/// ```
/// InputMediaAnimation
/// InputMediaDocument
/// InputMediaAudio
/// InputMediaPhoto
/// InputMediaVideo
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum InputMedia {
    #[serde(rename = "animation")]
    InputMediaAnimation(InputMediaAnimation),
    #[serde(rename = "document")]
    InputMediaDocument(InputMediaDocument),
    #[serde(rename = "audio")]
    InputMediaAudio(InputMediaAudio),
    #[serde(rename = "photo")]
    InputMediaPhoto(InputMediaPhoto),
    #[serde(rename = "video")]
    InputMediaVideo(InputMediaVideo),
}

impl InputMedia {
    /// prepare_input_media_param evaluates a single InputMedia and determines if it
    /// needs to be modified for a successful upload. If it returns nil, then the
    /// value does not need to be included in the params. Otherwise, it will return
    /// the same type as was originally provided.
    ///
    /// The idx is used to calculate the file field name. If you only have a single
    /// file, 0 may be used. It is formatted into "attach://file-%d" for the primary
    /// media and "attach://file-%d-thumb" for thumbnails.
    ///
    /// It is expected to be used in conjunction with prepareInputMediaFile.
    pub fn prepare_input_media_param(&self, idx: i32) -> Self {
        match self {
            InputMedia::InputMediaAnimation(animation) => {
                let mut media = animation.media.clone();
                if media.need_upload() {
                    media = Self::attach_file(idx);
                }
                let mut thumb: Option<InputFile> = None;
                if let Some(some_thumb) = &animation.thumb {
                    if some_thumb.need_upload() {
                        thumb = Some(Self::attach_thumb_file(idx));
                    }
                }
                Self::InputMediaAnimation(InputMediaAnimation {
                    media,
                    thumb,
                    ..animation.clone()
                })
            }
            InputMedia::InputMediaDocument(document) => {
                let mut media = document.media.clone();
                if media.need_upload() {
                    media = Self::attach_file(idx);
                }
                let mut thumb: Option<InputFile> = None;
                if let Some(some_thumb) = &document.thumb {
                    if some_thumb.need_upload() {
                        thumb = Some(Self::attach_thumb_file(idx));
                    }
                }
                Self::InputMediaDocument(InputMediaDocument {
                    media,
                    thumb,
                    ..document.clone()
                })
            }
            InputMedia::InputMediaAudio(audio) => {
                let mut media = audio.media.clone();
                if media.need_upload() {
                    media = Self::attach_file(idx);
                }
                let mut thumb: Option<InputFile> = None;
                if let Some(some_thumb) = &audio.thumb {
                    if some_thumb.need_upload() {
                        thumb = Some(Self::attach_thumb_file(idx));
                    }
                }
                Self::InputMediaAudio(InputMediaAudio {
                    media,
                    thumb,
                    ..audio.clone()
                })
            }
            InputMedia::InputMediaPhoto(photo) => {
                if !photo.media.need_upload() {
                    return self.clone();
                }
                Self::InputMediaPhoto(InputMediaPhoto {
                    media: Self::attach_file(idx),
                    ..photo.clone()
                })
            }
            InputMedia::InputMediaVideo(video) => {
                let mut media = video.media.clone();
                if media.need_upload() {
                    media = Self::attach_file(idx);
                }
                let mut thumb: Option<InputFile> = None;
                if let Some(some_thumb) = &video.thumb {
                    if some_thumb.need_upload() {
                        thumb = Some(Self::attach_thumb_file(idx));
                    }
                }
                Self::InputMediaVideo(InputMediaVideo {
                    media,
                    thumb,
                    ..video.clone()
                })
            }
        }
    }

    /// prepare_input_media_file generates an array of RequestFile to provide for
    /// Fileable's files method. It returns an array as a single InputMedia may have
    /// multiple files, for the primary media and a thumbnail.
    ///
    /// The idx parameter is used to generate file field names. It uses the names
    /// "file-%d" for the main file and "file-%d-thumb" for the thumbnail.
    ///
    /// It is expected to be used in conjunction with prepareInputMediaParam.
    pub fn prepare_input_media_file(&self, idx: i32) -> Vec<(String, InputFile)> {
        let mut result: Vec<(String, InputFile)> = Vec::new();
        match self {
            InputMedia::InputMediaAnimation(animation) => {
                let media = animation.media.clone();
                if media.need_upload() {
                    result.push((Self::attach_file_name(idx), media.clone()));
                }
                if let Some(thumb) = &animation.thumb {
                    if thumb.need_upload() {
                        result.push((Self::attach_thumb_file_name(idx), thumb.clone()));
                    }
                }
            }
            InputMedia::InputMediaDocument(document) => {
                let media = document.media.clone();
                if media.need_upload() {
                    result.push((Self::attach_file_name(idx), media.clone()));
                }
                if let Some(thumb) = &document.thumb {
                    if thumb.need_upload() {
                        result.push((Self::attach_thumb_file_name(idx), thumb.clone()));
                    }
                }
            }
            InputMedia::InputMediaAudio(audio) => {
                let media = audio.media.clone();
                if media.need_upload() {
                    result.push((Self::attach_file_name(idx), media.clone()));
                }
                if let Some(thumb) = &audio.thumb {
                    if thumb.need_upload() {
                        result.push((Self::attach_thumb_file_name(idx), thumb.clone()));
                    }
                }
            }
            InputMedia::InputMediaPhoto(photo) => {
                if photo.media.need_upload() {
                    result.push((Self::attach_file_name(idx), photo.media.clone()));
                }
            }
            InputMedia::InputMediaVideo(video) => {
                let media = video.media.clone();
                if media.need_upload() {
                    result.push((Self::attach_file_name(idx), media.clone()));
                }
                if let Some(thumb) = &video.thumb {
                    if thumb.need_upload() {
                        result.push((Self::attach_thumb_file_name(idx), thumb.clone()));
                    }
                }
            }
        }
        result
    }

    fn attach_file_name(idx: i32) -> String {
        format!("file-{}", idx)
    }

    fn attach_thumb_file_name(idx: i32) -> String {
        format!("file-{}-thumb", idx)
    }

    fn attach_file(idx: i32) -> InputFile {
        InputFile::FileAttach(format!("attach://file-{}", idx))
    }

    fn attach_thumb_file(idx: i32) -> InputFile {
        InputFile::FileAttach(format!("attach://file-{}-thumb", idx))
    }
}

/// method will return Message or True
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum MayBeMessage {
    Message(Message),
    Bool(bool),
}

impl Chat {
    pub fn new_private(id: i64) -> Self {
        Self::new(id, ChatType::Private)
    }
    pub fn new_group(id: i64) -> Self {
        Self::new(id, ChatType::Group)
    }
    pub fn new_super_group(id: i64) -> Self {
        Self::new(id, ChatType::Supergroup)
    }
    pub fn new_channel(id: i64) -> Self {
        Self::new(id, ChatType::Channel)
    }
    pub fn is_private(&self) -> bool {
        matches!(self.type_name, ChatType::Private)
    }
    pub fn is_group(&self) -> bool {
        matches!(self.type_name, ChatType::Group)
    }
    pub fn is_super_group(&self) -> bool {
        matches!(self.type_name, ChatType::Supergroup)
    }
    pub fn is_channel(&self) -> bool {
        matches!(self.type_name, ChatType::Channel)
    }
}

impl MessageEntity {
    pub fn new_mention(offset: i64, length: i64) -> Self {
        Self::new("mention".to_string(), offset, length)
    }
    pub fn new_hashtag(offset: i64, length: i64) -> Self {
        Self::new("hashtag".to_string(), offset, length)
    }
    pub fn new_cashtag(offset: i64, length: i64) -> Self {
        Self::new("cashtag".to_string(), offset, length)
    }
    pub fn new_bot_command(offset: i64, length: i64) -> Self {
        Self::new("bot_command".to_string(), offset, length)
    }
    pub fn new_url(offset: i64, length: i64) -> Self {
        Self::new("url".to_string(), offset, length)
    }
    pub fn new_email(offset: i64, length: i64) -> Self {
        Self::new("email".to_string(), offset, length)
    }
    pub fn new_phone_number(offset: i64, length: i64) -> Self {
        Self::new("phone_number".to_string(), offset, length)
    }
    pub fn new_bold(offset: i64, length: i64) -> Self {
        Self::new("bold".to_string(), offset, length)
    }
    pub fn new_italic(offset: i64, length: i64) -> Self {
        Self::new("italic".to_string(), offset, length)
    }
    pub fn new_underline(offset: i64, length: i64) -> Self {
        Self::new("underline".to_string(), offset, length)
    }
    pub fn new_strikethrough(offset: i64, length: i64) -> Self {
        Self::new("strikethrough".to_string(), offset, length)
    }
    pub fn new_code(offset: i64, length: i64) -> Self {
        Self::new("code".to_string(), offset, length)
    }
    pub fn new_pre(offset: i64, length: i64) -> Self {
        Self::new("pre".to_string(), offset, length)
    }
    pub fn new_text_link(offset: i64, length: i64) -> Self {
        Self::new("text_link".to_string(), offset, length)
    }
    pub fn new_text_mention(offset: i64, length: i64) -> Self {
        Self::new("text_mention".to_string(), offset, length)
    }
}

impl Sticker {
    pub fn new_regular(
        file_id: String,
        file_unique_id: String,
        width: i64,
        height: i64,
        is_animated: bool,
        is_video: bool,
    ) -> Self {
        Self::new(
            file_id,
            file_unique_id,
            "regular".to_string(),
            width,
            height,
            is_animated,
            is_video,
        )
    }
    pub fn new_mask(
        file_id: String,
        file_unique_id: String,
        width: i64,
        height: i64,
        is_animated: bool,
        is_video: bool,
    ) -> Self {
        Self::new(
            file_id,
            file_unique_id,
            "mask".to_string(),
            width,
            height,
            is_animated,
            is_video,
        )
    }
    pub fn new_custom_emoji(
        file_id: String,
        file_unique_id: String,
        width: i64,
        height: i64,
        is_animated: bool,
        is_video: bool,
    ) -> Self {
        Self::new(
            file_id,
            file_unique_id,
            "custom_emoji".to_string(),
            width,
            height,
            is_animated,
            is_video,
        )
    }
}

impl PassportElementErrorDataField {
    pub fn new_personal_details(
        source: String,
        field_name: String,
        data_hash: String,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "personal_details".to_string(),
            field_name,
            data_hash,
            message,
        )
    }
    pub fn new_passport(
        source: String,
        field_name: String,
        data_hash: String,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "passport".to_string(),
            field_name,
            data_hash,
            message,
        )
    }
    pub fn new_driver_license(
        source: String,
        field_name: String,
        data_hash: String,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "driver_license".to_string(),
            field_name,
            data_hash,
            message,
        )
    }
    pub fn new_identity_card(
        source: String,
        field_name: String,
        data_hash: String,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "identity_card".to_string(),
            field_name,
            data_hash,
            message,
        )
    }
    pub fn new_internal_passport(
        source: String,
        field_name: String,
        data_hash: String,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "internal_passport".to_string(),
            field_name,
            data_hash,
            message,
        )
    }
    pub fn new_address(
        source: String,
        field_name: String,
        data_hash: String,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "address".to_string(),
            field_name,
            data_hash,
            message,
        )
    }
}

impl PassportElementErrorFrontSide {
    pub fn new_passport(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "passport".to_string(), file_hash, message)
    }
    pub fn new_driver_license(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "driver_license".to_string(), file_hash, message)
    }
    pub fn new_identity_card(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "identity_card".to_string(), file_hash, message)
    }
    pub fn new_internal_passport(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "internal_passport".to_string(), file_hash, message)
    }
}

impl PassportElementErrorReverseSide {
    pub fn new_driver_license(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "driver_license".to_string(), file_hash, message)
    }
    pub fn new_identity_card(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "identity_card".to_string(), file_hash, message)
    }
}

impl PassportElementErrorSelfie {
    pub fn new_passport(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "passport".to_string(), file_hash, message)
    }
    pub fn new_driver_license(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "driver_license".to_string(), file_hash, message)
    }
    pub fn new_identity_card(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "identity_card".to_string(), file_hash, message)
    }
    pub fn new_internal_passport(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "internal_passport".to_string(), file_hash, message)
    }
}

impl PassportElementErrorFile {
    pub fn new_utility_bill(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "utility_bill".to_string(), file_hash, message)
    }
    pub fn new_bank_statement(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "bank_statement".to_string(), file_hash, message)
    }
    pub fn new_rental_agreement(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "rental_agreement".to_string(), file_hash, message)
    }
    pub fn new_passport_registration(source: String, file_hash: String, message: String) -> Self {
        Self::new(
            source,
            "passport_registration".to_string(),
            file_hash,
            message,
        )
    }
    pub fn new_temporary_registration(source: String, file_hash: String, message: String) -> Self {
        Self::new(
            source,
            "temporary_registration".to_string(),
            file_hash,
            message,
        )
    }
}

impl PassportElementErrorFiles {
    pub fn new_utility_bill(source: String, file_hashes: Vec<String>, message: String) -> Self {
        Self::new(source, "utility_bill".to_string(), file_hashes, message)
    }
    pub fn new_bank_statement(source: String, file_hashes: Vec<String>, message: String) -> Self {
        Self::new(source, "bank_statement".to_string(), file_hashes, message)
    }
    pub fn new_rental_agreement(source: String, file_hashes: Vec<String>, message: String) -> Self {
        Self::new(source, "rental_agreement".to_string(), file_hashes, message)
    }
    pub fn new_passport_registration(
        source: String,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "passport_registration".to_string(),
            file_hashes,
            message,
        )
    }
    pub fn new_temporary_registration(
        source: String,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "temporary_registration".to_string(),
            file_hashes,
            message,
        )
    }
}

impl PassportElementErrorTranslationFile {
    pub fn new_passport(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "passport".to_string(), file_hash, message)
    }
    pub fn new_driver_license(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "driver_license".to_string(), file_hash, message)
    }
    pub fn new_identity_card(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "identity_card".to_string(), file_hash, message)
    }
    pub fn new_internal_passport(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "internal_passport".to_string(), file_hash, message)
    }
    pub fn new_utility_bill(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "utility_bill".to_string(), file_hash, message)
    }
    pub fn new_bank_statement(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "bank_statement".to_string(), file_hash, message)
    }
    pub fn new_rental_agreement(source: String, file_hash: String, message: String) -> Self {
        Self::new(source, "rental_agreement".to_string(), file_hash, message)
    }
    pub fn new_passport_registration(source: String, file_hash: String, message: String) -> Self {
        Self::new(
            source,
            "passport_registration".to_string(),
            file_hash,
            message,
        )
    }
    pub fn new_temporary_registration(source: String, file_hash: String, message: String) -> Self {
        Self::new(
            source,
            "temporary_registration".to_string(),
            file_hash,
            message,
        )
    }
}

impl PassportElementErrorTranslationFiles {
    pub fn new_passport(source: String, file_hashes: Vec<String>, message: String) -> Self {
        Self::new(source, "passport".to_string(), file_hashes, message)
    }
    pub fn new_driver_license(source: String, file_hashes: Vec<String>, message: String) -> Self {
        Self::new(source, "driver_license".to_string(), file_hashes, message)
    }
    pub fn new_identity_card(source: String, file_hashes: Vec<String>, message: String) -> Self {
        Self::new(source, "identity_card".to_string(), file_hashes, message)
    }
    pub fn new_internal_passport(
        source: String,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "internal_passport".to_string(),
            file_hashes,
            message,
        )
    }
    pub fn new_utility_bill(source: String, file_hashes: Vec<String>, message: String) -> Self {
        Self::new(source, "utility_bill".to_string(), file_hashes, message)
    }
    pub fn new_bank_statement(source: String, file_hashes: Vec<String>, message: String) -> Self {
        Self::new(source, "bank_statement".to_string(), file_hashes, message)
    }
    pub fn new_rental_agreement(source: String, file_hashes: Vec<String>, message: String) -> Self {
        Self::new(source, "rental_agreement".to_string(), file_hashes, message)
    }
    pub fn new_passport_registration(
        source: String,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "passport_registration".to_string(),
            file_hashes,
            message,
        )
    }
    pub fn new_temporary_registration(
        source: String,
        file_hashes: Vec<String>,
        message: String,
    ) -> Self {
        Self::new(
            source,
            "temporary_registration".to_string(),
            file_hashes,
            message,
        )
    }
}

/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:
/// ```
/// InputTextMessageContent
/// InputLocationMessageContent
/// InputVenueMessageContent
/// InputContactMessageContent
/// InputInvoiceMessageContent
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
    InputLocationMessageContent(InputLocationMessageContent),
    InputVenueMessageContent(InputVenueMessageContent),
    InputContactMessageContent(InputContactMessageContent),
    InputInvoiceMessageContent(InputInvoiceMessageContent),
}

/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
/// ```
/// PassportElementErrorDataField
/// PassportElementErrorFrontSide
/// PassportElementErrorReverseSide
/// PassportElementErrorSelfie
/// PassportElementErrorFile
/// PassportElementErrorFiles
/// PassportElementErrorTranslationFile
/// PassportElementErrorTranslationFiles
/// PassportElementErrorUnspecified
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PassportElementError {
    PassportElementErrorDataField(PassportElementErrorDataField),
    PassportElementErrorFrontSide(PassportElementErrorFrontSide),
    PassportElementErrorReverseSide(PassportElementErrorReverseSide),
    PassportElementErrorSelfie(PassportElementErrorSelfie),
    PassportElementErrorFile(PassportElementErrorFile),
    PassportElementErrorFiles(PassportElementErrorFiles),
    PassportElementErrorTranslationFile(PassportElementErrorTranslationFile),
    PassportElementErrorTranslationFiles(PassportElementErrorTranslationFiles),
    PassportElementErrorUnspecified(PassportElementErrorUnspecified),
}

/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
/// ```
/// InlineQueryResultCachedAudio
/// InlineQueryResultCachedDocument
/// InlineQueryResultCachedGif
/// InlineQueryResultCachedMpeg4Gif
/// InlineQueryResultCachedPhoto
/// InlineQueryResultCachedSticker
/// InlineQueryResultCachedVideo
/// InlineQueryResultCachedVoice
/// InlineQueryResultArticle
/// InlineQueryResultAudio
/// InlineQueryResultContact
/// InlineQueryResultGame
/// InlineQueryResultDocument
/// InlineQueryResultGif
/// InlineQueryResultLocation
/// InlineQueryResultMpeg4Gif
/// InlineQueryResultPhoto
/// InlineQueryResultVenue
/// InlineQueryResultVideo
/// InlineQueryResultVoice
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    #[serde(rename = "audio")]
    InlineQueryResultCachedAudio(InlineQueryResultCachedAudio),
    #[serde(rename = "document")]
    InlineQueryResultCachedDocument(InlineQueryResultCachedDocument),
    #[serde(rename = "gif")]
    InlineQueryResultCachedGif(InlineQueryResultCachedGif),
    #[serde(rename = "mpeg4_gif")]
    InlineQueryResultCachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    #[serde(rename = "photo")]
    InlineQueryResultCachedPhoto(InlineQueryResultCachedPhoto),
    #[serde(rename = "sticker")]
    InlineQueryResultCachedSticker(InlineQueryResultCachedSticker),
    #[serde(rename = "video")]
    InlineQueryResultCachedVideo(InlineQueryResultCachedVideo),
    #[serde(rename = "voice")]
    InlineQueryResultCachedVoice(InlineQueryResultCachedVoice),
    #[serde(rename = "article")]
    InlineQueryResultArticle(InlineQueryResultArticle),
    #[serde(rename = "audio")]
    InlineQueryResultAudio(InlineQueryResultAudio),
    #[serde(rename = "contact")]
    InlineQueryResultContact(InlineQueryResultContact),
    #[serde(rename = "game")]
    InlineQueryResultGame(InlineQueryResultGame),
    #[serde(rename = "document")]
    InlineQueryResultDocument(InlineQueryResultDocument),
    #[serde(rename = "gif")]
    InlineQueryResultGif(InlineQueryResultGif),
    #[serde(rename = "location")]
    InlineQueryResultLocation(InlineQueryResultLocation),
    #[serde(rename = "mpeg4_gif")]
    InlineQueryResultMpeg4Gif(InlineQueryResultMpeg4Gif),
    #[serde(rename = "photo")]
    InlineQueryResultPhoto(InlineQueryResultPhoto),
    #[serde(rename = "venue")]
    InlineQueryResultVenue(InlineQueryResultVenue),
    #[serde(rename = "video")]
    InlineQueryResultVideo(InlineQueryResultVideo),
    #[serde(rename = "voice")]
    InlineQueryResultVoice(InlineQueryResultVoice),
}

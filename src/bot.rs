use reqwest::header::HeaderMap;
use reqwest::multipart;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::{collections::HashMap, fmt::Debug};

use crate::{methods, types};

/// APIResponse is a response from the Telegram API with the result
/// stored raw.
#[derive(Deserialize, Serialize, Debug)]
pub struct APIResponse {
    ok: bool,
    error_code: Option<i32>,
    result: Option<serde_json::Value>,
    description: Option<String>,
    parameters: Option<types::ResponseParameters>,
}

/// the APIResponseError is returned when send request failed.
pub type APIResponseError = Box<dyn std::error::Error>;
/// ReplyResult is returned when send a request
pub type ReplyResult<T> = Result<T, APIResponseError>;

impl APIResponse {
    fn parse(self) -> Result<Self, APIResponseError> {
        if self.ok {
            return Ok(self);
        }
        Err(Error::new_option(self.error_code, self.description, self.parameters).into())
    }
}

/// Error is an error containing extra information returned by the Telegram API.
#[derive(Deserialize, Serialize, Debug)]
pub struct Error {
    pub code: i32,
    pub message: String,
    pub parameters: Option<types::ResponseParameters>,
}

impl Error {
    pub fn new(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            parameters: None,
        }
    }

    pub fn new_option(
        code: Option<i32>,
        message: Option<String>,
        parameters: Option<types::ResponseParameters>,
    ) -> Self {
        Self {
            code: code.unwrap_or(400),
            message: message.unwrap_or("server inter error.".to_string()),
            parameters,
        }
    }

    pub fn not_found() -> Self {
        Self {
            code: 404,
            message: "not found".to_string(),
            parameters: None,
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

/// BotAPI allows you to interact with the Telegram Bot API.
#[derive(Debug)]
pub struct BotApi {
    url: String,
    token: String,
    client: reqwest::Client,
}

impl BotApi {
    /// NewBotAPI creates a new BotAPI instance.
    /// It requires a token, provided by @BotFather on Telegram.
    /// # Using a Custom Bot API Server
    /// ```rust
    /// new(String::from("token"),Some(String::from("http://127.0.0.1:8081/bot")));
    /// ```
    pub async fn new(token: String, url: Option<String>) -> ReplyResult<Self> {
        let result = Self {
            url: url.unwrap_or(String::from("https://api.telegram.org/bot")),
            token,
            client: reqwest::Client::builder().build().unwrap(),
        };
        match result.get_me().await {
            Ok(_) => Ok(result),
            Err(err) => Err(err),
        }
    }

    /// send request
    pub async fn send<T, R>(&self, request: T) -> ReplyResult<R>
    where
        T: methods::Methods,
        R: DeserializeOwned,
    {
        if let Some(result) = self.request(&request).await?.result {
            return Ok(serde_json::from_value(result)?);
        }
        Err(Error::not_found().into())
    }

    /// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
    pub async fn get_me(&self) -> ReplyResult<types::User> {
        Ok(self.send(methods::GetMe::new()).await?)
    }

    /// Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on success. Requires no parameters.
    pub async fn log_out(&self) -> ReplyResult<bool> {
        Ok(self.send(methods::LogOut::new()).await?)
    }

    /// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters.
    pub async fn close(&self) -> ReplyResult<bool> {
        Ok(self.send(methods::Close::new()).await?)
    }

    /// Use this method to send text messages. On success, the sent Message is returned.
    pub async fn send_message(&self, request: methods::SendMessage) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to forward messages of any kind. Service messages can't be forwarded. On success, the sent Message is returned.
    pub async fn forward_message(
        &self,
        request: methods::ForwardMessage,
    ) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to copy messages of any kind. Service messages and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessage, but the copied message doesn't have a link to the original message. Returns the MessageId of the sent message on success.
    pub async fn copy_message(
        &self,
        request: methods::CopyMessage,
    ) -> ReplyResult<types::MessageId> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send photos. On success, the sent Message is returned.
    pub async fn send_photo(&self, request: methods::SendPhoto) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
    pub async fn send_audio(&self, request: methods::SendAudio) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    pub async fn send_document(
        &self,
        request: methods::SendDocument,
    ) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
    pub async fn send_video(&self, request: methods::SendVideo) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    pub async fn send_animation(
        &self,
        request: methods::SendAnimation,
    ) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    pub async fn send_voice(&self, request: methods::SendVoice) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// As of v.4.0, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
    pub async fn send_video_note(
        &self,
        request: methods::SendVideoNote,
    ) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned.
    pub async fn send_media_group(
        &self,
        request: methods::SendMediaGroup,
    ) -> ReplyResult<Vec<types::Message>> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send point on the map. On success, the sent Message is returned.
    pub async fn send_location(
        &self,
        request: methods::SendLocation,
    ) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
    pub async fn edit_message_live_location(
        &self,
        request: methods::EditMessageLiveLocation,
    ) -> ReplyResult<types::MayBeMessage> {
        Ok(self.send(request).await?)
    }

    /// Use this method to stop updating a live location message before live_period expires. On success, if the message is not an inline message, the edited Message is returned, otherwise True is returned.
    pub async fn stop_message_live_location(
        &self,
        request: methods::StopMessageLiveLocation,
    ) -> ReplyResult<types::MayBeMessage> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send information about a venue. On success, the sent Message is returned.
    pub async fn send_venue(&self, request: methods::SendVenue) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send phone contacts. On success, the sent Message is returned.
    pub async fn send_contact(&self, request: methods::SendContact) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send a native poll. On success, the sent Message is returned.
    pub async fn send_poll(&self, request: methods::SendPoll) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned.
    pub async fn send_dice(&self, request: methods::SendDice) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
    pub async fn send_chat_action(&self, request: methods::SendChatAction) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
    pub async fn get_user_profile_photos(
        &self,
        request: methods::GetUserProfilePhotos,
    ) -> ReplyResult<types::UserProfilePhotos> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
    pub async fn get_file(&self, request: methods::GetFile) -> ReplyResult<types::File> {
        Ok(self.send(request).await?)
    }

    /// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    pub async fn ban_chat_member(&self, request: methods::BanChatMember) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to unban a previously banned user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be removed from the chat. If you don't want this, use the parameter only_if_banned. Returns True on success.
    pub async fn unban_chat_member(&self, request: methods::UnbanChatMember) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass True for all permissions to lift restrictions from a user. Returns True on success.
    pub async fn restrict_chat_member(
        &self,
        request: methods::RestrictChatMember,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass False for all boolean parameters to demote a user. Returns True on success.
    pub async fn promote_chat_member(
        &self,
        request: methods::PromoteChatMember,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success.
    pub async fn set_chat_administrator_custom_title(
        &self,
        request: methods::SetChatAdministratorCustomTitle,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns True on success.
    pub async fn ban_chat_sender_chat(
        &self,
        request: methods::BanChatSenderChat,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns True on success.
    pub async fn unban_chat_sender_chat(
        &self,
        request: methods::UnbanChatSenderChat,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members administrator rights. Returns True on success.
    pub async fn set_chat_permissions(
        &self,
        request: methods::SetChatPermissions,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as String on success.
    pub async fn export_chat_invite_link(
        &self,
        request: methods::ExportChatInviteLink,
    ) -> ReplyResult<String> {
        Ok(self.send(request).await?)
    }

    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as ChatInviteLink object.
    pub async fn create_chat_invite_link(
        &self,
        request: methods::CreateChatInviteLink,
    ) -> ReplyResult<types::ChatInviteLink> {
        Ok(self.send(request).await?)
    }

    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a ChatInviteLink object.
    pub async fn edit_chat_invite_link(
        &self,
        request: methods::EditChatInviteLink,
    ) -> ReplyResult<types::ChatInviteLink> {
        Ok(self.send(request).await?)
    }

    /// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as ChatInviteLink object.
    pub async fn revoke_chat_invite_link(
        &self,
        request: methods::RevokeChatInviteLink,
    ) -> ReplyResult<types::ChatInviteLink> {
        Ok(self.send(request).await?)
    }

    /// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
    pub async fn approve_chat_join_request(
        &self,
        request: methods::ApproveChatJoinRequest,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
    pub async fn decline_chat_join_request(
        &self,
        request: methods::DeclineChatJoinRequest,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    pub async fn set_chat_photo(&self, request: methods::SetChatPhoto) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    pub async fn delete_chat_photo(&self, request: methods::DeleteChatPhoto) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    pub async fn set_chat_title(&self, request: methods::SetChatTitle) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    pub async fn set_chat_description(
        &self,
        request: methods::SetChatDescription,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
    pub async fn pin_chat_message(&self, request: methods::PinChatMessage) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
    pub async fn unpin_chat_message(
        &self,
        request: methods::UnpinChatMessage,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
    pub async fn unpin_all_chat_messages(
        &self,
        request: methods::UnpinAllChatMessages,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
    pub async fn leave_chat(&self, request: methods::LeaveChat) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat object on success.
    pub async fn get_chat(&self, request: methods::GetChat) -> ReplyResult<types::Chat> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of ChatMember objects.
    pub async fn get_chat_administrators(
        &self,
        request: methods::GetChatAdministrators,
    ) -> ReplyResult<Vec<types::ChatMember>> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get the number of members in a chat. Returns Int on success.
    pub async fn get_chat_member_count(
        &self,
        request: methods::GetChatMemberCount,
    ) -> ReplyResult<i64> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get information about a member of a chat. Returns a ChatMember object on success.
    pub async fn get_chat_member(
        &self,
        request: methods::GetChatMember,
    ) -> ReplyResult<types::ChatMember> {
        Ok(self.send(request).await?)
    }

    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
    pub async fn set_chat_sticker_set(
        &self,
        request: methods::SetChatStickerSet,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
    pub async fn delete_chat_sticker_set(
        &self,
        request: methods::DeleteChatStickerSet,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned.
    pub async fn answer_callback_query(
        &self,
        request: methods::AnswerCallbackQuery,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to change the list of the bot's commands. See https://core.telegram.org/bots#commands for more details about bot commands. Returns True on success.
    pub async fn set_my_commands(&self, request: methods::SetMyCommands) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success.
    pub async fn delete_my_commands(
        &self,
        request: methods::DeleteMyCommands,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of BotCommand objects. If commands aren't set, an empty list is returned.
    pub async fn get_my_commands(
        &self,
        request: methods::GetMyCommands,
    ) -> ReplyResult<Vec<types::BotCommand>> {
        Ok(self.send(request).await?)
    }

    /// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success.
    pub async fn set_chat_menu_button(
        &self,
        request: methods::SetChatMenuButton,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns MenuButton on success.
    pub async fn get_chat_menu_button(
        &self,
        request: methods::GetChatMenuButton,
    ) -> ReplyResult<types::MenuButton> {
        Ok(self.send(request).await?)
    }

    /// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are are free to modify the list before adding the bot. Returns True on success.
    pub async fn set_my_default_administrator_rights(
        &self,
        request: methods::SetMyDefaultAdministratorRights,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get the current default administrator rights of the bot. Returns ChatAdministratorRights on success.
    pub async fn get_my_default_administrator_rights(
        &self,
        request: methods::GetMyDefaultAdministratorRights,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects.
    pub async fn get_updates(
        &self,
        request: methods::GetUpdates,
    ) -> ReplyResult<Vec<types::Update>> {
        Ok(self.send(request).await?)
    }

    /// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
    pub async fn set_webhook(&self, request: methods::SetWebhook) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
    pub async fn delete_webhook(&self, request: methods::DeleteWebhook) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
    pub async fn get_webhook_info(&self) -> ReplyResult<types::WebhookInfo> {
        Ok(self.send(methods::GetWebhookInfo::new()).await?)
    }

    /// Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the sent Message is returned.
    pub async fn send_sticker(&self, request: methods::SendSticker) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get a sticker set. On success, a StickerSet object is returned.
    pub async fn get_sticker_set(
        &self,
        request: methods::GetStickerSet,
    ) -> ReplyResult<types::StickerSet> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of Sticker objects.
    pub async fn get_custom_emoji_stickers(
        &self,
        request: methods::GetCustomEmojiStickers,
    ) -> ReplyResult<Vec<types::Sticker>> {
        Ok(self.send(request).await?)
    }

    /// Use this method to upload a .PNG file with a sticker for later use in createNewStickerSet and addStickerToSet methods (can be used multiple times). Returns the uploaded File on success.
    pub async fn upload_sticker_file(
        &self,
        request: methods::UploadStickerFile,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. You must use exactly one of the fields png_sticker, tgs_sticker, or webm_sticker. Returns True on success.
    pub async fn create_new_sticker_set(
        &self,
        request: methods::CreateNewStickerSet,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to add a new sticker to a set created by the bot. You must use exactly one of the fields png_sticker, tgs_sticker, or webm_sticker. Animated stickers can be added to animated sticker sets and only to them. Animated sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns True on success.
    pub async fn add_sticker_to_set(&self, request: methods::AddStickerToSet) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to move a sticker in a set created by the bot to a specific position. Returns True on success.
    pub async fn set_sticker_position_in_set(
        &self,
        request: methods::SetStickerPositionInSet,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to delete a sticker from a set created by the bot. Returns True on success.
    pub async fn delete_sticker_from_set(
        &self,
        request: methods::DeleteStickerFromSet,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to set the thumbnail of a sticker set. Animated thumbnails can be set for animated sticker sets only. Video thumbnails can be set only for video sticker sets only. Returns True on success.
    pub async fn set_sticker_set_thumb(
        &self,
        request: methods::SetStickerSetThumb,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send answers to an inline query. On success, True is returned. No more than 50 results per query are allowed.
    pub async fn answer_inline_query(
        &self,
        request: methods::AnswerInlineQuery,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to set the result of an interaction with a Web App and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a SentWebAppMessage object is returned.
    pub async fn answer_web_app_query(
        &self,
        request: methods::AnswerWebAppQuery,
    ) -> ReplyResult<types::SentWebAppMessage> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send invoices. On success, the sent Message is returned.
    pub async fn send_invoice(&self, request: methods::SendInvoice) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to create a link for an invoice. Returns the created invoice link as String on success.
    pub async fn create_invoice_link(
        &self,
        request: methods::CreateInvoiceLink,
    ) -> ReplyResult<String> {
        Ok(self.send(request).await?)
    }

    /// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
    pub async fn answer_shipping_query(
        &self,
        request: methods::AnswerShippingQuery,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
    pub async fn answer_pre_checkout_query(
        &self,
        request: methods::AnswerPreCheckoutQuery,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns True on success.
    pub async fn set_passport_data_errors(
        &self,
        request: methods::SetPassportDataErrors,
    ) -> ReplyResult<bool> {
        Ok(self.send(request).await?)
    }

    /// Use this method to send a game. On success, the sent Message is returned.
    pub async fn send_game(&self, request: methods::SendGame) -> ReplyResult<types::Message> {
        Ok(self.send(request).await?)
    }

    /// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the Message is returned, otherwise True is returned. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
    pub async fn set_game_score(
        &self,
        request: methods::SetGameScore,
    ) -> ReplyResult<types::MayBeMessage> {
        Ok(self.send(request).await?)
    }

    /// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of GameHighScore objects.
    pub async fn get_game_high_scores(
        &self,
        request: methods::GetGameHighScores,
    ) -> ReplyResult<Vec<types::GameHighScore>> {
        Ok(self.send(request).await?)
    }
}

impl BotApi {
    /// specific url
    fn method(&self, endpoint: String) -> String {
        format!("{}{}/{}", self.url, self.token, endpoint)
    }

    /// make_request makes a request to a specific endpoint with our token.
    async fn make_request(
        &self,
        endpoint: String,
        params: types::Params,
    ) -> ReplyResult<APIResponse> {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        Ok(self
            .client
            .post(self.method(String::from(endpoint)))
            .headers(headers)
            .json(&params)
            .send()
            .await?
            .json::<APIResponse>()
            .await?
            .parse()?)
    }

    /// upload_files makes a request to the API with files.
    async fn upload_files(
        &self,
        endpoint: String,
        params: types::Params,
        files: HashMap<String, types::InputFile>,
    ) -> ReplyResult<APIResponse> {
        let mut form = reqwest::multipart::Form::new();
        for (param_key, param_value) in params {
            form = form.part(
                param_key.to_string(),
                multipart::Part::text(param_value.to_string()),
            );
        }
        for (file_key, file_value) in files {
            match file_value.data().await? {
                types::InputFileResult::Text(text) => {
                    form = form.part(
                        file_key.to_string(),
                        multipart::Part::text(text.to_string()),
                    );
                }
                types::InputFileResult::Part(part) => {
                    form = form.part(file_key.to_string(), part);
                }
            }
        }
        Ok(self
            .client
            .post(self.method(String::from(endpoint)))
            .multipart(form)
            .send()
            .await?
            .json::<APIResponse>()
            .await?
            .parse()?)
    }

    /// request sends a func to Telegram, and returns the APIResponse.
    async fn request<T: methods::Methods>(&self, request: &T) -> ReplyResult<APIResponse> {
        let mut params = request.params()?;
        if || -> bool {
            for (_, file) in request.files() {
                if file.need_upload() {
                    return true;
                }
            }
            false
        }() {
            return Ok(self
                .upload_files(request.endpoint(), params, request.files())
                .await?);
        }
        for (key, file) in request.files() {
            match file.data().await? {
                types::InputFileResult::Text(text) => {
                    params.insert(key, serde_json::json!(text));
                }
                _ => {}
            }
        }
        Ok(self.make_request(request.endpoint(), params).await?)
    }
}

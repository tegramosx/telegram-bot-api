use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

use crate::types;

/// request param interface
pub trait Params {
    fn params(&self) -> Result<types::Params, Box<dyn std::error::Error>>;
}

/// available methods interface
pub trait Methods: Params {
    fn endpoint(&self) -> String;
    fn files(&self) -> HashMap<String, types::InputFile> {
        HashMap::new()
    }
}

/// impl params for any method
impl<T> Params for T
where
    T: Serialize,
{
    fn params(&self) -> Result<types::Params, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(serde_json::to_string(self)?.as_str()).unwrap())
    }
}

/// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetMe {}
impl GetMe {
    pub fn new() -> Self {
        Self {}
    }
}

impl Methods for GetMe {
    fn endpoint(&self) -> String {
        "getMe".to_string()
    }
}

/// Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Returns True on success. Requires no parameters.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LogOut {}
impl LogOut {
    pub fn new() -> Self {
        Self {}
    }
}

impl Methods for LogOut {
    fn endpoint(&self) -> String {
        "logOut".to_string()
    }
}

/// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will error 429 in the first 10 minutes after the bot is launched. Returns True on success. Requires no parameters.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Close {}
impl Close {
    pub fn new() -> Self {
        Self {}
    }
}

impl Methods for Close {
    fn endpoint(&self) -> String {
        "close".to_string()
    }
}

/// Use this method to send text messages. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Text of the message to be sent, 1-4096 characters after entities parsing
    pub text: String,
    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<types::MessageEntity>>,
    /// Disables link previews for links in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendMessage {
    pub fn new(chat_id: types::ChatId, text: String) -> Self {
        Self {
            chat_id,
            text,
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendMessage {
    fn endpoint(&self) -> String {
        "sendMessage".to_string()
    }
}

/// Use this method to forward messages of any kind. Service messages can't be forwarded. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    pub from_chat_id: types::ChatId,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the forwarded message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i64,
}
impl ForwardMessage {
    pub fn new(chat_id: types::ChatId, from_chat_id: types::ChatId, message_id: i64) -> Self {
        Self {
            chat_id,
            from_chat_id,
            disable_notification: None,
            protect_content: None,
            message_id,
        }
    }
}

impl Methods for ForwardMessage {
    fn endpoint(&self) -> String {
        "forwardMessage".to_string()
    }
}

/// Use this method to copy messages of any kind. Service messages and invoice messages can't be copied. A quiz poll can be copied only if the value of the field correct_option_id is known to the bot. The method is analogous to the method forwardMessage, but the copied message doesn't have a link to the original message. Returns the MessageId of the sent message on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CopyMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    pub from_chat_id: types::ChatId,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i64,
    /// New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the new caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the new caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<types::MessageEntity>>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl CopyMessage {
    pub fn new(chat_id: types::ChatId, from_chat_id: types::ChatId, message_id: i64) -> Self {
        Self {
            chat_id,
            from_chat_id,
            message_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for CopyMessage {
    fn endpoint(&self) -> String {
        "copyMessage".to_string()
    }
}

/// Use this method to send photos. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20. More information on Sending Files »
    #[serde(skip_serializing)]
    pub photo: types::InputFile,
    /// Photo caption (may also be used when resending photos by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<types::MessageEntity>>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendPhoto {
    pub fn new(chat_id: types::ChatId, photo: types::InputFile) -> Self {
        Self {
            chat_id,
            photo,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendPhoto {
    fn endpoint(&self) -> String {
        "sendPhoto".to_string()
    }
    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        result.insert("photo".to_string(), self.photo.clone());
        result
    }
}

/// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendAudio {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files »
    #[serde(skip_serializing)]
    pub audio: types::InputFile,
    /// Audio caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the audio caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<types::MessageEntity>>,
    /// Duration of the audio in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Performer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    /// Track name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<types::InputFile>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendAudio {
    pub fn new(chat_id: types::ChatId, audio: types::InputFile) -> Self {
        Self {
            chat_id,
            audio,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
            thumb: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendAudio {
    fn endpoint(&self) -> String {
        "sendAudio".to_string()
    }
    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        result.insert("audio".to_string(), self.audio.clone());
        if let Some(thumb) = &self.thumb {
            result.insert("thumb".to_string(), thumb.clone());
        }
        result
    }
}

/// Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendDocument {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files »
    #[serde(skip_serializing)]
    pub document: types::InputFile,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<types::InputFile>,
    /// Document caption (may also be used when resending documents by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<types::MessageEntity>>,
    /// Disables automatic server-side content type detection for files uploaded using multipart/form-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendDocument {
    pub fn new(chat_id: types::ChatId, document: types::InputFile) -> Self {
        Self {
            chat_id,
            document,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendDocument {
    fn endpoint(&self) -> String {
        "sendDocument".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        result.insert("document".to_string(), self.document.clone());
        if let Some(thumb) = &self.thumb {
            result.insert("thumb".to_string(), thumb.clone());
        }
        result
    }
}

/// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendVideo {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Video to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. More information on Sending Files »
    #[serde(skip_serializing)]
    pub video: types::InputFile,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<types::InputFile>,
    /// Video caption (may also be used when resending videos by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<types::MessageEntity>>,
    /// Pass True if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendVideo {
    pub fn new(chat_id: types::ChatId, video: types::InputFile) -> Self {
        Self {
            chat_id,
            video,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            supports_streaming: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendVideo {
    fn endpoint(&self) -> String {
        "sendVideo".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        result.insert("video".to_string(), self.video.clone());
        if let Some(thumb) = &self.thumb {
            result.insert("thumb".to_string(), thumb.clone());
        }
        result
    }
}

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendAnimation {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Animation to send. Pass a file_id as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. More information on Sending Files »
    #[serde(skip_serializing)]
    pub animation: types::InputFile,
    /// Duration of sent animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<types::InputFile>,
    /// Animation caption (may also be used when resending animation by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the animation caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<types::MessageEntity>>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendAnimation {
    pub fn new(chat_id: types::ChatId, animation: types::InputFile) -> Self {
        Self {
            chat_id,
            animation,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendAnimation {
    fn endpoint(&self) -> String {
        "sendAnimation".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        result.insert("animation".to_string(), self.animation.clone());
        if let Some(thumb) = &self.thumb {
            result.insert("thumb".to_string(), thumb.clone());
        }
        result
    }
}

/// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message. For this to work, your audio must be in an .OGG file encoded with OPUS (other formats may be sent as Audio or Document). On success, the sent Message is returned. Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendVoice {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Audio file to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files »
    #[serde(skip_serializing)]
    pub voice: types::InputFile,
    /// Voice message caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<types::MessageEntity>>,
    /// Duration of the voice message in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendVoice {
    pub fn new(chat_id: types::ChatId, voice: types::InputFile) -> Self {
        Self {
            chat_id,
            voice,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendVoice {
    fn endpoint(&self) -> String {
        "sendVoice".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        result.insert("voice".to_string(), self.voice.clone());
        result
    }
}

/// As of v.4.0, Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendVideoNote {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Video note to send. Pass a file_id as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using multipart/form-data. More information on Sending Files ». Sending video notes by a URL is currently unsupported
    #[serde(skip_serializing)]
    pub video_note: types::InputFile,
    /// Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Video width and height, i.e. diameter of the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More information on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<types::InputFile>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendVideoNote {
    pub fn new(chat_id: types::ChatId, video_note: types::InputFile) -> Self {
        Self {
            chat_id,
            video_note,
            duration: None,
            length: None,
            thumb: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendVideoNote {
    fn endpoint(&self) -> String {
        "sendVideoNote".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        result.insert("video_note".to_string(), self.video_note.clone());
        if let Some(thumb) = &self.thumb {
            result.insert("thumb".to_string(), thumb.clone());
        }
        result
    }
}

/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type. On success, an array of Messages that were sent is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendMediaGroup {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// A JSON-serialized array describing messages to be sent, must include 2-10 items
    #[serde(serialize_with = "serialize_input_media")]
    pub media: Vec<types::InputMedia>,
    /// Sends messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the messages are a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
}

/// SendMediaGroup serialize media field
fn serialize_input_media<S>(input_media: &[types::InputMedia], s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let mut seq = s.serialize_seq(Some(input_media.len()))?;
    let mut idx = 0;
    for elem in input_media {
        seq.serialize_element(&(elem.prepare_input_media_param(idx)))?;
        idx += 1;
    }
    seq.end()
}

impl SendMediaGroup {
    pub fn new(chat_id: types::ChatId, media: Vec<types::InputMedia>) -> Self {
        Self {
            chat_id,
            media,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
        }
    }
}

impl Methods for SendMediaGroup {
    fn endpoint(&self) -> String {
        "sendMediaGroup".to_string()
    }
    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        let mut idx = 0;
        for elem in self.media.clone() {
            for (name, file) in elem.prepare_input_media_file(idx) {
                result.insert(name, file);
            }
            idx += 1;
        }
        result
    }
}

/// Use this method to send point on the map. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds for which the location will be updated (see Live Locations, should be between 60 and 86400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendLocation {
    pub fn new(chat_id: types::ChatId, latitude: f64, longitude: f64) -> Self {
        Self {
            chat_id,
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendLocation {
    fn endpoint(&self) -> String {
        "sendLocation".to_string()
    }
}

/// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message is not an inline message, the edited Message is returned, otherwise True is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EditMessageLiveLocation {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<types::ChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// Latitude of new location
    pub latitude: f64,
    /// Longitude of new location
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i64>,
    /// The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i64>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}
impl EditMessageLiveLocation {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            latitude,
            longitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }
}

impl Methods for EditMessageLiveLocation {
    fn endpoint(&self) -> String {
        "editMessageLiveLocation".to_string()
    }
}

/// Use this method to stop updating a live location message before live_period expires. On success, if the message is not an inline message, the edited Message is returned, otherwise True is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct StopMessageLiveLocation {
    /// Required if inline_message_id is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<types::ChatId>,
    /// Required if inline_message_id is not specified. Identifier of the message with live location to stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}
impl StopMessageLiveLocation {
    pub fn new() -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }
}

impl Methods for StopMessageLiveLocation {
    fn endpoint(&self) -> String {
        "stopMessageLiveLocation".to_string()
    }
}

/// Use this method to send information about a venue. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendVenue {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Latitude of the venue
    pub latitude: f64,
    /// Longitude of the venue
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendVenue {
    pub fn new(
        chat_id: types::ChatId,
        latitude: f64,
        longitude: f64,
        title: String,
        address: String,
    ) -> Self {
        Self {
            chat_id,
            latitude,
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendVenue {
    fn endpoint(&self) -> String {
        "sendVenue".to_string()
    }
}

/// Use this method to send phone contacts. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendContact {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendContact {
    pub fn new(chat_id: types::ChatId, phone_number: String, first_name: String) -> Self {
        Self {
            chat_id,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendContact {
    fn endpoint(&self) -> String {
        "sendContact".to_string()
    }
}

/// Use this method to send a native poll. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendPoll {
    /// unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Poll question, 1-300 characters
    pub question: String,
    /// A JSON-serialized list of answer options, 2-10 strings 1-100 characters each
    pub options: Vec<String>,
    /// True, if the poll needs to be anonymous, defaults to True
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Poll type, “quiz” or “regular”, defaults to “regular”
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_name: Option<String>,
    /// True, if the poll allows multiple answers, ignored for polls in quiz mode, defaults to False
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    /// 0-based identifier of the correct answer option, required for polls in quiz mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i64>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters with at most 2 line feeds after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// Mode for parsing entities in the explanation. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the poll explanation, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<types::MessageEntity>>,
    /// Amount of time in seconds the poll will be active after creation, 5-600. Can't be used together with close_date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed. Must be at least 5 and no more than 600 seconds in the future. Can't be used together with open_period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    /// Pass True if the poll needs to be immediately closed. This can be useful for poll preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendPoll {
    pub fn new(chat_id: types::ChatId, question: String, options: Vec<String>) -> Self {
        Self {
            chat_id,
            question,
            options,
            is_anonymous: None,
            type_name: None,
            allows_multiple_answers: None,
            correct_option_id: None,
            explanation: None,
            explanation_parse_mode: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
            is_closed: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendPoll {
    fn endpoint(&self) -> String {
        "sendPoll".to_string()
    }
}

/// Use this method to send an animated emoji that will display a random value. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendDice {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Emoji on which the dice throw animation is based. Currently, must be one of “🎲”, “🎯”, “🏀”, “⚽”, “🎳”, or “🎰”. Dice can have values 1-6 for “🎲”, “🎯” and “🎳”, values 1-5 for “🏀” and “⚽”, and values 1-64 for “🎰”. Defaults to “🎲”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendDice {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self {
            chat_id,
            emoji: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendDice {
    fn endpoint(&self) -> String {
        "sendDice".to_string()
    }
}

/// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendChatAction {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, upload_photo for photos, record_video or upload_video for videos, record_voice or upload_voice for voice notes, upload_document for general files, choose_sticker for stickers, find_location for location data, record_video_note or upload_video_note for video notes.
    pub action: String,
}
impl SendChatAction {
    pub fn new(chat_id: types::ChatId, action: String) -> Self {
        Self { chat_id, action }
    }
}

impl Methods for SendChatAction {
    fn endpoint(&self) -> String {
        "sendChatAction".to_string()
    }
}

/// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}
impl GetUserProfilePhotos {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            offset: None,
            limit: None,
        }
    }
}

impl Methods for GetUserProfilePhotos {
    fn endpoint(&self) -> String {
        "getUserProfilePhotos".to_string()
    }
}

/// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>, where <file_path> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetFile {
    /// File identifier to get information about
    pub file_id: String,
}
impl GetFile {
    pub fn new(file_id: String) -> Self {
        Self { file_id }
    }
}

impl Methods for GetFile {
    fn endpoint(&self) -> String {
        "getFile".to_string()
    }
}

/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    /// Pass True to delete all messages from the chat for the user that is being removed. If False, the user will be able to see messages in the group that were sent before the user was removed. Always True for supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}
impl BanChatMember {
    pub fn new(chat_id: types::ChatId, user_id: i64) -> Self {
        Self {
            chat_id,
            user_id,
            until_date: None,
            revoke_messages: None,
        }
    }
}

impl Methods for BanChatMember {
    fn endpoint(&self) -> String {
        "banChatMember".to_string()
    }
}

/// Use this method to unban a previously banned user in a supergroup or channel. The user will not to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be removed from the chat. If you don't want this, use the parameter only_if_banned. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Do nothing if the user is not banned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}
impl UnbanChatMember {
    pub fn new(chat_id: types::ChatId, user_id: i64) -> Self {
        Self {
            chat_id,
            user_id,
            only_if_banned: None,
        }
    }
}

impl Methods for UnbanChatMember {
    fn endpoint(&self) -> String {
        "unbanChatMember".to_string()
    }
}

/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass True for all permissions to lift restrictions from a user. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RestrictChatMember {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: types::ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// A JSON-serialized object for new user permissions
    pub permissions: types::ChatPermissions,
    /// Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}
impl RestrictChatMember {
    pub fn new(chat_id: types::ChatId, user_id: i64, permissions: types::ChatPermissions) -> Self {
        Self {
            chat_id,
            user_id,
            permissions,
            until_date: None,
        }
    }
}

impl Methods for RestrictChatMember {
    fn endpoint(&self) -> String {
        "restrictChatMember".to_string()
    }
}

/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Pass False for all boolean parameters to demote a user. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PromoteChatMember {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Pass True if the administrator's presence in the chat is hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// Pass True if the administrator can access the chat event log, chat statistics, message statistics in channels, see channel members, see anonymous administrators in supergroups and ignore slow mode. Implied by any other administrator privilege
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// Pass True if the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// Pass True if the administrator can edit messages of other users and can pin messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// Pass True if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// Pass True if the administrator can manage video chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    /// Pass True if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// Pass True if the administrator can add new administrators with a subset of their own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// Pass True if the administrator can change chat title, photo and other settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// Pass True if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// Pass True if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}
impl PromoteChatMember {
    pub fn new(chat_id: types::ChatId, user_id: i64) -> Self {
        Self {
            chat_id,
            user_id,
            is_anonymous: None,
            can_manage_chat: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_manage_video_chats: None,
            can_restrict_members: None,
            can_promote_members: None,
            can_change_info: None,
            can_invite_users: None,
            can_pin_messages: None,
        }
    }
}

impl Methods for PromoteChatMember {
    fn endpoint(&self) -> String {
        "promoteChatMember".to_string()
    }
}

/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: types::ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}
impl SetChatAdministratorCustomTitle {
    pub fn new(chat_id: types::ChatId, user_id: i64, custom_title: String) -> Self {
        Self {
            chat_id,
            user_id,
            custom_title,
        }
    }
}

impl Methods for SetChatAdministratorCustomTitle {
    fn endpoint(&self) -> String {
        "setChatAdministratorCustomTitle".to_string()
    }
}

/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}
impl BanChatSenderChat {
    pub fn new(chat_id: types::ChatId, sender_chat_id: i64) -> Self {
        Self {
            chat_id,
            sender_chat_id,
        }
    }
}

impl Methods for BanChatSenderChat {
    fn endpoint(&self) -> String {
        "banChatSenderChat".to_string()
    }
}

/// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UnbanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}
impl UnbanChatSenderChat {
    pub fn new(chat_id: types::ChatId, sender_chat_id: i64) -> Self {
        Self {
            chat_id,
            sender_chat_id,
        }
    }
}

impl Methods for UnbanChatSenderChat {
    fn endpoint(&self) -> String {
        "unbanChatSenderChat".to_string()
    }
}

/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members administrator rights. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetChatPermissions {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: types::ChatId,
    /// A JSON-serialized object for new default chat permissions
    pub permissions: types::ChatPermissions,
}
impl SetChatPermissions {
    pub fn new(chat_id: types::ChatId, permissions: types::ChatPermissions) -> Self {
        Self {
            chat_id,
            permissions,
        }
    }
}

impl Methods for SetChatPermissions {
    fn endpoint(&self) -> String {
        "setChatPermissions".to_string()
    }
}

/// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as String on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
}
impl ExportChatInviteLink {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self { chat_id }
    }
}

impl Methods for ExportChatInviteLink {
    fn endpoint(&self) -> String {
        "exportChatInviteLink".to_string()
    }
}

/// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as ChatInviteLink object.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}
impl CreateChatInviteLink {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self {
            chat_id,
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }
}

impl Methods for CreateChatInviteLink {
    fn endpoint(&self) -> String {
        "createChatInviteLink".to_string()
    }
}

/// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a ChatInviteLink object.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EditChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// The invite link to edit
    pub invite_link: String,
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}
impl EditChatInviteLink {
    pub fn new(chat_id: types::ChatId, invite_link: String) -> Self {
        Self {
            chat_id,
            invite_link,
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }
}

impl Methods for EditChatInviteLink {
    fn endpoint(&self) -> String {
        "editChatInviteLink".to_string()
    }
}

/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as ChatInviteLink object.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RevokeChatInviteLink {
    /// Unique identifier of the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// The invite link to revoke
    pub invite_link: String,
}
impl RevokeChatInviteLink {
    pub fn new(chat_id: types::ChatId, invite_link: String) -> Self {
        Self {
            chat_id,
            invite_link,
        }
    }
}

impl Methods for RevokeChatInviteLink {
    fn endpoint(&self) -> String {
        "revokeChatInviteLink".to_string()
    }
}

/// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApproveChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}
impl ApproveChatJoinRequest {
    pub fn new(chat_id: types::ChatId, user_id: i64) -> Self {
        Self { chat_id, user_id }
    }
}

impl Methods for ApproveChatJoinRequest {
    fn endpoint(&self) -> String {
        "approveChatJoinRequest".to_string()
    }
}

/// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the can_invite_users administrator right. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeclineChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}
impl DeclineChatJoinRequest {
    pub fn new(chat_id: types::ChatId, user_id: i64) -> Self {
        Self { chat_id, user_id }
    }
}

impl Methods for DeclineChatJoinRequest {
    fn endpoint(&self) -> String {
        "declineChatJoinRequest".to_string()
    }
}

/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// New chat photo, uploaded using multipart/form-data
    #[serde(skip_serializing)]
    pub photo: types::InputFile,
}
impl SetChatPhoto {
    pub fn new(chat_id: types::ChatId, photo: types::InputFile) -> Self {
        Self { chat_id, photo }
    }
}

impl Methods for SetChatPhoto {
    fn endpoint(&self) -> String {
        "setChatPhoto".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        result.insert("photo".to_string(), self.photo.clone());
        result
    }
}

/// Use this method to delete a chat photo. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteChatPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
}
impl DeleteChatPhoto {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self { chat_id }
    }
}

impl Methods for DeleteChatPhoto {
    fn endpoint(&self) -> String {
        "deleteChatPhoto".to_string()
    }
}

/// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetChatTitle {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// New chat title, 1-255 characters
    pub title: String,
}
impl SetChatTitle {
    pub fn new(chat_id: types::ChatId, title: String) -> Self {
        Self { chat_id, title }
    }
}

impl Methods for SetChatTitle {
    fn endpoint(&self) -> String {
        "setChatTitle".to_string()
    }
}

/// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetChatDescription {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SetChatDescription {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self {
            chat_id,
            description: None,
        }
    }
}

impl Methods for SetChatDescription {
    fn endpoint(&self) -> String {
        "setChatDescription".to_string()
    }
}

/// Use this method to add a message to the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Identifier of a message to pin
    pub message_id: i64,
    /// Pass True if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}
impl PinChatMessage {
    pub fn new(chat_id: types::ChatId, message_id: i64) -> Self {
        Self {
            chat_id,
            message_id,
            disable_notification: None,
        }
    }
}

impl Methods for PinChatMessage {
    fn endpoint(&self) -> String {
        "pinChatMessage".to_string()
    }
}

/// Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UnpinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Identifier of a message to unpin. If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}
impl UnpinChatMessage {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self {
            chat_id,
            message_id: None,
        }
    }
}

impl Methods for UnpinChatMessage {
    fn endpoint(&self) -> String {
        "unpinChatMessage".to_string()
    }
}

/// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UnpinAllChatMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
}
impl UnpinAllChatMessages {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self { chat_id }
    }
}

impl Methods for UnpinAllChatMessages {
    fn endpoint(&self) -> String {
        "unpinAllChatMessages".to_string()
    }
}

/// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LeaveChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: types::ChatId,
}
impl LeaveChat {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self { chat_id }
    }
}

impl Methods for LeaveChat {
    fn endpoint(&self) -> String {
        "leaveChat".to_string()
    }
}

/// Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.). Returns a Chat object on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: types::ChatId,
}
impl GetChat {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self { chat_id }
    }
}

impl Methods for GetChat {
    fn endpoint(&self) -> String {
        "getChat".to_string()
    }
}

/// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of ChatMember objects.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: types::ChatId,
}
impl GetChatAdministrators {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self { chat_id }
    }
}

impl Methods for GetChatAdministrators {
    fn endpoint(&self) -> String {
        "getChatAdministrators".to_string()
    }
}

/// Use this method to get the number of members in a chat. Returns Int on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetChatMemberCount {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: types::ChatId,
}
impl GetChatMemberCount {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self { chat_id }
    }
}

impl Methods for GetChatMemberCount {
    fn endpoint(&self) -> String {
        "getChatMemberCount".to_string()
    }
}

/// Use this method to get information about a member of a chat. Returns a ChatMember object on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Unique identifier of the target user
    pub user_id: i64,
}
impl GetChatMember {
    pub fn new(chat_id: types::ChatId, user_id: i64) -> Self {
        Self { chat_id, user_id }
    }
}

impl Methods for GetChatMember {
    fn endpoint(&self) -> String {
        "getChatMember".to_string()
    }
}

/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: types::ChatId,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}
impl SetChatStickerSet {
    pub fn new(chat_id: types::ChatId, sticker_set_name: String) -> Self {
        Self {
            chat_id,
            sticker_set_name,
        }
    }
}

impl Methods for SetChatStickerSet {
    fn endpoint(&self) -> String {
        "setChatStickerSet".to_string()
    }
}

/// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: types::ChatId,
}
impl DeleteChatStickerSet {
    pub fn new(chat_id: types::ChatId) -> Self {
        Self { chat_id }
    }
}

impl Methods for DeleteChatStickerSet {
    fn endpoint(&self) -> String {
        "deleteChatStickerSet".to_string()
    }
}

/// Use this method to send answers to callback queries sent from inline keyboards. The answer will be displayed to the user as a notification at the top of the chat screen or as an alert. On success, True is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered
    pub callback_query_id: String,
    /// Text of the notification. If not specified, nothing will be shown to the user, 0-200 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// If True, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    /// URL that will be opened by the user's client. If you have created a Game and accepted the conditions via @BotFather, specify the URL that opens your game - note that this will only work if the query comes from a callback_game button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl AnswerCallbackQuery {
    pub fn new(callback_query_id: String) -> Self {
        Self {
            callback_query_id,
            text: None,
            show_alert: None,
            url: None,
        }
    }
}

impl Methods for AnswerCallbackQuery {
    fn endpoint(&self) -> String {
        "answerCallbackQuery".to_string()
    }
}

/// Use this method to change the list of the bot's commands. See https://core.telegram.org/bots#commands for more details about bot commands. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetMyCommands {
    /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
    pub commands: Vec<types::BotCommand>,
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<types::BotCommandScope>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
impl SetMyCommands {
    pub fn new(commands: Vec<types::BotCommand>) -> Self {
        Self {
            commands,
            scope: None,
            language_code: None,
        }
    }
}

impl Methods for SetMyCommands {
    fn endpoint(&self) -> String {
        "setMyCommands".to_string()
    }
}

/// Use this method to delete the list of the bot's commands for the given scope and user language. After deletion, higher level commands will be shown to affected users. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteMyCommands {
    /// A JSON-serialized object, describing scope of users for which the commands are relevant. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<types::BotCommandScope>,
    /// A two-letter ISO 639-1 language code. If empty, commands will be applied to all users from the given scope, for whose language there are no dedicated commands
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
impl DeleteMyCommands {
    pub fn new() -> Self {
        Self {
            scope: None,
            language_code: None,
        }
    }
}

impl Methods for DeleteMyCommands {
    fn endpoint(&self) -> String {
        "deleteMyCommands".to_string()
    }
}

/// Use this method to get the current list of the bot's commands for the given scope and user language. Returns an Array of BotCommand objects. If commands aren't set, an empty list is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetMyCommands {
    /// A JSON-serialized object, describing scope of users. Defaults to BotCommandScopeDefault.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<types::BotCommandScope>,
    /// A two-letter ISO 639-1 language code or an empty string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}
impl GetMyCommands {
    pub fn new() -> Self {
        Self {
            scope: None,
            language_code: None,
        }
    }
}

impl Methods for GetMyCommands {
    fn endpoint(&self) -> String {
        "getMyCommands".to_string()
    }
}

/// Use this method to change the bot's menu button in a private chat, or the default menu button. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetChatMenuButton {
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// A JSON-serialized object for the bot's new menu button. Defaults to MenuButtonDefault
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<types::MenuButton>,
}
impl SetChatMenuButton {
    pub fn new() -> Self {
        Self {
            chat_id: None,
            menu_button: None,
        }
    }
}

impl Methods for SetChatMenuButton {
    fn endpoint(&self) -> String {
        "setChatMenuButton".to_string()
    }
}

/// Use this method to get the current value of the bot's menu button in a private chat, or the default menu button. Returns MenuButton on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetChatMenuButton {
    /// Unique identifier for the target private chat. If not specified, default bot's menu button will be returned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
}
impl GetChatMenuButton {
    pub fn new() -> Self {
        Self { chat_id: None }
    }
}

impl Methods for GetChatMenuButton {
    fn endpoint(&self) -> String {
        "getChatMenuButton".to_string()
    }
}

/// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are are free to modify the list before adding the bot. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetMyDefaultAdministratorRights {
    /// A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<types::ChatAdministratorRights>,
    /// Pass True to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}
impl SetMyDefaultAdministratorRights {
    pub fn new() -> Self {
        Self {
            rights: None,
            for_channels: None,
        }
    }
}

impl Methods for SetMyDefaultAdministratorRights {
    fn endpoint(&self) -> String {
        "setMyDefaultAdministratorRights".to_string()
    }
}

/// Use this method to get the current default administrator rights of the bot. Returns ChatAdministratorRights on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetMyDefaultAdministratorRights {
    /// Pass True to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}
impl GetMyDefaultAdministratorRights {
    pub fn new() -> Self {
        Self { for_channels: None }
    }
}

impl Methods for GetMyDefaultAdministratorRights {
    fn endpoint(&self) -> String {
        "getMyDefaultAdministratorRights".to_string()
    }
}

/// Use this method to receive incoming updates using long polling (wiki). Returns an Array of Update objects.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will forgotten.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of updates to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except chat_member (default). If not specified, the previous setting will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}
impl GetUpdates {
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }
}

impl Methods for GetUpdates {
    fn endpoint(&self) -> String {
        "getUpdates".to_string()
    }
}

/// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetWebhook {
    /// HTTPS URL to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    #[serde(skip_serializing)]
    pub certificate: Option<types::InputFile>,
    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except chat_member (default). If not specified, the previous setting will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
    /// A secret token to be sent in a header “X-Telegram-Bot-Api-Secret-Token” in every webhook request, 1-256 characters. Only characters A-Z, a-z, 0-9, _ and - are allowed. The header is useful to ensure that the request comes from a webhook set by you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}
impl SetWebhook {
    pub fn new(url: String) -> Self {
        Self {
            url,
            certificate: None,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
            secret_token: None,
        }
    }
}

impl Methods for SetWebhook {
    fn endpoint(&self) -> String {
        "setWebhook".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        if let Some(certificate) = &self.certificate {
            result.insert("certificate".to_string(), certificate.clone());
        }
        result
    }
}

/// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteWebhook {
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}
impl DeleteWebhook {
    pub fn new() -> Self {
        Self {
            drop_pending_updates: None,
        }
    }
}

impl Methods for DeleteWebhook {
    fn endpoint(&self) -> String {
        "deleteWebhook".to_string()
    }
}

/// Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetWebhookInfo {}
impl GetWebhookInfo {
    pub fn new() -> Self {
        Self {}
    }
}

impl Methods for GetWebhookInfo {
    fn endpoint(&self) -> String {
        "getWebhookInfo".to_string()
    }
}

/// Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendSticker {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files »
    #[serde(skip_serializing)]
    pub sticker: types::InputFile,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::ReplyMarkup>,
}
impl SendSticker {
    pub fn new(chat_id: types::ChatId, sticker: types::InputFile) -> Self {
        Self {
            chat_id,
            sticker,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendSticker {
    fn endpoint(&self) -> String {
        "sendSticker".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        result.insert("sticker".to_string(), self.sticker.clone());
        result
    }
}

/// Use this method to get a sticker set. On success, a StickerSet object is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}
impl GetStickerSet {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Methods for GetStickerSet {
    fn endpoint(&self) -> String {
        "getStickerSet".to_string()
    }
}

/// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of Sticker objects.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetCustomEmojiStickers {
    /// List of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    pub custom_emoji_ids: Vec<String>,
}
impl GetCustomEmojiStickers {
    pub fn new(custom_emoji_ids: Vec<String>) -> Self {
        Self { custom_emoji_ids }
    }
}

impl Methods for GetCustomEmojiStickers {
    fn endpoint(&self) -> String {
        "getCustomEmojiStickers".to_string()
    }
}

/// Use this method to upload a .PNG file with a sticker for later use in createNewStickerSet and addStickerToSet methods (can be used multiple times). Returns the uploaded File on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: i64,
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. More information on Sending Files »
    #[serde(skip_serializing)]
    pub png_sticker: types::InputFile,
}
impl UploadStickerFile {
    pub fn new(user_id: i64, png_sticker: types::InputFile) -> Self {
        Self {
            user_id,
            png_sticker,
        }
    }
}

impl Methods for UploadStickerFile {
    fn endpoint(&self) -> String {
        "uploadStickerFile".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        result.insert("png_sticker".to_string(), self.png_sticker.clone());
        result
    }
}

/// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. You must use exactly one of the fields png_sticker, tgs_sticker, or webm_sticker. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateNewStickerSet {
    /// User identifier of created sticker set owner
    pub user_id: i64,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in _by_<bot_username>. <bot_username> is case insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files »
    #[serde(skip_serializing)]
    pub png_sticker: Option<types::InputFile>,
    /// TGS animation with the sticker, uploaded using multipart/form-data. See https://core.telegram.org/stickers#animated-sticker-requirements for technical requirements
    #[serde(skip_serializing)]
    pub tgs_sticker: Option<types::InputFile>,
    /// WEBM video with the sticker, uploaded using multipart/form-data. See https://core.telegram.org/stickers#video-sticker-requirements for technical requirements
    #[serde(skip_serializing)]
    pub webm_sticker: Option<types::InputFile>,
    /// Type of stickers in the set, pass “regular” or “mask”. Custom emoji sticker sets can't be created via the Bot API at the moment. By default, a regular sticker set is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<String>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<types::MaskPosition>,
}
impl CreateNewStickerSet {
    pub fn new(user_id: i64, name: String, title: String, emojis: String) -> Self {
        Self {
            user_id,
            name,
            title,
            png_sticker: None,
            tgs_sticker: None,
            webm_sticker: None,
            sticker_type: None,
            emojis,
            mask_position: None,
        }
    }
}

impl Methods for CreateNewStickerSet {
    fn endpoint(&self) -> String {
        "createNewStickerSet".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        if let Some(png_sticker) = &self.png_sticker {
            result.insert("png_sticker".to_string(), png_sticker.clone());
        }
        if let Some(tgs_sticker) = &self.tgs_sticker {
            result.insert("tgs_sticker".to_string(), tgs_sticker.clone());
        }
        if let Some(webm_sticker) = &self.webm_sticker {
            result.insert("webm_sticker".to_string(), webm_sticker.clone());
        }
        result
    }
}

/// Use this method to add a new sticker to a set created by the bot. You must use exactly one of the fields png_sticker, tgs_sticker, or webm_sticker. Animated stickers can be added to animated sticker sets and only to them. Animated sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddStickerToSet {
    /// User identifier of sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files »
    #[serde(skip_serializing)]
    pub png_sticker: Option<types::InputFile>,
    /// TGS animation with the sticker, uploaded using multipart/form-data. See https://core.telegram.org/stickers#animated-sticker-requirements for technical requirements
    #[serde(skip_serializing)]
    pub tgs_sticker: Option<types::InputFile>,
    /// WEBM video with the sticker, uploaded using multipart/form-data. See https://core.telegram.org/stickers#video-sticker-requirements for technical requirements
    #[serde(skip_serializing)]
    pub webm_sticker: Option<types::InputFile>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<types::MaskPosition>,
}
impl AddStickerToSet {
    pub fn new(user_id: i64, name: String, emojis: String) -> Self {
        Self {
            user_id,
            name,
            png_sticker: None,
            tgs_sticker: None,
            webm_sticker: None,
            emojis,
            mask_position: None,
        }
    }
}

impl Methods for AddStickerToSet {
    fn endpoint(&self) -> String {
        "addStickerToSet".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        if let Some(png_sticker) = &self.png_sticker {
            result.insert("png_sticker".to_string(), png_sticker.clone());
        }
        if let Some(tgs_sticker) = &self.tgs_sticker {
            result.insert("tgs_sticker".to_string(), tgs_sticker.clone());
        }
        if let Some(webm_sticker) = &self.webm_sticker {
            result.insert("webm_sticker".to_string(), webm_sticker.clone());
        }
        result
    }
}

/// Use this method to move a sticker in a set created by the bot to a specific position. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: i64,
}
impl SetStickerPositionInSet {
    pub fn new(sticker: String, position: i64) -> Self {
        Self { sticker, position }
    }
}

impl Methods for SetStickerPositionInSet {
    fn endpoint(&self) -> String {
        "setStickerPositionInSet".to_string()
    }
}

/// Use this method to delete a sticker from a set created by the bot. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    pub sticker: String,
}
impl DeleteStickerFromSet {
    pub fn new(sticker: String) -> Self {
        Self { sticker }
    }
}

impl Methods for DeleteStickerFromSet {
    fn endpoint(&self) -> String {
        "deleteStickerFromSet".to_string()
    }
}

/// Use this method to set the thumbnail of a sticker set. Animated thumbnails can be set for animated sticker sets only. Video thumbnails can be set only for video sticker sets only. Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetStickerSetThumb {
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// A PNG image with the thumbnail, must be up to 128 kilobytes in size and have width and height exactly 100px, or a TGS animation with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/stickers#animated-sticker-requirements for animated sticker technical requirements, or a WEBM video with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/stickers#video-sticker-requirements for video sticker technical requirements. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files ». Animated sticker set thumbnails can't be uploaded via HTTP URL.
    #[serde(skip_serializing)]
    pub thumb: Option<types::InputFile>,
}
impl SetStickerSetThumb {
    pub fn new(name: String, user_id: i64) -> Self {
        Self {
            name,
            user_id,
            thumb: None,
        }
    }
}

impl Methods for SetStickerSetThumb {
    fn endpoint(&self) -> String {
        "setStickerSetThumb".to_string()
    }

    fn files(&self) -> HashMap<String, types::InputFile> {
        let mut result = HashMap::new();
        if let Some(thumb) = &self.thumb {
            result.insert("thumb".to_string(), thumb.clone());
        }
        result
    }
}

/// Use this method to send answers to an inline query. On success, True is returned. No more than 50 results per query are allowed.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnswerInlineQuery {
    /// Unique identifier for the answered query
    pub inline_query_id: String,
    /// A JSON-serialized array of results for the inline query
    pub results: Vec<types::InlineQueryResult>,
    /// The maximum amount of time in seconds that the result of the inline query may be cached on the server. Defaults to 300.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i64>,
    /// Pass True if results may be cached on the server side only for the user that sent the query. By default, results may be returned to any user who sends the same query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// Pass the offset that a client should send in the next query with the same text to receive more results. Pass an empty string if there are no more results or if you don't support pagination. Offset length can't exceed 64 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// If passed, clients will display a button with specified text that switches the user to a private chat with the bot and sends the bot a start message with the parameter switch_pm_parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_text: Option<String>,
    /// Deep-linking parameter for the /start message sent to the bot when user presses the switch button. 1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_pm_parameter: Option<String>,
}
impl AnswerInlineQuery {
    pub fn new(inline_query_id: String, results: Vec<types::InlineQueryResult>) -> Self {
        Self {
            inline_query_id,
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }
}

impl Methods for AnswerInlineQuery {
    fn endpoint(&self) -> String {
        "answerInlineQuery".to_string()
    }
}

/// Use this method to set the result of an interaction with a Web App and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a SentWebAppMessage object is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnswerWebAppQuery {
    /// Unique identifier for the query to be answered
    pub web_app_query_id: String,
    /// A JSON-serialized object describing the message to be sent
    pub result: types::InlineQueryResult,
}
impl AnswerWebAppQuery {
    pub fn new(web_app_query_id: String, result: types::InlineQueryResult) -> Self {
        Self {
            web_app_query_id,
            result,
        }
    }
}

impl Methods for AnswerWebAppQuery {
    fn endpoint(&self) -> String {
        "answerWebAppQuery".to_string()
    }
}

/// Use this method to send invoices. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendInvoice {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: types::ChatId,
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
    pub prices: Vec<types::LabeledPrice>,
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// Unique deep-linking parameter. If left empty, forwarded copies of the sent message will have a Pay button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a URL button with a deep link to the bot (instead of a Pay button), with the value used as the start parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Pass True if you require the user's full name to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Pass True if you require the user's phone number to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Pass True if you require the user's email address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Pass True if you require the user's shipping address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Pass True if the user's phone number should be sent to provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass True if the user's email address should be sent to provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Pass True if the final price depends on the shipping method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}
impl SendInvoice {
    pub fn new(
        chat_id: types::ChatId,
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<types::LabeledPrice>,
    ) -> Self {
        Self {
            chat_id,
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            start_parameter: None,
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
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendInvoice {
    fn endpoint(&self) -> String {
        "sendInvoice".to_string()
    }
}

/// Use this method to create a link for an invoice. Returns the created invoice link as String on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateInvoiceLink {
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Payment provider token, obtained via BotFather
    pub provider_token: String,
    /// Three-letter ISO 4217 currency code, see more on currencies
    pub currency: String,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<types::LabeledPrice>,
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed max_tip_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Pass True if you require the user's full name to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Pass True if you require the user's phone number to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Pass True if you require the user's email address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Pass True if you require the user's shipping address to complete the order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Pass True if the user's phone number should be sent to the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass True if the user's email address should be sent to the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Pass True if the final price depends on the shipping method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}
impl CreateInvoiceLink {
    pub fn new(
        title: String,
        description: String,
        payload: String,
        provider_token: String,
        currency: String,
        prices: Vec<types::LabeledPrice>,
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

impl Methods for CreateInvoiceLink {
    fn endpoint(&self) -> String {
        "createInvoiceLink".to_string()
    }
}

/// If you sent an invoice requesting a shipping address and the parameter is_flexible was specified, the Bot API will send an Update with a shipping_query field to the bot. Use this method to reply to shipping queries. On success, True is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnswerShippingQuery {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Pass True if delivery to the specified address is possible and False if there are any problems (for example, if delivery to the specified address is not possible)
    pub ok: bool,
    /// Required if ok is True. A JSON-serialized array of available shipping options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<types::ShippingOption>>,
    /// Required if ok is False. Error message in human readable form that explains why it is impossible to complete the order (e.g. 'Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl AnswerShippingQuery {
    pub fn new(shipping_query_id: String, ok: bool) -> Self {
        Self {
            shipping_query_id,
            ok,
            shipping_options: None,
            error_message: None,
        }
    }
}

impl Methods for AnswerShippingQuery {
    fn endpoint(&self) -> String {
        "answerShippingQuery".to_string()
    }
}

/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field pre_checkout_query. Use this method to respond to such pre-checkout queries. On success, True is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnswerPreCheckoutQuery {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
    /// Specify True if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use False if there are any problems.
    pub ok: bool,
    /// Required if ok is False. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. 'Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!'). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl AnswerPreCheckoutQuery {
    pub fn new(pre_checkout_query_id: String, ok: bool) -> Self {
        Self {
            pre_checkout_query_id,
            ok,
            error_message: None,
        }
    }
}

impl Methods for AnswerPreCheckoutQuery {
    fn endpoint(&self) -> String {
        "answerPreCheckoutQuery".to_string()
    }
}

/// Informs a user that some of the Telegram Passport elements they provided contains errors. The user will not be able to re-submit their Passport to you until the errors are fixed (the contents of the field for which you returned the error must change). Returns True on success.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetPassportDataErrors {
    /// User identifier
    pub user_id: i64,
    /// A JSON-serialized array describing the errors
    pub errors: Vec<types::PassportElementError>,
}
impl SetPassportDataErrors {
    pub fn new(user_id: i64, errors: Vec<types::PassportElementError>) -> Self {
        Self { user_id, errors }
    }
}

impl Methods for SetPassportDataErrors {
    fn endpoint(&self) -> String {
        "setPassportDataErrors".to_string()
    }
}

/// Use this method to send a game. On success, the sent Message is returned.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SendGame {
    /// Unique identifier for the target chat
    pub chat_id: i64,
    /// Short name of the game, serves as the unique identifier for the game. Set up your games via @BotFather.
    pub game_short_name: String,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Play game_title' button will be shown. If not empty, the first button must launch the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<types::InlineKeyboardMarkup>,
}
impl SendGame {
    pub fn new(chat_id: i64, game_short_name: String) -> Self {
        Self {
            chat_id,
            game_short_name,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Methods for SendGame {
    fn endpoint(&self) -> String {
        "sendGame".to_string()
    }
}

/// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the Message is returned, otherwise True is returned. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SetGameScore {
    /// User identifier
    pub user_id: i64,
    /// New score, must be non-negative
    pub score: i64,
    /// Pass True if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// Pass True if the game message should not be automatically edited to include the current scoreboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
impl SetGameScore {
    pub fn new(user_id: i64, score: i64) -> Self {
        Self {
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }
}

impl Methods for SetGameScore {
    fn endpoint(&self) -> String {
        "setGameScore".to_string()
    }
}

/// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of GameHighScore objects.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GetGameHighScores {
    /// Target user id
    pub user_id: i64,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}
impl GetGameHighScores {
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }
}

impl Methods for GetGameHighScores {
    fn endpoint(&self) -> String {
        "getGameHighScores".to_string()
    }
}

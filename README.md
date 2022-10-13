# Rust bindings for the Telegram Bot API

Reference golang project [go-telegram-bot-api](https://github.com/go-telegram-bot-api/telegram-bot-api.git)

All methods are fairly self-explanatory, If something isn't clear, open an issue or submit a pull request.

The scope of this project is just to provide a wrapper around the API without any additional features. There are other projects for creating something with plugins and command handlers without having to design all that yourself.

## Example
First, add dependencies to your `Cargo.toml`:
```
[dependencies]
telegram-bot-api = "0.1.0"
```

```rust
use telegram_bot_api::{bot, methods, types};

#[tokio::main]
async fn main() {
    let bot = bot::BotApi::new(String::from("token"), None).await;
    if bot.is_err() {
        panic!("{:?}", bot);
    }
    let bot = bot.unwrap();

    match bot.get_me().await {
        Ok(user) => {
            println!("get_me result: {:#?}", user);
        }
        Err(err) => {
            println!("err:{:?}", err);
        }
    }

    let mut entities = Vec::with_capacity(1);
    entities.push(types::MessageEntity::new_url(6, 22));
    let mut send_message = methods::SendMessage::new(
        types::ChatId::IntType(123456),
        String::from("hello https://www.google.com"),
    );
    send_message.entities = Some(entities);
    let mut btn = types::InlineKeyboardButton::new(String::from("google"));
    btn.url = Some(String::from("https://www.google.com"));
    let mut btn2 = types::InlineKeyboardButton::new(String::from("telegram"));
    btn2.url = Some(String::from("https://telegram.org"));
    let keyboard = vec![vec![btn, btn2]];
    send_message.reply_markup = Some(types::ReplyMarkup::InlineKeyboardMarkup(
        types::InlineKeyboardMarkup::new(keyboard),
    ));
    match bot.send_message(send_message).await {
        Ok(message) => {
            println!("send_message result: {:#?}", message);
        }
        Err(err) => {
            println!("err:{:?}", err);
        }
    }

    let mut get_chat_menu_button = methods::GetChatMenuButton::new();
    get_chat_menu_button.chat_id = Some(123456);
    match bot.get_chat_menu_button(get_chat_menu_button).await {
        Ok(result) => match result {
            _ => {
                println!("get_chat_menu_button result: {:#?}", result);
            }
        },
        Err(err) => {
            println!("err:{:?}", err);
        }
    }

    let mut media = Vec::new();
    media.push(types::InputMedia::InputMediaPhoto(
        types::InputMediaPhoto::new(types::InputFile::FilePath(String::from("logo_256.png"))),
    ));
    media.push(types::InputMedia::InputMediaPhoto(
        types::InputMediaPhoto::new(types::InputFile::FileID(
            String::from("AgACAgQAAx0EYIAjxwADkGNGxUhE7P-hV_54gWPJs_IKj7v4AAK3ujEbNBcRUoJ4Nhu7bEmQAQADAgADcwADKgQ"),
        )),
    ));
    media.push(types::InputMedia::InputMediaPhoto(
        types::InputMediaPhoto::new(types::InputFile::FilePath(String::from("iconbig_green.png"))),
    ));
    match bot
        .send_media_group(methods::SendMediaGroup::new(
            types::ChatId::IntType(123456),
            media,
        ))
        .await
    {
        Ok(result) => {
            println!("send_media_group result: {:#?}", result);
        }
        Err(err) => {
            println!("err:{:?}", err);
        }
    }
}

```
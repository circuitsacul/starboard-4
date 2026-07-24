use std::{str::FromStr, sync::LazyLock};

use regex::Regex;
use twilight_http::request::channel::reaction::RequestReactionType;
use twilight_mention::Mention;
use twilight_model::{
    channel::message::EmojiReactionType,
    id::{
        Id,
        marker::{EmojiMarker, GuildMarker},
    },
};
use unicode_segmentation::UnicodeSegmentation;

use crate::client::bot::StarboardBot;

/// Get rid of the Variation-Selector-16 codepoint that is sometimes present in user
/// input. https://emojipedia.org/variation-selector-16/
pub fn remove_v16(target: &str) -> &str {
    target.strip_suffix('\u{fe0f}').unwrap_or(target)
}

pub fn qualify_emoji(target: &str) -> &str {
    emojis::get(target).map(|e| e.as_str()).unwrap_or(target)
}

pub fn compare_unicode_emojis(left: &str, right: &str) -> bool {
    qualify_emoji(left) == qualify_emoji(right)
}

#[derive(Clone)]
pub struct SimpleEmoji {
    raw: String,
    as_id: Option<Id<EmojiMarker>>,
}

impl PartialEq for SimpleEmoji {
    fn eq(&self, other: &Self) -> bool {
        compare_unicode_emojis(&self.raw, &other.raw)
    }
}

impl PartialEq<String> for SimpleEmoji {
    fn eq(&self, other: &String) -> bool {
        compare_unicode_emojis(&self.raw, other)
    }
}

impl SimpleEmoji {
    fn new(raw: String, as_id: Option<Id<EmojiMarker>>) -> Self {
        Self { raw, as_id }
    }

    pub fn from_user_input(
        mut input: &str,
        bot: &StarboardBot,
        guild_id: Id<GuildMarker>,
    ) -> Vec<Self> {
        static CUSTOM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{10,}").unwrap());

        let mut emojis = Vec::new();

        while !input.is_empty() {
            if let Some(m) = CUSTOM.find(input) {
                input = &input[m.end()..];

                let Ok(id) = m.as_str().parse() else {
                    continue;
                };
                if !bot.cache.guild_emoji_exists(guild_id, id) {
                    continue;
                }
                emojis.push(SimpleEmoji::new(id.to_string(), Some(id)));
            } else if let Some(cluster) = input.graphemes(true).next() {
                input = &input[cluster.len()..];

                let Some(emoji) = emojis::get(cluster) else {
                    continue;
                };

                emojis.push(SimpleEmoji::new(emoji.to_string(), None));
            } else {
                break;
            }
        }

        emojis
    }
}

pub trait EmojiCommon: Sized {
    type FromOut;
    type Stored;

    fn into_readable(self, bot: &StarboardBot, guild_id: Id<GuildMarker>) -> String;
    fn into_stored(self) -> Self::Stored;
    fn from_stored(stored: Self::Stored) -> Self;
}

impl SimpleEmoji {
    pub fn reactable(&'_ self) -> RequestReactionType<'_> {
        if let Some(emoji_id) = self.as_id {
            RequestReactionType::Custom {
                name: None,
                id: emoji_id,
            }
        } else {
            RequestReactionType::Unicode {
                name: remove_v16(qualify_emoji(&self.raw)),
            }
        }
    }
}

impl EmojiCommon for SimpleEmoji {
    type FromOut = Option<Self>;
    type Stored = String;

    fn into_readable(self, bot: &StarboardBot, guild_id: Id<GuildMarker>) -> String {
        if let Some(emoji_id) = self.as_id {
            match bot.cache.is_emoji_animated(guild_id, emoji_id) {
                None => self.raw,
                Some(true) => format!("<a:name:{emoji_id}>"),
                Some(false) => emoji_id.mention().to_string(),
            }
        } else {
            qualify_emoji(&self.raw).to_string()
        }
    }

    fn from_stored(raw: Self::Stored) -> Self {
        let as_id = Id::<EmojiMarker>::from_str(&raw).ok();

        Self::new(raw, as_id)
    }

    fn into_stored(self) -> Self::Stored {
        self.raw
    }
}

impl EmojiCommon for Vec<SimpleEmoji> {
    type FromOut = Self;
    type Stored = Vec<String>;

    fn into_readable(self, bot: &StarboardBot, guild_id: Id<GuildMarker>) -> String {
        let mut arr = Vec::new();
        for emoji in self {
            arr.push(emoji.into_readable(bot, guild_id));
        }
        if arr.is_empty() {
            "no emojis".to_string()
        } else {
            arr.join(", ")
        }
    }

    fn from_stored(stored: Self::Stored) -> Self {
        let mut arr = Vec::new();
        for piece in stored {
            arr.push(SimpleEmoji::from_stored(piece));
        }
        arr
    }

    fn into_stored(self) -> Self::Stored {
        let mut arr = Vec::new();
        for emoji in self {
            arr.push(emoji.into_stored());
        }
        arr
    }
}

impl From<EmojiReactionType> for SimpleEmoji {
    fn from(reaction: EmojiReactionType) -> Self {
        match reaction {
            EmojiReactionType::Custom { id, .. } => SimpleEmoji::new(id.to_string(), Some(id)),
            EmojiReactionType::Unicode { name } => SimpleEmoji::new(name, None),
        }
    }
}

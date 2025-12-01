use serde::Serialize;
#[allow(clippy::doc_markdown)]
/// Main webhook payload structure for sending messages to Discord
#[derive(Serialize, Default, Debug, Clone)]
pub struct WebhookPayload {
    /// Message content (up to 2000 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Override the webhook's default username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Override the webhook's default avatar
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// Enable text-to-speech for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    /// Array of rich embed objects (up to 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
    /// Control which mentions are parsed from the content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,
    /// Message components (buttons, select menus)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
    /// Attachment objects with filenames and descriptions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// Name of thread to create (requires thread in forum/media channel)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_name: Option<String>,
    /// Message flags (e.g., SUPPRESS_EMBEDS = 1 << 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
}

/// Rich embed object for formatted messages
#[derive(Serialize, Default, Debug, Clone)]
pub struct Embed {
    /// Title of the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Description text (supports Markdown)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL to make the title a hyperlink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// ISO8601 timestamp for the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Color code as a decimal integer (not hex)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,
    /// Footer information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,
    /// Full-size image displayed in the embed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,
    /// Small thumbnail image in top-right corner
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedThumbnail>,
    /// Author information displayed above the title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,
    /// Array of field objects (up to 25)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<EmbedField>>,
}

/// Footer section of an embed
#[derive(Serialize, Debug, Clone)]
pub struct EmbedFooter {
    /// Footer text
    pub text: String,
    /// URL of footer icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

/// Image displayed in an embed
#[derive(Serialize, Debug, Clone)]
pub struct EmbedImage {
    /// Source URL of the image
    pub url: String,
}

/// Thumbnail image for an embed
#[derive(Serialize, Debug, Clone)]
pub struct EmbedThumbnail {
    /// Source URL of the thumbnail
    pub url: String,
}

/// Author information for an embed
#[derive(Serialize, Debug, Clone)]
pub struct EmbedAuthor {
    /// Name of the author
    pub name: String,
    /// URL to make the name a hyperlink
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// URL of author icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
}

/// Individual field within an embed
#[derive(Serialize, Debug, Clone)]
pub struct EmbedField {
    /// Name of the field
    pub name: String,
    /// Value of the field
    pub value: String,
    /// Whether to display inline (max 3 per row)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}

/// Configuration for which mentions are parsed
#[derive(Serialize, Default, Debug, Clone)]
pub struct AllowedMentions {
    /// Types of mentions to parse: "roles", "users", "everyone"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse: Option<Vec<String>>,
    /// Array of role IDs to allow mentions for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// Array of user IDs to allow mentions for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    /// Whether to mention the author of the message being replied to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replied_user: Option<bool>,
}

#[allow(clippy::doc_markdown)]
/// Message component (buttons, select menus)
#[derive(Serialize, Debug, Clone)]
pub struct Component {
    /// Component type: 1 = ActionRow, 2 = Button, 3 = SelectMenu, 4 = TextInput
    #[serde(rename = "type")]
    pub component_type: u8,
    /// Child components (for ActionRow)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,
    /// Button style: 1 = Primary, 2 = Secondary, 3 = Success, 4 = Danger, 5 = Link
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<u8>,
    /// Label text for buttons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Emoji to display on component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,
    /// Developer-defined identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// URL for link-style buttons
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether the component is disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Choices for select menus
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SelectOption>>,
    /// Placeholder text for select menus
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Minimum number of selections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,
    /// Maximum number of selections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,
}

/// Emoji for components or reactions
#[derive(Serialize, Debug, Clone)]
pub struct Emoji {
    /// Custom emoji ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Unicode emoji or custom emoji name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the emoji is animated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

/// Option in a select menu
#[derive(Serialize, Debug, Clone)]
pub struct SelectOption {
    /// User-facing option label
    pub label: String,
    /// Developer-defined value
    pub value: String,
    /// Additional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Emoji to display with the option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,
    /// Whether this option is selected by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

/// Attachment metadata for files
#[derive(Serialize, Debug, Clone)]
pub struct Attachment {
    /// Attachment ID
    pub id: String,
    /// Name of the file
    pub filename: String,
    /// Description of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// Convenience implementations

impl WebhookPayload {
    /// Create a simple text message
    pub fn new_text(content: impl Into<String>) -> Self {
        Self {
            content: Some(content.into()),
            ..Default::default()
        }
    }
    #[must_use]
    /// Create a message with a single embed
    pub fn new_embed(embed: Embed) -> Self {
        Self {
            embeds: Some(vec![embed]),
            ..Default::default()
        }
    }
    #[must_use]
    /// Add or replace message content
    pub fn with_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }
    #[must_use]
    /// Override the webhook's username
    pub fn with_username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
    #[must_use]
    /// Override the webhook's avatar
    pub fn with_avatar_url(mut self, url: impl Into<String>) -> Self {
        self.avatar_url = Some(url.into());
        self
    }
    #[must_use]
    /// Add an embed to the message
    pub fn add_embed(mut self, embed: Embed) -> Self {
        self.embeds.get_or_insert_with(Vec::new).push(embed);
        self
    }
    #[must_use]
    /// Enable text-to-speech
    pub const fn with_tts(mut self, tts: bool) -> Self {
        self.tts = Some(tts);
        self
    }
}

impl Embed {
    #[must_use]
    /// Create a new embed with a title
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    /// Set the embed title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    #[must_use]
    /// Set the embed description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    /// Set the embed color (as decimal)
    pub const fn with_color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    #[must_use]
    /// Set the embed URL
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    #[must_use]
    /// Add a field to the embed
    pub fn add_field(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        inline: bool,
    ) -> Self {
        self.fields.get_or_insert_with(Vec::new).push(EmbedField {
            name: name.into(),
            value: value.into(),
            inline: Some(inline),
        });
        self
    }

    #[must_use]
    /// Set the footer
    pub fn with_footer(
        mut self,
        text: impl Into<String>,
        icon_url: Option<String>,
    ) -> Self {
        self.footer = Some(EmbedFooter {
            text: text.into(),
            icon_url,
        });
        self
    }

    #[must_use]
    /// Set the author
    pub fn with_author(
        mut self,
        name: impl Into<String>,
        url: Option<String>,
        icon_url: Option<String>,
    ) -> Self {
        self.author = Some(EmbedAuthor {
            name: name.into(),
            url,
            icon_url,
        });
        self
    }

    #[must_use]
    /// Set the thumbnail
    pub fn with_thumbnail(mut self, url: impl Into<String>) -> Self {
        self.thumbnail = Some(EmbedThumbnail {
            url: url.into(),
        });
        self
    }

    #[must_use]
    /// Set the image
    pub fn with_image(mut self, url: impl Into<String>) -> Self {
        self.image = Some(EmbedImage {
            url: url.into(),
        });
        self
    }

    // /// Set the timestamp to now
    // pub fn with_timestamp_now(mut self) -> Self {
    //     self.timestamp = Some(chrono::Utc::now().to_rfc3339());
    //     self
    // }
}

impl AllowedMentions {
    #[must_use]
    /// Create allowed mentions config that allows nothing
    pub fn none() -> Self {
        Self {
            parse: Some(vec![]),
            ..Default::default()
        }
    }

    #[must_use]
    /// Create allowed mentions config that allows everything
    pub fn all() -> Self {
        Self {
            parse: Some(vec![
                "roles".to_string(),
                "users".to_string(),
                "everyone".to_string(),
            ]),
            ..Default::default()
        }
    }

    #[must_use]
    /// Allow only user mentions
    pub fn users_only() -> Self {
        Self {
            parse: Some(vec!["users".to_string()]),
            ..Default::default()
        }
    }
}

use reqwest::blocking::Client;
/// Send a message to Discord via webhook
///
/// Use [`WebhookPayload::default()`](WebhookPayload::default) to customize the payload you wanna send
///
/// If the message is longer than 2000 characters, it will automatically split and send in 2000 character chunks
///
/// # Example
/// ```
/// fn function() {
///     let url = "{Your webhook}";
///     let payload = mirl::misc::discord::WebhookPayload::default()
///                     .with_content("This message will be send :)");
///     mirl::misc::discord::send_discord_message(
///                url,
///                &payload,
///            ).unwrap();
///     }
/// ```
/// # Errors
/// When unable to send the message or when the server gives a non 2xx return code
pub fn send_discord_message(
    webhook_url: &str,
    payload: &WebhookPayload,
) -> Result<(), Box<dyn std::error::Error>> {
    const MAX_CONTENT_LENGTH: usize = 2000;

    // Check if we need to chunk the content
    if let Some(content) = &payload.content {
        if content.len() > MAX_CONTENT_LENGTH {
            // Split content into chunks
            let chunks: Vec<String> = content
                .chars()
                .collect::<Vec<char>>()
                .chunks(MAX_CONTENT_LENGTH)
                .map(|chunk| chunk.iter().collect())
                .collect();

            // Send each chunk
            for chunk in chunks {
                let mut chunk_payload = payload.clone();
                chunk_payload.content = Some(chunk);
                // Only include embeds, components, etc. in the first message
                if chunk_payload.content.as_ref().is_some_and(|c| c != content)
                {
                    chunk_payload.embeds = None;
                    chunk_payload.components = None;
                    chunk_payload.attachments = None;
                }

                send_discord_message_single(webhook_url, &chunk_payload)?;
            }
            return Ok(());
        }
    }
    send_discord_message_single(webhook_url, payload)
}
/// Same as [`send_discord_message`] but doesn't chunk messages that are too large
/// # Errors
/// When unable to send the message or when the server gives a non 2xx return code
pub fn send_discord_message_single(
    webhook_url: &str,
    payload: &WebhookPayload,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    client
        .post(webhook_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?
        .error_for_status()?;
    Ok(())
}

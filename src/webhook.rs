use crate::discord::{Profile, SnipeResult};
use crate::util::user_to_tag;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use serde_json::Value;
use serenity::model::channel::{Embed, Message};

type HttpsClient = Client<HttpsConnector<HttpConnector>>;

pub struct Webhook {
    pub url: String,
}

impl Webhook {
    pub fn new(url: String) -> Self {
        Webhook { url }
    }

    pub async fn send(
        &self,
        message: &Message,
        client: &HttpsClient,
        finder: &Profile,
        result: SnipeResult,
    ) -> Result<(), ()> {
        let payload = WebhookPayload::new(message, finder, result);
        let request = Request::builder()
            .method(Method::POST)
            .uri(&self.url)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&payload).unwrap()))
            .unwrap();

        let response_result = client.request(request).await;
        if let Ok(response) = response_result {
            if let StatusCode::NO_CONTENT = response.status() {
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

#[derive(Serialize)]
struct WebhookPayload {
    username: String,
    avatar_url: String,
    embeds: Vec<Value>,
}

impl WebhookPayload {
    fn new(message: &Message, finder: &Profile, result: SnipeResult) -> Self {
        let (title, description, color) = match result {
                SnipeResult::Success => (
                    "Yay! Claimed a Nitro!",
                    "Nitro successfully claimed!",
                    0x43B581,
                ),
                SnipeResult::FakeOrExpired => (
                    "Code was fake or expired",
                    "The code was invalid or already expired.",
                    0xF04747, 
                ),
                SnipeResult::AlreadyRedeemed => (
                    "Code was already redeemed",
                    "Someone beat us to it!",
                    0xF04747, 
                ),
                SnipeResult::RateLimited => (
                    "Rate Limited",
                    "Rate-limited by Discord.",
                    0xF04747, 
                ),
                SnipeResult::DiscordError => (
                    "Discord Error",
                    "There was an error on Discord's side.",
                    0xF04747, 
                ),
                SnipeResult::ConnectionError => (
                    "Connection Error",
                    "Failed to connect to Discord.",
                    0xF04747,
                ),
                SnipeResult::Unknown => (
                    "Unknown Response",
                    "Received an unknown response from Discord.",
                    0x000000,
                ),
        };

        let embed = Embed::fake(|create| {
            create
                .author(|a| a.icon_url(finder.face()).name(finder.to_string()))
                .title(title)
                .description(description)
                .field(
                    "Code sent by:",
                    format!(
                        "[{}](https://discord.com/users/{})",
                        user_to_tag(&message.author),
                        message.author.id
                    ),
                    false,
                )
                .field(
                    "Message:",
                    format!(
                        "[Posted here!](https://discordapp.com/channels/{}/{}/{})",
                        message
                            .guild_id
                            .map_or_else(|| "@me".to_string(), |g| g.to_string()),
                        message.channel_id,
                        message.id
                    ),
                    false,
                )
                .footer(|f| {
                    f.icon_url(WebhookPayload::get_longshot_avatar())
                            .text(format!("Longshot {}", env!("CARGO_PKG_VERSION")))
                })
                .timestamp(chrono::Local::now().to_rfc3339())
                .colour(color)
        });
        let embeds = vec![embed];
        WebhookPayload {
            embeds,
            ..Default::default()
        }
    }

        fn get_longshot_avatar() -> String {
            "https://yes.nighty.works/raw/IH8LqF.png".to_string()
    }
}

    impl Default for WebhookPayload {
        fn default() -> Self {
            WebhookPayload {
                username: "Longshot".to_string(),
                avatar_url: WebhookPayload::get_longshot_avatar(),
                embeds: Vec::new(),
            }
        }
    }

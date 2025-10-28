mod cache;
mod config;
mod discord;
mod matcher;
mod util;
mod webhook;
mod logging;

use colored::*;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use log::{error, info};
use serenity::Client as DiscordClient;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    logging::set_up_logger().expect("Failed setting up logger.");

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let config = config::try_read_config().map_err(|e| e.handle()).unwrap();
    let main_token = config.main_token();
    let main_profile = discord::get_profile_for_token(&main_token, &client)
        .await
        .map_err(|e| e.handle())
        .unwrap();

    pretty_info!(
        "Starting Nitro sniping for {}!\n",
        main_profile.to_string().underline()
    );

    let mut sniping_tokens = config.get_all_sniping_tokens();
    sniping_tokens.sort();
    sniping_tokens.dedup();

    if sniping_tokens.is_empty() {
        log_error_and_exit!("At least one token is required to start sniping...");
    }

    pretty_info!(
        "Sniping on {} account(s)! Connecting to Discord...\n",
        sniping_tokens.len()
    );

    let handler_info = Arc::new(discord::HandlerInfo::new(
        client,
        config,
        sniping_tokens.len(),
    ));

    let mut tasks = Vec::new();

    for (index, token) in sniping_tokens.iter().enumerate() {
        let discord_client_result = DiscordClient::builder(token)
            .event_handler(discord::Handler::new(handler_info.clone()))
            .await;

        if let Ok(mut discord_client) = discord_client_result {
            tasks.push(tokio::spawn(async move {
                let connection_result = discord_client.start().await;
                if connection_result.is_err() {
                    pretty_error!(
                        "Connection failed for token #{}. Check token validity.",
                        index
                    );
                }
            }));
        } else {
            pretty_error!("Failed to create Discord client for token #{}.", index,);
        }
    }

    futures::future::join_all(tasks).await;
    log_error_and_exit!("Lost all connections.");
}

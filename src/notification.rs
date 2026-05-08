use crate::config::TelegramConfig;

pub async fn send_telegram(telegram: &TelegramConfig, full_message: &str) {
    println!("Sending Telegram notification...");
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        telegram.bot_token
    );

    let client = reqwest::Client::new();
    let mut body = std::collections::HashMap::new();
    body.insert("chat_id", telegram.chat_id.clone());
    body.insert("text", full_message.to_string());
    body.insert("parse_mode", "HTML".to_string());

    let res = client.post(&url).json(&body).send().await;
    if let Err(e) = res {
        eprintln!("Failed to send Telegram notification: {}", e);
    } else {
        println!("Telegram notification sent successfully!");
    }
}

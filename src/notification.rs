use crate::config::{NotificationConfig, NtfyConfig, TelegramConfig};

pub struct TelegramNotifier {
    config: TelegramConfig,
}

impl TelegramNotifier {
    pub fn new(config: TelegramConfig) -> Self {
        Self { config }
    }

    async fn notify(&self, raw_message: &str) -> anyhow::Result<()> {
        println!("Sending Telegram notification...");
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.config.bot_token
        );

        let full_message = format!("{}\n\n{}", self.config.message_prefix, raw_message);

        let client = reqwest::Client::new();
        let mut body = std::collections::HashMap::new();
        body.insert("chat_id", self.config.chat_id.clone());
        body.insert("text", full_message);
        body.insert("parse_mode", "Markdown".to_string());

        let res = client.post(&url).json(&body).send().await;
        if let Err(e) = res {
            eprintln!("Failed to send Telegram notification: {}", e);
        } else {
            println!("Telegram notification sent successfully!");
        }

        Ok(())
    }
}

pub struct NtfyNotifier {
    config: NtfyConfig,
}

impl NtfyNotifier {
    pub fn new(config: NtfyConfig) -> Self {
        Self { config }
    }

    async fn notify(&self, raw_message: &str) -> anyhow::Result<()> {
        println!("Sending Ntfy notification...");
        let url = format!(
            "{}/{}",
            self.config.server_url.trim_end_matches('/'),
            self.config.topic
        );

        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .header("Markdown", "yes")
            .body(raw_message.to_string())
            .send()
            .await;

        if let Err(e) = res {
            eprintln!("Failed to send Ntfy notification: {}", e);
        } else {
            println!("Ntfy notification sent successfully!");
        }

        Ok(())
    }
}

pub enum Notifier {
    Telegram(TelegramNotifier),
    Ntfy(NtfyNotifier),
}

impl Notifier {
    pub async fn notify(&self, raw_message: &str) -> anyhow::Result<()> {
        match self {
            Notifier::Telegram(t) => t.notify(raw_message).await,
            Notifier::Ntfy(n) => n.notify(raw_message).await,
        }
    }
}

// Factory function to create notifiers
pub fn create_notifiers(config: NotificationConfig) -> Vec<Notifier> {
    let mut notifiers = Vec::new();

    if let Some(telegram) = config.telegram {
        println!("Telegram notifier enabled");
        notifiers.push(Notifier::Telegram(TelegramNotifier::new(telegram)));
    }

    if let Some(ntfy) = config.ntfy {
        println!("Ntfy notifier enabled");
        notifiers.push(Notifier::Ntfy(NtfyNotifier::new(ntfy)));
    }

    notifiers
}

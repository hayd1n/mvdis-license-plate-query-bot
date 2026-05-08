mod config;
mod notification;

use self::config::load_config;
use mvdis_license_plate_query::{
    options::Station,
    pool::{PoolClient, PoolClientOptions},
};

struct MatchedPlate {
    station: Station,
    plate_no: String,
    price: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config()?;

    let mut all_matched_messages = Vec::new();

    for query in config.query.iter() {
        println!(
            "Querying for {} {}...",
            query.plate_ver.as_name(),
            query.plate_type.as_name()
        );

        let options = PoolClientOptions {
            plate_ver: query.plate_ver,
            plate_type: query.plate_type,
            stations: query.stations.clone(),
            retry_times: config.client.retry_times,
            concurrency: config.client.concurrency,
            ..Default::default()
        };

        let client = PoolClient::new(options)?;

        let results = client.execute().await?;

        // Filter and collect results by match patterns from config
        let mut matched_plates = Vec::new();
        for (station, plates) in &results {
            for plate_info in plates {
                if query
                    .matchs
                    .iter()
                    .any(|pattern| plate_info.plate_no.contains(pattern))
                {
                    matched_plates.push(MatchedPlate {
                        station: station.clone(),
                        plate_no: plate_info.plate_no.clone(),
                        price: plate_info.price,
                    });
                }
            }
        }

        let total_plates = matched_plates.len();
        let total_results: usize = results.iter().map(|(_, plates)| plates.len()).sum();

        println!("Matched plates: {} / {} total", total_plates, total_results);

        for plate in &matched_plates {
            println!(
                "  - {}: {} (${})",
                plate.station.as_name(),
                plate.plate_no,
                plate.price
            );
        }

        if !matched_plates.is_empty() {
            let mut message = format!(
                "🚗 <b>{} {}</b>\n",
                query.plate_ver.as_name(),
                query.plate_type.as_name()
            );
            for plate in &matched_plates {
                message.push_str(&format!(
                    "  - {}: <b>{}</b> (${})\n",
                    plate.station.as_name(),
                    plate.plate_no,
                    plate.price
                ));
            }
            all_matched_messages.push(message);
        }
    }

    if !all_matched_messages.is_empty() {
        if let Some(telegram) = &config.notification.telegram {
            let full_message = format!(
                "{}\n\n{}",
                telegram.message_prefix,
                all_matched_messages.join("\n")
            );
            notification::send_telegram(telegram, &full_message).await;
        }
    }

    Ok(())
}

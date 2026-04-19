use base64::Engine;
use base64::engine::general_purpose;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::{Cell, Color, Table};
use core::error;
use tokio::time;

use std::path::Path;
use std::{fs, time::Duration};
use transmission_client::{Client, Torrent};
use url::{ParseError, Url};

use crate::helpers::read_download_dir_from_config;

const DOWNLOADING: i32 = 4;

pub fn decode_torrent_file(torrent_location: &Path) -> String {
    let torrent_bytes = fs::read(torrent_location)
        .map_err(|e| format!("Error reading file -> {:?}, err -> {}", torrent_location, e))
        .unwrap();

    let metainfo_base64 = general_purpose::STANDARD.encode(&torrent_bytes);

    metainfo_base64
}

pub fn initialize_torrent_client() -> Result<Client, ParseError> {
    let url = Url::parse("http://localhost:9091/transmission/rpc")?;

    let client = Client::new(url);

    client.set_authentication(None);

    Ok(client)
}

pub async fn add_torrent_download(
    client: &Client,
    meta_info: String,
) -> Result<Torrent, Box<dyn std::error::Error>> {
    let added_torrent_result = Client::torrent_add_metainfo(&client, &meta_info).await?;

    let added_torrent = match added_torrent_result {
        Some(torrent) => torrent,
        None => {
            return Err("Torrent already exists (Duplicated).".into());
        }
    };

    let torrent_ids = vec![added_torrent.hash_string.clone()];

    client
        .torrent_set_location(
            Some(torrent_ids.clone()),
            read_download_dir_from_config(),
            true,
        )
        .await?;

    start_torrent_download(client, torrent_ids).await?;
    show_single_download(&client, added_torrent.id).await?;

    Ok(added_torrent)
}

pub async fn start_torrent_download(
    client: &Client,
    torrent_id: Vec<String>,
) -> Result<(), Box<dyn error::Error>> {
    match client.torrent_start(Some(torrent_id), true).await {
        Ok(()) => Ok(()),
        Err(e) => return Err(format!("Error starting torrent download: {:?}", e).into()),
    }
}

pub async fn show_single_download(
    client: &Client,
    torrent_id: i32,
) -> Result<(), Box<dyn error::Error>> {
    'check_is_downloading: loop {
        let torrent = client.torrents(Some(vec![torrent_id])).await?;

        for t in &torrent {
            if t.status == DOWNLOADING && t.total_size != 0 {
                println!(
                    "Downloading -> {}\n file total size (MB) -> {}",
                    t.name,
                    t.total_size / 1048576
                );
                break 'check_is_downloading;
            }
        }
        time::sleep(Duration::from_secs_f32(1.5)).await;
    }
    Ok(())
}

pub async fn delete_torrent(client: &Client, torrent_id: &u8) -> Result<(), Box<dyn error::Error>> {
    match client.torrent_remove(torrent_id) {
        Ok(()) => Ok(()),
        Err(e) => return Err(format!("Error starting torrent download: {:?}", e).into()),
    }

    Ok(())
}

pub async fn show_downloads(client: &Client) -> Result<(), Box<dyn error::Error>> {
    loop {
        let torrents = client.torrents(None).await?;
        let mut has_downloading = false;

        let mut table = Table::new();
        table.load_preset(UTF8_FULL_CONDENSED);
        table.set_header(vec![
            Cell::new("ID")
                .add_attribute(comfy_table::Attribute::Bold)
                .fg(Color::DarkYellow),
            Cell::new("Nombre").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Progreso (%)").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Velocidad").add_attribute(comfy_table::Attribute::Bold),
        ]);

        for torrent in &torrents {
            // 4 : Downloading | 0 :Not yet
            if torrent.status == DOWNLOADING {
                has_downloading = true;

                let progress = format!("{:.2}%", torrent.percent_done * 100.0);
                let speed_mb = format!("{:.2} MB/s", torrent.rate_download as f64 / 1_048_576.0);

                table.add_row(vec![
                    Cell::new(&torrent.id.to_string()).fg(Color::DarkYellow),
                    Cell::new(&torrent.name),
                    Cell::new(&progress).fg(Color::DarkGreen), // Verde suave para el progreso
                    Cell::new(&speed_mb).fg(Color::DarkCyan),
                ]);
            }
        }

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        if has_downloading {
            println!("{table}");
        } else {
            break Ok(());
        }

        time::sleep(Duration::from_secs_f64(2.0)).await;
    }
}

// fn initialize_config_file() {}

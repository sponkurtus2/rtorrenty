use base64::Engine;
use base64::engine::general_purpose;
use config_file::FromConfigFile;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use transmission_client::{Client, Torrent};
use url::{ParseError, Url};

#[derive(Deserialize)]
struct Config {
    download_dir: String,
}

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

    // We will not use auth, since this is a very basic TUI.
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
        .torrent_set_location(Some(torrent_ids), read_download_dir_from_config(), true)
        .await?;

    Ok(added_torrent)
}

fn read_download_dir_from_config() -> String {
    // Use a variable to find the global config file.
    let config = Config::from_config_file("/home/sponk2/rtorrenty/config.toml").unwrap();
    println!("{}", config.download_dir);
    config.download_dir.to_string()
}

// fn initialize_config_file() {}

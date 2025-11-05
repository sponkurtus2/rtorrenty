use std::fs;
use std::path::Path;

use base64::{Engine as _, engine::general_purpose};
use url::{ParseError, Url};

use transmission_client::{Client, Torrent};

#[tokio::main]
async fn main() {
    let torrent_file_location = Path::new("../testTorrent/debian-13.1.0-amd64-netinst.iso.torrent");
    let download_dir = "~/Downloads/"; // Set def download dir in the tui

    let client = match initialize_torrent_client() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error when initializing the client -> {}", e);
            return;
        }
    };

    let decoded_torrent_file = decode_torrent_file(&torrent_file_location);

    match add_torrent_download(&client, &download_dir, decoded_torrent_file).await {
        Ok(torrent) => {
            println!("Torrent started to download!");
            println!("Torrent name: {}", torrent.name);
            println!("Torrent id: {}", torrent.id);
            println!("Torrent total size: {}", torrent.total_size);
        }
        Err(e) => {
            eprintln!("Error when starting to download torrent: {}", e);
        }
    }
}

fn decode_torrent_file(torrent_location: &Path) -> String {
    let torrent_bytes = fs::read(torrent_location)
        .map_err(|e| format!("Error reading file -> {:?}, err -> {}", torrent_location, e))
        .unwrap();

    let metainfo_base64 = general_purpose::STANDARD.encode(&torrent_bytes);

    metainfo_base64
}

fn initialize_torrent_client() -> Result<Client, ParseError> {
    let url = Url::parse("http://localhost:9091/transmission/rpc")?;

    let client = Client::new(url);

    // We will not use auth, since this is a very basic TUI.
    client.set_authentication(None);

    Ok(client)
}

async fn add_torrent_download(
    client: &Client,
    download_dir: &str,
    meta_info: String,
) -> Result<Torrent, Box<dyn std::error::Error>> {
    let added_torrent_result = Client::torrent_add_metainfo(&client, &meta_info).await?;
    println!("Sending torrent to daemon");

    let added_torrent = match added_torrent_result {
        Some(torrent) => torrent,
        None => {
            return Err("Torrent already exists (Duplicated).".into());
        }
    };
    println!("Torrent added with id: {}", added_torrent.id);

    let torrent_ids = vec![added_torrent.hash_string.clone()];

    client
        .torrent_set_location(Some(torrent_ids), download_dir.to_string(), true)
        .await?;

    println!("Torrent moved to {}", download_dir);
    Ok(added_torrent)
}

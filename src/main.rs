mod rtorrenty_cli;
mod rtorrenty_logic;

use base64::Engine as _;
use clap::Parser;
use rtorrenty_cli::Args;
use serde::Deserialize;

use config_file::FromConfigFile;

#[tokio::main]
async fn main() {
    // let args = Args::parse();
    // println!("{:?}", args.f_name)

    // let torrent_file_location = Path::new("../testTorrent/debian-13.1.0-amd64-netinst.iso.torrent");
    // let download_dir = "/home/sponk2/Downloads/"; // Set def download dir in the tui
    //
    // let client = match initialize_torrent_client() {
    //     Ok(c) => c,
    //     Err(e) => {
    //         eprintln!("Error when initializing the client -> {}", e);
    //         return;
    //     }
    // };
    //
    // let decoded_torrent_file = decode_torrent_file(&torrent_file_location);
    //
    // match add_torrent_download(&client, &download_dir, decoded_torrent_file).await {
    //     Ok(torrent) => {
    //         println!("Torrent started to download!");
    //         println!("Torrent name: {}", torrent.name);
    //     }
    //     Err(e) => {
    //         eprintln!("Error when starting to download torrent: {:?}", e);
    //     }
    // }
}

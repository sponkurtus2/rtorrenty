use clap::Parser;
use core::error;
use std::path::Path;

use crate::rtorrenty_logic::add_torrent_download;
use crate::rtorrenty_logic::decode_torrent_file;
use crate::rtorrenty_logic::initialize_torrent_client;

#[derive(Parser, Debug)]
#[command(version, author = "Carlos Reyes", about, long_about = None)]
pub struct Args {
    /// Name of the file to download
    #[arg(short, long, action)]
    pub f_name: Option<String>,

    #[arg(long, help_heading = "Config", alias = "download-folder")]
    pub download_folder: bool,

    #[arg(long, help_heading = "Config", alias = "list-downloading-files")]
    pub list_downloading_files: bool,

    #[arg(long, help_heading = "Config", alias = "delete-file")]
    pub delete_file: bool, // Maybe an ID (Torrent ID),
}

impl Args {
    #[expect(clippy::print_stdout, reason = "This is where we parse the command")]
    pub async fn execute(self) -> Result<(), Box<dyn error::Error>> {
        // Flag to download a torrent file
        if let Some(file) = &self
            .f_name
            .as_ref()
            .and_then(|f| (!f.is_empty()).then_some(f))
        {
            println!("Torrent to download: {}", file);

            let torrent_file_location: &Path = Path::new(file);
            let decoded_torrent_file = decode_torrent_file(torrent_file_location);

            let client = match initialize_torrent_client() {
                Ok(c) => c,
                Err(e) => {
                    return Err(format!("Error when initializing the client -> {}", e).into());
                }
            };

            match add_torrent_download(&client, decoded_torrent_file).await {
                Ok(_) => {
                    println!("Downloading torrent");
                }
                Err(e) => {
                    return Err(format!("Error when starting to download torrent: {:?}", e).into());
                }
            }
        }
        // Add more flags
        Ok(())
    }
}

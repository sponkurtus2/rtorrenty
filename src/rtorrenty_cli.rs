use clap::Parser;
use core::error;
use std::path::Path;

use crate::rtorrenty_logic::add_torrent_download;
use crate::rtorrenty_logic::decode_torrent_file;
use crate::rtorrenty_logic::delete_torrent;
use crate::rtorrenty_logic::initialize_torrent_client;
use crate::rtorrenty_logic::show_downloads;

#[derive(Parser, Debug)]
#[command(version, author = "Carlos Reyes", about, long_about = None)]
pub struct Args {
    /// Name of the file to download
    #[arg(short, long, action)]
    pub file_name: Option<String>,

    // [[TODO]]
    #[arg(long, help_heading = "Config", alias = "download-folder")]
    pub download_folder: bool,

    /// Enter in a loop to print a table with your current downloads
    #[arg(long, help_heading = "Config", alias = "list-downloading-files")]
    pub list_downloading_files: bool,

    /// Use some torrent ID to delete it from your downloads
    #[arg(long, help_heading = "Config", alias = "delete-file")]
    pub delete_file: Option<u8>,
}

impl Args {
    #[expect(clippy::print_stdout, reason = "This is where we parse the command")]
    pub async fn execute(self) -> Result<(), Box<dyn error::Error>> {
        // Initialize the client ONCE
        let torrent_client = match initialize_torrent_client() {
            Ok(c) => c,
            Err(e) => {
                return Err(format!("Error when initializing the client -> {}", e).into());
            }
        };

        // Flag to download a torrent file
        if let Some(file) = &self
            .file_name
            .as_ref()
            .and_then(|f| (!f.is_empty()).then_some(f))
        {
            println!("Torrent to download: {}", file);

            let torrent_file_location: &Path = Path::new(file);
            let decoded_torrent_file = decode_torrent_file(torrent_file_location);

            match add_torrent_download(&torrent_client, decoded_torrent_file).await {
                Ok(_) => {
                    println!("Downloading torrent");
                }
                Err(e) => {
                    return Err(format!("Error when starting to download torrent: {:?}", e).into());
                }
            }
        }

        // Add more flags
        if self.list_downloading_files {
            match show_downloads(&torrent_client).await {
                Ok(_) => {
                    println!("Showing downloading files.");
                }
                Err(e) => {
                    return Err(format!("Error showing downloading files -> {}", e).into());
                }
            }
        }

        if let Some(torrent_id) = &self
            .delete_file
            .as_ref()
        {
            match delete_torrent(&torrent_client, torrent_id).await {
                Ok(_) => {
                    println!("Deleting torrent...");
                }
                Err(e) => {
                    return Err(format!("Error deleting torrent -> {}", e).into());
                }
            }
        }

        Ok(())
    }
}

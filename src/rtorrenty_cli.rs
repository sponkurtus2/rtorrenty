use std::error::Error;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author = "Carlos Reyes", about, long_about = None)]
pub struct Args {
    /// Name of the file to download
    #[arg(short, long, action)]
    pub f_name: String,

    #[arg(long, help_heading = "Config", alias = "download-folder")]
    pub download_folder: String,

    #[arg(long, help_heading = "Config", alias = "list-downloading-files")]
    pub list_downloading_files: bool,

    #[arg(long, help_heading = "Config", alias = "delete-file")]
    pub delete_file: String // Maybe an ID (Torrent ID),
}

impl Args {
    #[expect(clippy::print_stdout, reason = "This is where we parse the command")]
    pub async fn parse(self) -> Result<(), Box<dyn Error>> {
        // Flag to download a torrent file
        if let Some(file) = &self
            .f_name
            .as_ref()
            .and_then(|f| (!f.is_empty()).then_some(f)){

        }
    }
}

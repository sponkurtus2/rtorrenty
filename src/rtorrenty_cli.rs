use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author = "Carlos Reyes", about, long_about = None)]
pub struct Args {
    /// Name of the file to download
    #[arg(short, long, action)]
    f_name: String,

    #[arg(long, help_heading = "Config", alias = "download-folder")]
    pub download_folder: bool,

    #[arg(long, help_heading = "Config", alias = "file-download-status")]
    pub file_download_status: bool,

    #[arg(long, help_heading = "Config", alias = "list-downloading-files")]
    pub list_downloading_files: bool,

    #[arg(long, help_heading = "Config", alias = "delete-file")]
    pub delete_file: bool,
}

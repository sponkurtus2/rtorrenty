mod rtorrenty_cli;
mod rtorrenty_logic;

use clap::Parser;
use rtorrenty_cli::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let args = Args::parse();
    args.execute().await?;

    Ok(())
}

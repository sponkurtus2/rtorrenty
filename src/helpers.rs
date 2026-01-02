use config_file::FromConfigFile;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    download_dir: String,
}
pub fn read_download_dir_from_config() -> String {
    // Use a variable to find the global config file.
    let config = Config::from_config_file("/home/sponk2/rtorrenty/config.toml").unwrap();
    println!("{}", config.download_dir);
    config.download_dir.to_string()
}

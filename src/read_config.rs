use luxlib::Config;
use std::fs::read_to_string;
use std::io;

pub fn read_config(extension: &str) -> io::Result<Config> {
    let config_str: String = read_to_string(
        format!("{}/.config/lux/basic_format/{}.toml", env!("HOME"), extension)
    )?;
    let config: Config = match toml::from_str(&config_str) {
        Ok(cfg) => cfg,
        Err(e) => return Err(
            io::Error::new(io::ErrorKind::InvalidData, format!("{}", e))
        )
    };

    Ok(config)
}

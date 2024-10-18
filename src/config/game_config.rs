use crate::config::model::game_config::Config;
use crate::resources::resource_handler::{read_file_as_string, write_file};

const CONFIG_FILE_NAME: &str = "config.toml";

pub struct GameConfig {
    pub model: Config,
}

impl GameConfig {
    pub fn new() -> Self {
        GameConfig {
            model: Config::default()
        }
    }

    pub fn load_or_create_new() -> Self {
        GameConfig::load_config().unwrap_or_else(|e| {
            println!("{}", e);
            GameConfig::new()
        })
    }

    pub fn save_config(cfg: &GameConfig) {
        let toml = match toml::to_string(&cfg.model) {
            Ok(s) => { s }
            Err(e) => {
                println!("{}", e);
                "".into()
            }
        };

        println!("Save toml: \n{toml}");

        write_file(CONFIG_FILE_NAME, &toml).unwrap_or_else(|e| println!("{}", e));
    }


    pub fn load_config() -> Result<Self, String> {
        match read_file_as_string(CONFIG_FILE_NAME) {
            Ok(s) => {
                match toml::from_str(&s) {
                    Ok(cfg) => {
                        Ok(GameConfig { model: cfg })
                    }
                    Err(e) => { Err(e.to_string()) }
                }
            }
            Err(e) => { Err(e.to_string()) }
        }
    }
}

use crate::config::model::game_config::Config;
use crate::resources::resource_handler::write_file;

const CONFIG_FILE_NAME: &str = "config.toml";

struct GameConfig {
    model: Config,
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
            Err(e) => { println!("{}", e.to_string()); "".into() }
        };

        write_file(CONFIG_FILE_NAME, &toml).unwrap_or_else(|e| println!("{}", e.to_string()));
    }


    pub fn load_config() -> Result<Self, String> {
        Err("Not implemented".to_string())
    }
}

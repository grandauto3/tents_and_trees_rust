use notan::prelude::*;

pub const ASSET_PATH: &str = "./assets";



// !here for demo purpose
// pub fn create_toml_loader() -> AssetLoader {
//     type ParserSig = fn(&str, Vec<u8>) -> Result<String, String>;
//     const TOML_PARSER: ParserSig = |_id, data| {
//         String::from_utf8(data).map_err(|e| e.to_string())
//     };
//     AssetLoader::new().use_parser(TOML_PARSER).extension("toml")
// }

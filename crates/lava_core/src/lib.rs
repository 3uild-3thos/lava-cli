pub use lava_config::{LavaConfig, LavaConfigJSON};
use wasm_bindgen::prelude::*;

pub mod lava_config;
pub mod seeds;

#[wasm_bindgen]
pub fn json_to_mocka(json: &str) -> String {
    let config: LavaConfigJSON = serde_json::from_str(json).unwrap();
    let config = LavaConfig::try_from(&config).unwrap();
    config.to_mocha()
}

#[cfg(test)]
mod tests {
    use crate::{LavaConfig, LavaConfigJSON};
    use std::fs::File;
    use std::io::{Read, Write};

    #[test]
    fn test_parse() {
        let file_name = "../../Escrow2024.json";
        let mut file = File::open(file_name).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let lava_config_json: LavaConfigJSON = serde_json::from_slice(buffer.as_bytes()).unwrap();
        let config = LavaConfig::try_from(&lava_config_json).unwrap();
        let mocha = config.to_mocha();
        let mut tests = File::create("../../test.mocha.ts").unwrap();
        tests.write_all(mocha.as_bytes()).unwrap();
        //   println!("{}", mocha);
    }
}

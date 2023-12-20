use wasm_bindgen::prelude::*;
use anyhow::{Result, Error};
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavaConfig {
    pub name: String,
    pub programs: Vec<Program>,
    pub wallets: Vec<Wallet>,
    pub tokens: Vec<Token>,
}

impl TryFrom<&str> for LavaConfig {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        let config: LavaConfig = serde_json::from_str(&value)?;
        config.check()?;
        Ok(config)
    }
}

impl LavaConfig {
    pub fn check(&self) -> Result<()> {
        // TODO: Make this actually do checks
        let y;

        // let wallets = self.wallets.clone();
        // let tokens = self.tokens.clone();

        // // Ensure wallets don't have any undefined tokens
        // for wallet in &self.wallets {
        //     for t in wallet.tokens {
        //         if !tokens.contains(&t.symbol) {
        //             return Err(Error::msg(format!("Unknown token: {}", t)));
        //         }
        //     }
        // }
    
        // // Ensure tokens don't have any undefined wallets
        // for token in &self.tokens {
        //     if let Some(a) = &token.1.authority {
        //         if a != "*" && !wallets.contains(a) {
        //             return Err(Error::msg(format!("Unknown authority: {}", a)));
        //         }
        //     }
        // }
        Ok(())
    }

    pub fn to_mocha(&self) -> String {
        // let wallets = &self.mocha_generate_wallets();
        // let airdrops = &self.mocha_generate_airdrops();
        // let tokens = &self.mocha_generate_tokens();
        let atas = ""; //&self.mocha_generate_atas();
        let mints = "";
        //  &self.mocha_generate_mints();

        format!(r#"describe("setup", async() => {{
    let instructions = [

    ];
    // Create wallet keypairs
    {}
    // Airdrop SOL
    {}
    // Create Tokens
    {}
    // Create ATAs
    {}
    // Mint tokens to ATAs
    {}
}})"#, "", "", "", "", "")
        // wallets, airdrops, tokens, atas, mints)
    }

    // fn mocha_generate_wallets(&self) -> String {
    //     let mut wallets = vec!["lava_master_wallet".to_string()];
    //     let mut keypairs = vec!["new Keypair()".to_string()];
    //     self
    //         .wallets
    //         .clone()
    //         .into_iter()
    //         .for_each(|w| {
    //             wallets.push(format!("lava_wallet_{}", &w));
    //             keypairs.push(keypairs[0].clone());
    //         });
    //     format!("const [{}] = [{}]", wallets.join(", "), keypairs.join(", "))
    // }
    

    // fn mocha_generate_airdrops(&self) -> String {
    //     let mut airdrops = vec!["anchor.getProvider().connection.requestAirdrop(lava_master_wallet.publicKey, 100_000_000_000).then(confirmTx)".to_string()];
    //     self
    //     .wallets
    //     .clone()
    //     .into_iter()
    //     .for_each(|w| {
    //         let sol_amount = match w.1.get("SOL") {
    //             Some(a) => a.clone(),
    //             None => 1_000_000_000
    //         };
    //         airdrops.push(format!("anchor.getProvider().connection.requestAirdrop(lava_wallet_{}.publicKey, {}).then(confirmTx)", &w.0, sol_amount))
    //     });
    //     format!("await Promise.all([\n{}\n])",
    //         airdrops.join(",\n")
    //     )
    // }

    // fn mocha_generate_tokens(&self) -> String {
    //     // Make a mint for each token
    //     if self.tokens.len() == 0 {
    //         return "// No tokens defined".to_string();
    //     }
    //     let mut tokens = vec![];
    //     self
    //     .tokens
    //     .clone()
    //     .into_iter()
    //     .for_each(|t| {
    //         let authority = match t.1.authority {
    //             Some(a) => format!("lava_wallet_{}", a),
    //             None => "lava_master_wallet".to_string()
    //         };
    //         let decimals = t.1.decimals;
    //         let freeze = match t.1.freeze {
    //             Some(a) => format!("lava_wallet_{}.publicKey", a),
    //             None => "null".to_string()
    //         };
    //         let ticker = t.0.clone();
    //         tokens.push(format!("const mint_{} = await createMint(connection, {}, {}.publicKey, {}, {}).then(confirmTx)", &ticker.to_ascii_lowercase(), authority, authority, freeze, decimals))
    //     });
    //     format!("await Promise.all([\n{}\n])",
    //         tokens.join(",\n")
    //     )
    // }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Wallet {
    pub name: String,
    pub address: Option<String>,
    pub sol_balance: u64,
    pub tokens: Vec<TokenBalance>
}

#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenBalance {
    pub amount: u64,
    pub symbol: String
}

#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    #[serde(rename = "mintAuthority")]
    pub mint_authority: Option<String>,
    #[serde(rename = "freezeAuthority")]
    pub freeze_authority: Option<String>,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    #[serde(rename = "type")]
    pub of_type: AccountType
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub name: String,
    #[serde(rename = "isMut")]
    pub is_mut: bool,
        #[serde(rename = "isSigner")]
    pub is_signer: bool
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountType { 
    pub fields: Vec<Field>, 
    pub kind: String 
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    #[serde(rename = "type")]
    pub of_type: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub accounts: Vec<AccountInfo>,
    pub args: Vec<Argument>,
    pub name: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub address: Option<String>,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub accounts: Vec<Account>,
    pub instructions: Vec<Instruction>,
    pub metadata: Option<Metadata>,
    pub name: String,
    pub version: String,
}

#[cfg(test)]
mod tests {
    use crate::LavaConfig;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_parse() {
        let file_name = "TBW2023.json";
        let mut file = File::open(file_name).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let config = LavaConfig::try_from(buffer.as_str()).unwrap();
        let mocha = config.to_mocha();
        println!("{}", mocha);
    }
}
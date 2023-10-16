extern crate serde;
extern crate serde_yaml;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeSet, HashSet};
use std::fs::File;
use std::io::{self, Read};
use anyhow::{Result, Error};

#[derive(Debug, Deserialize, Serialize)]
pub struct Balance {
    pub ticker: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tokens {
    pub name: Option<String>,
    pub decimals: u8,
    pub authority: Option<String>,
    pub freeze: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LavaConfig {
    pub version: String,
    pub workspace: Workspace,
    pub program: String,
    pub wallets: Wallets,
    pub tokens: TokensMap,
}

impl TryFrom<&str> for LavaConfig {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        let config: LavaConfig = serde_yaml::from_str(&value)?;
        config.check()?;
        Ok(config)
    }
}

impl LavaConfig {
    pub fn check(&self) -> Result<()> {
        let wallets = self.wallets.clone().into_keys().collect::<HashSet<String>>();
        let tokens = self.tokens.clone().into_keys().collect::<HashSet<String>>();

        // Ensure wallets don't have any undefined tokens
        for wallet in &self.wallets {
            for t in wallet.1.clone().into_keys() {
                if t != "SOL" && !tokens.contains(&t) {
                    return Err(Error::msg(format!("Unknown token: {}", t)));
                }
            }
        }
    
        // Ensure tokens don't have any undefined wallets
        for token in &self.tokens {
            if let Some(a) = &token.1.authority {
                if a != "*" && !wallets.contains(a) {
                    return Err(Error::msg(format!("Unknown authority: {}", a)));
                }
            }
        }
        Ok(())
    }

    pub fn to_mocha(&self) -> String {
        let wallets = &self.mocha_generate_wallets();
        let airdrops = &self.mocha_generate_airdrops();
        let tokens = &self.mocha_generate_tokens();
        let atas = ""; //&self.mocha_generate_atas();
        let mints = "";
        //  &self.mocha_generate_mints();

        format!(r#"
            describe("lava-generated-tests", async() => {{
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
            }})
        "#, wallets, airdrops, tokens, atas, mints)
    }

    fn mocha_generate_wallets(&self) -> String {
        let mut wallets = vec!["lava_master_wallet".to_string()];
        let mut keypairs = vec!["new Keypair()".to_string()];
        self
            .wallets
            .clone()
            .into_iter()
            .for_each(|w| {
                wallets.push(format!("lava_wallet_{}", &w.0));
                keypairs.push(keypairs[0].clone());
            });
        format!("const [{}] = [{}]", wallets.join(", "), keypairs.join(", "))
    }
    

    fn mocha_generate_airdrops(&self) -> String {
        let mut airdrops = vec!["anchor.getProvider().connection.requestAirdrop(lava_master_wallet.publicKey, 100_000_000_000).then(confirmTx)".to_string()];
        self
        .wallets
        .clone()
        .into_iter()
        .for_each(|w| {
            let sol_amount = match w.1.get("SOL") {
                Some(a) => a.clone(),
                None => 1_000_000_000
            };
            airdrops.push(format!("anchor.getProvider().connection.requestAirdrop(lava_wallet_{}.publicKey, {}).then(confirmTx)", &w.0, sol_amount))
        });
        format!("await Promise.all([\n{}\n])",
            airdrops.join(",\n")
        )
    }

    fn mocha_generate_tokens(&self) -> String {
        // Make a mint for each token
        if self.tokens.len() == 0 {
            return "// No tokens defined".to_string();
        }
        let mut tokens = vec![];
        self
        .tokens
        .clone()
        .into_iter()
        .for_each(|t| {
            let authority = match t.1.authority {
                Some(a) => format!("lava_wallet_{}", a),
                None => "lava_master_wallet".to_string()
            };
            let decimals = t.1.decimals;
            let freeze = match t.1.freeze {
                Some(a) => format!("lava_wallet_{}.publicKey", a),
                None => "null".to_string()
            };
            let ticker = t.0.clone();
            tokens.push(format!("const mint_{} = await createMint(connection, {}, {}.publicKey, {}, {}).then(confirmTx)", &ticker.to_ascii_lowercase(), authority, authority, freeze, decimals))
        });
        format!("await Promise.all([\n{}\n])",
            tokens.join(",\n")
        )
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workspace {
    pub name: String,
}

type Wallet = HashMap<String, u64>;
type Wallets = HashMap<String, Wallet>;
type TokensMap = HashMap<String, Tokens>;

pub fn declareAirdrop(wallet: String, config: &LavaConfig) -> String {
    let sol_balance = match config.wallets.get(&wallet) {
        Some(w) => {
            *w.get("SOL").unwrap_or(&1_000_000_000)
        },
        None => 1000000000
    };
    format!("anchor.getProvider().connection.requestAirdrop({}.publicKey, {}).then(confirmTx)", wallet, sol_balance)
}

#[cfg(test)]
mod tests {
    use crate::parser::LavaConfig;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_parse() {
        let file_name = "lava.yaml";
        let mut file = File::open(file_name).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let config = LavaConfig::try_from(buffer.as_str()).unwrap();
        let mocha = config.to_mocha();
        println!("{}", mocha);
    }
}
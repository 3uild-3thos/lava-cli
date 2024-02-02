use std::collections::HashMap;

use serde_json::Value;
use wasm_bindgen::prelude::*;
use anyhow::{Result, Error};
use serde::{Serialize, Deserialize};
use convert_case::{Case, Casing};

use crate::seeds::LavaSeed;

pub mod seeds;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavaConfigJSON {
    name: String,
    accounts: Vec<Value>,
    tests: Vec<LavaTest>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavaTest {
    name: String,
    programId: String,
    instruction: String,
    accounts: Value,
    args: Vec<Value>
}



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavaProgram {
    name: String,
    #[serde(default = "anchor_program")]
    address: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavaWallet {
    name: String,
    balance: u64
}

impl LavaWallet {
    fn to_mocha_account(&self) -> String {
        format!("const {} = new Keypair();", self.name.to_case(Case::Snake))
    }
    fn to_key_value(&self) -> String {
        format!("{}: {}.publicKey", self.name.to_case(Case::Snake), self.name.to_case(Case::Snake))
    }
}

impl Default for LavaWallet {
    fn default() -> Self {
        LavaWallet {
            name: "anchorProvider".to_string(),
            balance: u64::MAX
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavaMint {
    name: String,
    symbol: String,
    decimals: u8,
    #[serde(default = "anchor_provider")]
    mint_authority: String,
    freeze_authority: Option<LavaWallet>
}

impl LavaMint {
    fn to_mocha_account(&self) -> String {
        format!("const {} = new Keypair();", self.name.to_case(Case::Snake))
    }
    fn to_key_value(&self) -> String {
        format!("{}: {}.publicKey", self.name.to_case(Case::Snake), self.name.to_case(Case::Snake))
    }
}

fn anchor_provider() -> String {
    "provider".to_string()
}

fn anchor_program() -> String {
    "program".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavaATA {
    name: String,
    authority: String,
    mint: String,
    amount: u64
}

impl LavaATA {
    fn to_mocha_account(&self, pda_owner: bool) -> String {
        if pda_owner {
            format!(
                "const {} = getAssociatedTokenAddressSync({}.publicKey, {});", 
                self.name.to_case(Case::Snake),
                self.mint.to_case(Case::Snake),
                self.authority.to_case(Case::Snake)
            )
        } else {
            format!(
                "const {} = getAssociatedTokenAddressSync({}.publicKey, {}.publicKey);", 
                self.name.to_case(Case::Snake),
                self.mint.to_case(Case::Snake),
                self.authority.to_case(Case::Snake)
            )
        }
    }

    fn to_key_value(&self) -> String {
        self.name.to_case(Case::Snake)
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavaPDA {
    name: String,
    program: String,
    seeds: Vec<LavaSeed>
}

#[derive(Deserialize)]
struct LavaSeedJSON {
    kind: String,
    value: Value
}

#[derive(Deserialize)]
pub struct LavaPDAJSON {
    name: String,
    program: String,
    seeds: Vec<LavaSeedJSON>
}

impl LavaPDA {
    pub fn from_json(v: &[u8]) -> Result<Self, Error> {
        let lss: LavaPDAJSON = serde_json::from_slice(v).map_err(|_| Error::msg("Invalid PDA schema"))?;
        let seeds = vec![];
        lss.seeds.iter().map(|s| {
            match s.kind.as_str() {
                "u8" => Ok(LavaSeed::U8(s.value.as_u64().ok_or(Error::msg("Invalid u8"))? as u8)),
                "i8" => Ok(LavaSeed::I8(s.value.as_i64().ok_or(Error::msg("Invalid i8"))? as i8)),
                "u16" => Ok(LavaSeed::U16(s.value.as_u64().ok_or(Error::msg("Invalid u16"))? as u16)),
                "i16" => Ok(LavaSeed::I16(s.value.as_i64().ok_or(Error::msg("Invalid i16"))? as i16)),
                "u32" => Ok(LavaSeed::U32(s.value.as_u64().ok_or(Error::msg("Invalid u32"))? as u32)),
                "i32" => Ok(LavaSeed::I32(s.value.as_i64().ok_or(Error::msg("Invalid i32"))? as i32)),
                "u64" => Ok(LavaSeed::U64(s.value.as_u64().ok_or(Error::msg("Invalid u64"))?)),
                "i64" => Ok(LavaSeed::I64(s.value.as_i64().ok_or(Error::msg("Invalid i64"))?)),
                "String" => Ok(LavaSeed::String(s.value.as_str().ok_or(Error::msg("Invalid String"))?.to_string())),
                "Pubkey" => Ok(LavaSeed::PublicKey(s.value.as_str().ok_or(Error::msg("Invalid Public Key"))?.to_string())),
                _ => return Err(Error::msg("Unsupported PDA seed type"))
            }
        }).collect::<Result<Vec<LavaSeed>, Error>>()?;

        Ok(LavaPDA {
            name: lss.name,
            program: lss.program,
            seeds
        })
    }
    
    pub fn to_mocha_account(&self, wallets: &HashMap<String, LavaWallet>) -> String {
        format!(
            "const {} = PublicKey.findProgramAddressSync([{}], {})[0]", 
            self.name.to_case(Case::Snake),
            self.seeds.iter().map(|s| {
                if let LavaSeed::PublicKey(p) = s {
                    if !wallets.contains_key(p) {
                        return s.to_mocha_account(true)
                    }
                }
                s.to_mocha_account(false)
            }  ).collect::<Vec<String>>().join(", "),
            self.program.to_case(Case::Snake)
        )
    }

    fn to_key_value(&self) -> String {
        self.name.to_case(Case::Snake)
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavaConfig {
    name: String,
    wallets: HashMap<String, LavaWallet>,
    mints: HashMap<String, LavaMint>,
    atas: HashMap<String, LavaATA>,
    programs: HashMap<String, LavaProgram>,
    pdas: HashMap<String, LavaPDA>,
    tests: Vec<LavaTest>
}

impl TryFrom<&str> for LavaConfig {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        let config: LavaConfig = serde_json::from_str(&value)?;
        config.check()?;
        Ok(config)
    }
}

impl TryFrom<&LavaConfigJSON> for LavaConfig {
    type Error = Error;

    fn try_from(value: &LavaConfigJSON) -> Result<Self, Error> {
        let mut wallets = HashMap::new();
        let mut programs = HashMap::new();
        let mut pdas = HashMap::new();
        let mut mints = HashMap::new();
        let mut atas = HashMap::new();

        for v in &value.accounts {
            let name = v.get("name").and_then(|n| n.as_str()).ok_or(Error::msg("Account name missing"))?;

            match v.get("kind").and_then(|k| k.as_str()).ok_or(Error::msg("Account kind missing"))? {
                "program" => {
                    let program: LavaProgram = serde_json::from_value(v.clone())?;
                    programs.insert(name.to_string(), program);
                },
                "wallet" => {
                    let wallet: LavaWallet = serde_json::from_value(v.clone())?;
                    wallets.insert(name.to_string(), wallet);
                },
                "mint" => {
                    let mint: LavaMint = serde_json::from_value(v.clone())?;
                    mints.insert(name.to_string(), mint);
                },
                "pda" => {
                    let pda = LavaPDA::from_json(v.to_string().as_bytes())?;
                    pdas.insert(name.to_string(), pda);
                },
                "ata" => {
                    let ata: LavaATA = serde_json::from_value(v.clone())?;
                    atas.insert(name.to_string(), ata);
                },
                _ => {
                    println!("Not implemented: {}", v.get("kind").unwrap().to_string());
                }
            }
        }
        let tests = value.tests.clone();
        Ok(LavaConfig {
            name: value.name.clone(),
            wallets,
            mints,
            atas,
            programs,
            pdas,
            tests
        })
    }
}

#[wasm_bindgen]
impl LavaConfig {
    fn check(&self) -> Result<()> {
        // TODO: Make this actually check our Schema for problems
        Ok(())
    }

    #[wasm_bindgen]
    pub fn to_mocha(&self) -> String {
        let mut accounts: Vec<String> = vec![];
        let accounts_declarations: String = vec![
            // TODO: Add setup for multiple programs
            // self.programs.iter().map(|(_,p)| p.to_mocha_account()).collect::<Vec<String>>().join("\n"),
            self.wallets.iter().map(|(_,w)| {
                accounts.push(w.to_key_value());
                return w.to_mocha_account()}
            ).collect::<Vec<String>>().join("\n"),
            self.mints.iter().map(|(_,m)| {
                accounts.push(m.to_key_value());
                return m.to_mocha_account()}).collect::<Vec<String>>().join("\n"),
            self.pdas.iter().map(|(_,p)| {
                accounts.push(p.to_key_value());
                return p.to_mocha_account(&self.wallets)}).collect::<Vec<String>>().join("\n"),
            self.atas.iter().map(|(_,a)| {
                accounts.push(a.to_key_value());
                if !self.wallets.contains_key(&a.authority) {
                    a.to_mocha_account(true)
                } else {
                    a.to_mocha_account(false)
                }
            }).collect::<Vec<String>>().join("\n"),
            format!("const accounts = {{\n{}\n}}", accounts.join(",\n"))
        
        ].join("\n");

        let user_defined_tests = self.tests.iter().map(|t| {
            format!(r#"it("{}", async() => {{
                await program.methods
                .{}()
                .accounts({{ ...accounts }})
                .signers([])
                .rpc()
                .then(confirm)
                .then(log);
            }});"#, t.name, t.instruction)
        }).collect::<Vec<String>>().join("\n");

        let setup = "";

        format!(r#"
describe("{}", () => {{
    anchor.setProvider(anchor.AnchorProvider.env());

    const provider = anchor.getProvider();

    const connection = provider.connection;

    const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;

    const confirm = async (signature: string): Promise<string> => {{
        const block = await connection.getLatestBlockhash();
        await connection.confirmTransaction({{
        signature,
        ...block,
        }});
        return signature;
    }};

    const log = async (signature: string): Promise<string> => {{
        console.log(
        `Your transaction signature: https://explorer.solana.com/transaction/${{signature}}?cluster=custom&customUrl=${{connection.rpcEndpoint}}`
        );
        return signature;
    }};

    // Accounts
    {}

    it("setup", async() => {{
        let instructions = [
            {}
        ];
    }})

    {}
}})"#, self.name, accounts_declarations, setup, user_defined_tests)
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
    name: String,
    address: Option<String>,
    sol_balance: u64,
    tokens: Vec<TokenBalance>
}

#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenBalance {
    amount: u64,
    symbol: String
}

#[wasm_bindgen]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
    name: String,
    symbol: String,
    decimals: u8,
    #[serde(rename = "mintAuthority")]
    mint_authority: Option<String>,
    #[serde(rename = "freezeAuthority")]
    freeze_authority: Option<String>,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    name: String,
    #[serde(rename = "type")]
    field_type: String
}

// #[wasm_bindgen]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct LavaAccount {
//     name: String,
//     kind: String,
//     initialize: Option<bool>,
//     balance: Option<u64>,
//     amount: Option<u64>,
//     mint: Option<String>,
//     mint_authority: Option<String>,
//     freeze_authority: Option<String>,
//     symbol: Option<String>,
//     decimals: Option<u8>
// }

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    name: String,
    #[serde(rename = "isMut")]
    is_mut: bool,
    #[serde(rename = "isSigner")]
    is_signer: bool
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountType { 
    fields: Vec<Field>, 
    kind: String 
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argument {
    name: String,
    #[serde(rename = "type")]
    of_type: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    accounts: Vec<AccountInfo>,
    args: Vec<Argument>,
    name: String,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    address: Option<String>,
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
        tests.write_all(&mocha.as_bytes()).unwrap();
        println!("{}", mocha);
    }
}
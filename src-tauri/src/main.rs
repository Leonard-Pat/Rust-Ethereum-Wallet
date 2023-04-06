// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs::File, io::BufWriter};

use cocoon::Cocoon;
use serde::Serialize;

mod ethereum;

pub const WALLET_FILE_PATH: &'static str = "../../wallets";

#[derive(Serialize)]
struct Test {
    pub account_name: String,
}
fn main() {
    let mut file = std::fs::File::create("data.json").expect("create failed");

    // Finally, you want to store your data secretly.
    // Supply some password to Cocoon: it can be any byte array, basically.
    // Don't use a hard-coded password in real life!
    // It could be a user-supplied password.
    let cocoon = Cocoon::new(b"hello");

    let thingy = serde_json::to_vec(&Test {
        account_name: String::from("this data"),
    })
    .unwrap();
    // Dump the serialized database into a file as an encrypted container.
    let container = cocoon.dump(thingy, &mut file).unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

////////////////////////////////////////////////////
//                 tauri commands                //
///////////////////////////////////////////////////

// #[tauri::command]
// async fn create_wallet() -> Wallet {
//     // let (secret_key, pub_key) = address::create_keypair();
//     // let crypto_wallet = Wallet::new(&secret_key, &pub_key);
//     // crypto_wallet.save_to_file("wallet.json");
//     // crypto_wallet
// }

// #[tauri::command]
// async fn get_balance() -> f64 {
//     dotenv::dotenv().ok();
//     let wallet = Wallet::from_file("wallet.json").unwrap();
//     let endpoint = env::var("ALCHEMY_KEY").unwrap();
//     let web3_con = connect::establish_web3_connection(&endpoint).await.unwrap();
//     let balance = wallet.get_balance(&web3_con).await.unwrap();
//     balance
// }

mod zz_build_cfg;
use monero_lws_client::wallet::create_vault;

fn main() {
    zz_build_cfg::ensure();
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help") {
        eprintln!("monero-lws-client — XMR wallet");
        eprintln!("usage: monero-lws-client [name]");
        return;
    }
    let name = args.get(1).map(String::as_str).unwrap_or("default");
    let vault = create_vault(name, "demo");
    println!("{} {}", vault.id, vault.accounts[0].address);
}

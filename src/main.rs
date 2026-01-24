mod cmd;
mod crypto;
mod format;
mod model;
mod storage;
mod ui;
mod vault;

use cmd::Command;

fn main() {
    let result = cmd::parse_command().and_then(run);

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn cmd_get(key: String) -> Result<(), String> {
    let (data, _, _) = vault::load("Vault password: ")?;

    let entry = data.get(&key).ok_or_else(|| "Key not found".to_string())?;

    // Print value once
    println!("{}", entry.value);
    Ok(())
}

fn run(cmd: Command) -> Result<(), String> {
    match cmd {
        Command::Init => cmd_init(),
        Command::List => cmd_list(),

        // placeholders for next milestone:
        Command::Add { key } => cmd_add(key),
        Command::Get { key } => cmd_get(key),
        Command::Remove { key } => cmd_remove(key),
        Command::ChangePassword => cmd_change_password(),
    }
}

fn cmd_list() -> Result<(), String> {
    let (data, _, _) = vault::load("Vault password: ")?;

    if data.is_empty() {
        println!("(vault is empty)");
        return Ok(());
    }

    for key in data.keys() {
        println!("{}", key);
    }

    Ok(())
}

fn cmd_add(key: String) -> Result<(), String> {
    let (mut data, _, _) = vault::load("Vault password: ")?;

    if data.contains_key(&key) {
        return Err("Key already exists".into());
    }

    let value = ui::prompt_password("Secret value: ")?;
    let ts = time::OffsetDateTime::now_utc().unix_timestamp() as u64;

    data.insert(
        key,
        model::SecretEntry {
            value,
            created_at: ts,
        },
    );

    let password = ui::prompt_password("Confirm vault password: ")?;
    vault::save(&password, &data)?;

    println!("Secret added.");
    Ok(())
}
fn cmd_remove(key: String) -> Result<(), String> {
    let (mut data, _, _) = vault::load("Vault password: ")?;

    if data.remove(&key).is_none() {
        return Err("Key not found".into());
    }

    let password = ui::prompt_password("Confirm vault password: ")?;
    vault::save(&password, &data)?;

    println!("Secret removed.");
    Ok(())
}

fn cmd_init() -> Result<(), String> {
    let path = storage::vault_path()?;

    if storage::exists(&path) {
        return Err(format!("Vault already exists at {:?}", path));
    }

    let pw1 = ui::prompt_password("Set vault password: ")?;
    let pw2 = ui::prompt_password("Confirm password: ")?;

    if pw1 != pw2 {
        return Err("Passwords do not match".into());
    }
    if pw1.len() < 10 {
        eprintln!("Warning: consider using 10+ characters.");
    }

    // For now, plaintext is an empty JSON object.
    // Next milestone: this becomes JSON of secrets.
    let plaintext = br#"{}"#;

    let (salt, nonce, ciphertext) = crypto::encrypt(&pw1, plaintext)?;
    let bytes = format::encode(&salt, &nonce, &ciphertext)?;

    storage::write_atomic(&path, &bytes)?;
    println!("Vault initialized at {:?}", path);

    Ok(())
}

fn cmd_change_password() -> Result<(), String> {
    let (data, _, _) = vault::load("Current vault password: ")?;

    let pw1 = ui::prompt_password("Set new vault password: ")?;
    let pw2 = ui::prompt_password("Confirm password: ")?;

    if pw1 != pw2 {
        return Err("Passwords do not match".into());
    }
    if pw1.len() < 10 {
        eprintln!("Warning: consider using 10+ characters.");
    }

    vault::save(&pw1, &data)?;

    println!("Vault password updated.");
    Ok(())
}

use algorand_rs::account::Account;
use algorand_rs::models::Ed25519PublicKey;
use algorand_rs::transaction::{BaseTransaction, Payment, Transaction, TransactionType};
use algorand_rs::{Address, HashDigest, Kmd, MasterDerivationKey, MicroAlgos, Round};
use dotenv::dotenv;
use rand::{distributions::Alphanumeric, Rng};
use std::env;
use std::error::Error; // 0.8

#[test]
fn test_versions_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let versions = kmd.versions();
    println!("{:#?}", versions);
    assert!(versions.is_ok());

    Ok(())
}

#[test]
fn test_list_wallets_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallets = kmd.list_wallets();
    println!("{:#?}", wallets);
    assert!(wallets.is_ok());

    Ok(())
}

#[test]
fn test_create_wallet_and_obtain_handle() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    Ok(())
}

#[test]
fn test_release_wallet_handle_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let res = kmd.release_wallet_handle(handle.unwrap().wallet_handle_token.as_ref());

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_renew_wallet_handle_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let res = kmd.renew_wallet_handle(handle.unwrap().wallet_handle_token.as_ref());

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_rename_wallet_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let new_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let res = kmd.rename_wallet(
        wallet.unwrap().wallet.id.as_ref(),
        "testpassword",
        new_name.as_ref(),
    );

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_get_wallet_info_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let res = kmd.get_wallet_info(handle.unwrap().wallet_handle_token.as_ref());

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_export_wallet_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let res = kmd
        .export_master_derivation_key(handle.unwrap().wallet_handle_token.as_ref(), "testpassword");

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_import_export_key() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let handle = handle.unwrap();

    let key = kmd.import_key(handle.wallet_handle_token.as_ref(), [0; 32]);

    println!("{:#?}", key);
    assert!(key.is_ok());

    let key = key.unwrap();

    let res = kmd.export_key(
        handle.wallet_handle_token.as_ref(),
        "testpassword",
        key.address.as_ref(),
    );

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_generate_key_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let res = kmd.generate_key(handle.unwrap().wallet_handle_token.as_ref());

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_delete_key_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let handle = handle.unwrap();

    let key = kmd.generate_key(handle.wallet_handle_token.as_ref());

    println!("{:#?}", key);
    assert!(key.is_ok());

    let key = key.unwrap();

    let res = kmd.delete_key(
        handle.wallet_handle_token.as_ref(),
        "testpassword",
        key.address.as_ref(),
    );

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_list_keys_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let handle = handle.unwrap();

    let res = kmd.generate_key(handle.wallet_handle_token.as_ref());

    println!("{:#?}", res);
    assert!(res.is_ok());

    let res = kmd.list_keys(handle.wallet_handle_token.as_ref());

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_list_keys_of_empty_wallet() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let res = kmd.list_keys(handle.unwrap().wallet_handle_token.as_ref());

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_sign_transaction_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let account = Account::generate();

    let fee = MicroAlgos(1000);
    let amount = MicroAlgos(200000);
    let first_round = Round(12_326_444);
    let last_round = first_round + 1000;

    let base = BaseTransaction {
        sender: account.address(),
        first_valid: first_round,
        last_valid: last_round,
        note: Vec::new(),
        genesis_id: "".to_string(),
        genesis_hash: HashDigest([0; 32]),
    };

    let payment = Payment {
        amount,
        receiver: Address::from_string(
            "4MYUHDWHWXAKA5KA7U5PEN646VYUANBFXVJNONBK3TIMHEMWMD4UBOJBI4",
        )?,
        close_remainder_to: None,
    };

    let transaction = Transaction::new(base, fee, TransactionType::Payment(payment));

    println!("{:#?}", transaction);
    assert!(transaction.is_ok());

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );

    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let res = kmd.sign_transaction(
        handle.unwrap().wallet_handle_token.as_ref(),
        "testpassword",
        &transaction.unwrap(),
    );

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_list_multisig_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let res = kmd.list_multisig(handle.unwrap().wallet_handle_token.as_ref());

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_import_export_multisig() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let version = 1;
    let threshold = 1;
    let pks = [Ed25519PublicKey([0; 32]), Ed25519PublicKey([1; 32])];

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let handle = handle.unwrap();

    let multisig = kmd.import_multisig(
        handle.wallet_handle_token.as_ref(),
        version,
        threshold,
        &pks,
    );

    println!("{:#?}", multisig);
    assert!(multisig.is_ok());

    let res = kmd.export_multisig(
        handle.wallet_handle_token.as_ref(),
        multisig.unwrap().address.as_ref(),
    );

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_delete_multisig_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let version = 1;
    let threshold = 1;
    let pks = [Ed25519PublicKey([0; 32]), Ed25519PublicKey([1; 32])];

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let handle = handle.unwrap();

    let multisig = kmd.import_multisig(
        handle.wallet_handle_token.as_ref(),
        version,
        threshold,
        &pks,
    );

    println!("{:#?}", multisig);
    assert!(multisig.is_ok());

    let res = kmd.delete_multisig(
        handle.wallet_handle_token.as_ref(),
        "testpassword",
        multisig.unwrap().address.as_ref(),
    );

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

#[test]
fn test_sign_multisig_transaction_endpoint() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let kmd = Kmd::new()
        .bind(env::var("KMD_URL")?.as_ref())
        .auth(env::var("KMD_TOKEN")?.as_ref())
        .client_v1()?;

    let account = Account::generate();

    let fee = MicroAlgos(1000);
    let amount = MicroAlgos(20000);
    let first_round = Round(642_715);
    let last_round = first_round + 1000;

    let base = BaseTransaction {
        sender: account.address(),
        first_valid: first_round,
        last_valid: last_round,
        note: Vec::new(),
        genesis_id: "".to_string(),
        genesis_hash: HashDigest([0; 32]),
    };

    let payment = Payment {
        amount,
        receiver: Address::from_string(
            "4MYUHDWHWXAKA5KA7U5PEN646VYUANBFXVJNONBK3TIMHEMWMD4UBOJBI4",
        )?,
        close_remainder_to: None,
    };

    let transaction = Transaction::new(base, fee, TransactionType::Payment(payment))?;
    let pk = Ed25519PublicKey([0; 32]);
    let partial_multisig = None;

    let wallet_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let wallet = kmd.create_wallet(
        wallet_name.as_ref(),
        "testpassword",
        "sqlite",
        MasterDerivationKey([0; 32]),
    );
    println!("{:#?}", wallet);
    assert!(wallet.is_ok());

    let wallet = wallet.unwrap();

    let id = wallet.wallet.id.as_ref();
    let handle = kmd.init_wallet_handle(id, "testpassword");

    println!("{:#?}", handle);
    assert!(handle.is_ok());

    let res = kmd.sign_multisig_transaction(
        handle.unwrap().wallet_handle_token.as_ref(),
        "testpassword",
        &transaction,
        pk,
        partial_multisig,
    );

    println!("{:#?}", res);
    assert!(res.is_ok());

    Ok(())
}

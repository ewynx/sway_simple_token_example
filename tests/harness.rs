use fuels::prelude::*;
use fuels_abigen_macro::abigen;
use fuels::signers::wallet::Wallet;

abigen!(NewToken, "out/debug/sway_simple_token_example-abi.json");

async fn get_contract_instance() -> (NewToken, Wallet) {
    let num_wallets = 4;
    let coins_per_wallet = 1;
    let amount_per_coin = 1_000_000;

    let config = WalletsConfig::new(
        Some(num_wallets),
        Some(coins_per_wallet),
        Some(amount_per_coin),
    );
    let mut wallets = launch_provider_and_get_wallets(config).await;
    let deployer_wallet = wallets.pop().unwrap();
    let contract_id = Contract::deploy("./out/debug/sway_simple_token_example.bin", &deployer_wallet, TxParameters::default())
    .await.unwrap();

    let instance = NewToken::new(contract_id.to_string(), deployer_wallet);
    let other_wallet1 = wallets.pop().unwrap();

    (instance, other_wallet1)
}

#[tokio::test]
async fn can_mint_as_minter() {

    let (provider, _address) = setup_test_provider(vec![],  Config::local_node()).await;
    let phrase = "oblige salon price punch saddle immune slogan rare snap desert retire surprise";
    let _wallet = LocalWallet::new_from_mnemonic_phrase(phrase, Some(provider)).unwrap();
    // This is equal to the minter's address
    // let expected_address = "f18b6446deb8135544ba60333e5b7522685cd2cf64aa4e4c75df725149850b65";

    let contract_id = Contract::deploy("./out/debug/sway_simple_token_example.bin", &_wallet, TxParameters::default()).await.unwrap();
    let my_address = _wallet.address();
    let _instance = NewToken::new(contract_id.to_string(), _wallet);

    let amount = 45;
    //TODO Don't know how to make the call FROM the minter's address (same as deploy address here)
    let res = _instance.mint(my_address, amount).call().await;

    match res {
        Ok(_) => {
            let new_balance = _instance.balance_for_address(my_address).call().await.unwrap();
            assert_eq!(new_balance.value, amount)
        },
        Err(error) => panic!("Problem calling mint function: {:?}", error),
    }

}

#[tokio::test]
async fn cannot_mint_as_other() {
    let (_instance, _wallet) = get_contract_instance().await;
    let my_address = _wallet.address();
    // TODO this print is not working -anymore- somehow
    println!("my address {}", my_address);
    let amount = 42;
    let res = _instance.mint(my_address, amount).call().await;
    // This should error because I'm not the minter
    assert!(res.is_err());
}

#[tokio::test]
async fn can_send_amount() {
    let (_instance, _receiver) = get_contract_instance().await;
    let begin_balance_receiver = _instance.balance_for_address(_receiver.address()).call().await.unwrap().value;
    // TODO this print is not working....
    println!("begin balance is {}", begin_balance_receiver.to_string());
    let amount = 37;
    _instance.send(_receiver.address(), amount).call().await.unwrap().value;
    let end_balance_receiver = _instance.balance_for_address(_receiver.address()).call().await.unwrap().value;
    // TODO This fails because the sender has no funds.... (only works if you comment out lines 64&65 in the contract)
    assert_eq!(end_balance_receiver, begin_balance_receiver + amount)
}
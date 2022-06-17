contract;

use std::{
    address::Address,
    assert::*,
    chain::auth::{AuthError, msg_sender},
    hash::sha256,
    identity::Identity,
    logging::log,
    result::*,
    revert::revert,
    storage::StorageMap,
};


struct Sent {
    from: Address,
    to: Address,
    amount: u64,
}

abi NewToken {
    #[storage(read)]fn balance_for_address(address: Address) -> u64;
    #[storage(read, write)]fn mint(receiver: Address, amount: u64);
    #[storage(read, write)]fn send(receiver: Address, amount: u64);
}

const MINTER: b256 = 0xf18b6446deb8135544ba60333e5b7522685cd2cf64aa4e4c75df725149850b65;

storage {
    balances: StorageMap<Address, u64>
}

impl NewToken for Contract {
    #[storage(read)]fn balance_for_address(address: Address) -> u64 {
        storage.balances.get(address)
    }

    #[storage(read, write)]fn mint(receiver: Address, amount: u64) {
        let sender: Result<Identity, AuthError> = msg_sender();
        //why don't I see this log in the console?
        // log("NewToken my_address {}", amount);

        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                assert(addr == ~Address::from(MINTER));
                addr
            }
            _ => revert(0),
        };
        storage.balances.insert(receiver, storage.balances.get(receiver) + amount);
    }

    #[storage(read, write)]fn send(receiver: Address, amount: u64) {
        let sender: Result<Identity, AuthError> = msg_sender();
        let sender: Address = match sender.unwrap() {
            Identity::Address(addr) => {
                addr
            }
            _ => revert(0),
        };
        // Reduce the balance of sender
        let sender_amount = storage.balances.get(sender);
        assert(sender_amount > amount);
        storage.balances.insert(sender, sender_amount - amount);

        // Increase the balance of receiver
        storage.balances.insert(receiver, storage.balances.get(receiver) + amount);

        log(Sent {
            from: sender, to: receiver, amount: amount
        });
    }
}
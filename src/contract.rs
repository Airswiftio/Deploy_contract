//! This contract demonstrates a sample implementation of the Soroban token
//! interface.
use crate::admin::{has_administrator, read_administrator, write_administrator};
use soroban_sdk::{contract, contractimpl, Address,  Env,  Bytes, BytesN,};



pub trait DeployerTrait {
    fn initialize(e: Env, admin: Address);
    fn DeployContract(e: Env, token_wasm_hash: BytesN<32>, deployer: Address, salt: BytesN<32>)-> Address;

}


#[contract]
pub struct Deployer;

#[contractimpl]
impl DeployerTrait for Deployer {
    fn initialize(e: Env, admin: Address) {
        
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
    }

    fn DeployContract(e: Env, token_wasm_hash: BytesN<32>, deployer: Address, salt: BytesN<32> )-> Address {
        
        let admin = read_administrator(&e);
        admin.require_auth();

        let deployed_address = e
        .deployer()
        .with_address(deployer, salt)
        .deploy(token_wasm_hash);
        
        (deployed_address)
    }

}

// Find all our documentation at https://docs.near.org
use near_sdk::{near, Gas, NearToken};

mod deploy;

const NEAR_PER_STORAGE: NearToken = NearToken::from_yoctonear(10u128.pow(19)); // 10e18yⓃ

const DONATION_DEFAULT_CONTRACT: &[u8] = include_bytes!(env!("BUILD_RS_SUB_BUILD_ARTIFACT_1"));
const TGAS: Gas = Gas::from_tgas(1); // 10e12yⓃ
const NO_DEPOSIT: NearToken = NearToken::from_near(0); // 0yⓃ

// Define the contract structure
#[near(contract_state)]
pub struct Contract { }

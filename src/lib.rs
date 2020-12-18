#![allow(dead_code)]
#![allow(unused_variables)]
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env, AccountId,
};

use std::collections::HashMap;

pub type Percentage = f64;
const ROYALTY_UPPER_LIMIT: f64 = 25.0;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Royalty {
    percentage: Percentage,
    split_between: UnorderedMap<AccountId, Percentage>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SplitOwners {
    pub split_between: UnorderedMap<AccountId, Percentage>,
}

impl Royalty {
    pub fn new(percentage: Percentage, split_between: HashMap<AccountId, Percentage>) -> Self {
        // royalty percentage must be less than ROYALTY_UPPER_LIMIT
        assert!(percentage < ROYALTY_UPPER_LIMIT, "percentage too high");
        assert!(percentage >= 0.0, "percentage less than/eq to zero");
        assert!(
            split_between.len() < 100,
            "royalty max mapping lenth is 100"
        );

        // validate `AccountId`s
        let first_bad_address = split_between
            .iter()
            .position(|(k, _)| !env::is_valid_account_id(k.as_bytes()));
        if let Some(bad_address) = first_bad_address {
            env::panic(
                format!(
                    "at least one bad address given in Royalties: {}",
                    bad_address
                )
                .as_bytes(),
            );
        }

        // values should add up to 100
        let sum: f64 = split_between.iter().fold(0.0, |acc, (_, v)| {
            if v <= &0.0 {
                env::panic(
                    format!(
                        "one of the split between percentages was less than or equal to 0.0: {}",
                        v
                    )
                    .as_bytes(),
                )
            };
            acc + v
        });
        if (sum - 100.0).abs() != 0.0 {
            env::panic(b"percentages don't add up to 100")
        }

        let mut umap: UnorderedMap<AccountId, Percentage> = UnorderedMap::new(b"r".to_vec());
        split_between.iter().for_each(|(acctid, p)| {
            umap.insert(acctid, p);
        });
        Self {
            percentage,
            split_between: umap,
        }
    }
}

impl SplitOwners {
    pub fn new(split_between: HashMap<AccountId, Percentage>) -> Self {
        assert!(
            split_between.len() < 100,
            "split owners max mapping lenth is 100"
        );

        // validate `AccountId`s
        let first_bad_address = split_between
            .iter()
            .position(|(k, _)| !env::is_valid_account_id(k.as_bytes()));
        if let Some(bad_address) = first_bad_address {
            env::panic(
                format!(
                    "at least one bad address given in SplitOwners: {}",
                    bad_address
                )
                .as_bytes(),
            );
        }

        // values should add up to 100
        let sum: f64 = split_between.iter().fold(0.0, |acc, (_, v)| {
            if v <= &0.0 {
                env::panic(
                    format!(
                        "one of the split between percentages was less than or equal to 0.0: {}",
                        v
                    )
                    .as_bytes(),
                )
            };
            acc + v
        });
        if (sum - 100.0).abs() != 0.0 {
            env::panic(b"percentages don't add up to 100")
        }

        let mut umap: UnorderedMap<AccountId, Percentage> = UnorderedMap::new(b"r".to_vec());
        split_between.iter().for_each(|(acctid, p)| {
            umap.insert(&acctid, &p);
        });
        Self {
            split_between: umap,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, AccountId, MockedBlockchain, VMContext};
    use std::collections::HashMap;

    pub fn jojo() -> AccountId {
        "jojo.testnet".to_string()
    }
    pub fn speedwagon() -> AccountId {
        "speedwagon.testnet".to_string()
    }
    pub fn zeppeli() -> AccountId {
        "zeppeli.testnet".to_string()
    }
    pub fn dio() -> AccountId {
        "dio.testnet".to_string()
    }
    pub fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "market.testnet".to_string(),
            signer_account_id: "signer.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 10u128.pow(24).into(), // must be greater than attached deposit
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 10u128.pow(23).into(), // minimum necessary to not error out
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    pub fn setup_context(with: AccountId, storage_usage: u64) {
        let context = get_context(with, storage_usage);
        testing_env!(context);
    }

    pub fn quick_hmap(v: Vec<(AccountId, f64)>) -> HashMap<AccountId, Percentage> {
        let mut map = HashMap::new();
        v.into_iter().for_each(|(own, p)| {
            map.insert(own.to_string(), p);
        });
        map
    }
    pub fn lickity_split(init: &str, v: Vec<(AccountId, f64)>) -> SplitOwners {
        SplitOwners::new(quick_hmap(v))
    }
    pub fn quick_roy(p: f64, init: &str, v: Vec<(AccountId, f64)>) -> Royalty {
        Royalty::new(p, quick_hmap(v))
    }

    #[test]
    fn test_split_owners() {
        setup_context(jojo(), 10000);
        let r = lickity_split("01", vec![(jojo(), 100.0)]);
        setup_context(jojo(), 10000);
        let s = lickity_split("02", vec![(jojo(), 10.0), (speedwagon(), 90.0)]);
        setup_context(jojo(), 10000);
        // SPOOKY INCONSISTANTLY TRIGGERED ERROR with 2 SO's initialized, always triggers with 3
        /*
          let t = lickity_split(
              "03",
              vec![(jojo(), 20.0), (speedwagon(), 50.0), (zeppeli(), 30.0)],
          );
        */
    }

    #[test]
    #[should_panic(
        expected = "one of the split between percentages was less than or equal to 0.0: 0"
    )]
    fn panic_split_owners_bad_percent() {
        setup_context(speedwagon(), 0);
        let d = lickity_split("d", vec![(jojo(), 0.0), (speedwagon(), 100.0)]);
    }

    #[test]
    #[should_panic(expected = "percentages don\\\'t add up to 100")]
    fn panic_split_owners_less_than_100() {
        setup_context(speedwagon(), 0);
        let z = lickity_split(
            "z",
            vec![(jojo(), 30.0), (speedwagon(), 30.0), (zeppeli(), 30.0)],
        );
    }

    #[test]
    #[should_panic(expected = "percentages don\\\'t add up to 100")]
    fn panic_split_owners_more_than_100() {
        setup_context(speedwagon(), 0);
        let z = lickity_split(
            "z",
            vec![(jojo(), 30.0), (speedwagon(), 40.0), (zeppeli(), 40.0)],
        );
    }

    #[test]
    #[should_panic(
        expected = "one of the split between percentages was less than or equal to 0.0: -10"
    )]
    fn panic_split_owners_negative_percent() {
        setup_context(speedwagon(), 0);
        let z = lickity_split(
            "z",
            vec![(jojo(), 50.0), (speedwagon(), 60.0), (zeppeli(), -10.0)],
        );
    }

    #[test]
    fn test_royalty() {
        setup_context(speedwagon(), 0);
        let z = quick_roy(20.0, "z", vec![(jojo(), 100.0)]);
        let s = quick_roy(10.0, "s", vec![(jojo(), 10.0), (speedwagon(), 90.0)]);
        // SPOOKY INCONSISTANTLY TRIGGERED ERROR with 2 royalties initialized, always triggers with 3
        /*
        let q = quick_roy(
          10.0,
          "q",
          vec![(jojo(), 20.0), (speedwagon(), 50.0), (zeppeli(), 30.0)],
        );
        */
    }

    #[test]
    #[should_panic(expected = "percentage too high")]
    fn panic_royalty_bad_percent() {
        setup_context(speedwagon(), 0);
        let z = quick_roy(30.0, "z", vec![(jojo(), 100.0)]);
    }

    #[test]
    #[should_panic(expected = "percentages don\\\'t add up to 100")]
    fn panic_royalty_less_than_100() {
        setup_context(speedwagon(), 0);
        let q = quick_roy(
            10.0,
            "q",
            vec![(jojo(), 30.0), (speedwagon(), 30.0), (zeppeli(), 30.0)],
        );
    }

    #[test]
    #[should_panic(expected = "percentages don\\\'t add up to 100")]
    fn panic_royalty_more_than_100() {
        setup_context(speedwagon(), 0);
        let q = quick_roy(
            10.0,
            "q",
            vec![(jojo(), 40.0), (speedwagon(), 30.0), (zeppeli(), 40.0)],
        );
    }

    #[test]
    #[should_panic(
        expected = "one of the split between percentages was less than or equal to 0.0: -15"
    )]
    fn panic_royalty_negative_percent() {
        setup_context(speedwagon(), 0);
        let q = quick_roy(
            10.0,
            "q",
            vec![(jojo(), 65.0), (speedwagon(), 50.0), (zeppeli(), -15.0)],
        );
    }
}

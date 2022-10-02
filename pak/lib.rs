#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod pak {
    use ink_storage::traits::SpreadAllocate;

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        #[ink(topic)]
        value: Balance,
    }

    
    /// Create storage for a simple ERC-20 contract.
    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Pak {
        total_supply: Balance,
        /// The balance of each user.
        balances: ink_storage::Mapping<AccountId, Balance>,
        /// Approval spender on behalf of the message's sender.
        allowances: ink_storage::Mapping<(AccountId, AccountId), Balance>,
    }

    impl Pak {
        /// Create a new ERC-20 contract with an initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            // Initialize mapping for the contract.
            ink_lang::utils::initialize_contract(|contract| {
                Self::new_init(contract, initial_supply)
            })
        }

        /// Initialize the ERC-20 contract with the specified initial supply.
        fn new_init(&mut self, initial_supply: Balance) {
            let caller = Self::env().caller();
            self.balances.insert(&caller, &initial_supply);
            self.total_supply = initial_supply;
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        /// Returns the account balance for the specified `owner`.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }
    }

        #[cfg(test)]
        mod tests {
        use super::*;
    
        use ink_lang as ink;
    
        #[ink::test]
        fn new_works() {
            let contract = Pak::new(777);
            assert_eq!(contract.total_supply(), 777);
        }
    
        #[ink::test]
        fn balance_works() {
            let contract = Pak::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }
    }
}
